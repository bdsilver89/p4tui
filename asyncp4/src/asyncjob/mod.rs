use crate::error::Result;
use crossbeam_channel::Sender;
use std::sync::{Arc, Mutex, RwLock};

pub struct RunParams<T: Copy + Send, P: Clone + Send + Sync> {
    sender: Sender<T>,
    progress: Arc<RwLock<P>>,
}

impl<T: Copy + Send, P: Clone + Send + Sync> RunParams<T, P> {
    pub fn send(&self, notification: T) -> Result<()> {
        self.sender.send(notification)?;
        Ok(())
    }

    pub fn set_progress(&self, p: P) -> Result<()> {
        *(self.progress.write()?) = p;
        Ok(())
    }
}

pub trait AsyncJob: Send + Sync + Clone {
    type Notification: Copy + Send;
    type Progress: Clone + Default + Send + Sync;

    fn run(
        &mut self,
        params: RunParams<Self::Notification, Self::Progress>,
    ) -> Result<Self::Notification>;

    fn get_progress(&self) -> Self::Progress {
        Self::Progress::default()
    }
}

#[derive(Debug, Clone)]
pub struct AsyncSingleJob<J: AsyncJob> {
    next: Arc<Mutex<Option<J>>>,
    last: Arc<Mutex<Option<J>>>,
    progress: Arc<RwLock<J::Progress>>,
    sender: Sender<J::Notification>,
    pending: Arc<Mutex<()>>,
}

impl<J: 'static + AsyncJob> AsyncSingleJob<J> {
    pub fn new(sender: Sender<J::Notification>) -> Self {
        Self {
            next: Arc::new(Mutex::new(None)),
            last: Arc::new(Mutex::new(None)),
            pending: Arc::new(Mutex::new(())),
            progress: Arc::new(RwLock::new(J::Progress::default())),
            sender,
        }
    }

    pub fn is_pending(&self) -> bool {
        self.pending.try_lock().is_err()
    }

    pub fn cancel(&mut self) -> bool {
        if let Ok(mut next) = self.next.lock() {
            if next.is_some() {
                *next = None;
                return true;
            }
        }

        false
    }

    pub fn take_last(&self) -> Option<J> {
        self.last.lock().map_or(None, |mut last| last.take())
    }

    pub fn spawn(&mut self, task: J) -> bool {
        self.schedule_next(task);
        self.check_for_job()
    }

    pub fn progress(&self) -> Option<J::Progress> {
        self.progress.read().ok().map(|d| (*d).clone())
    }

    fn check_for_job(&self) -> bool {
        if self.is_pending() {
            return false;
        }

        if let Some(task) = self.take_next() {
            let self_clone = (*self).clone();
            rayon_core::spawn(move || {
                if let Err(e) = self_clone.run_job(task) {
                    log::error!("async job error: {}", e);
                }
            });

            return true;
        }

        false
    }

    fn run_job(&self, mut task: J) -> Result<()> {
        {
            let _pending = self.pending.lock()?;

            let notification = task.run(RunParams {
                progress: self.progress.clone(),
                sender: self.sender.clone(),
            })?;

            if let Ok(mut last) = self.last.lock() {
                *last = Some(task);
            }

            self.sender.send(notification)?;
        }

        self.check_for_job();

        Ok(())
    }

    fn schedule_next(&mut self, task: J) {
        if let Ok(mut next) = self.next.lock() {
            *next = Some(task);
        }
    }

    fn take_next(&self) -> Option<J> {
        self.next.lock().map_or(None, |mut next| next.take())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crossbeam_channel::unbounded;
    use pretty_assertions::assert_eq;
    use std::{
        sync::atomic::{AtomicBool, AtomicU32, Ordering},
        thread,
        time::Duration,
    };

    #[derive(Clone)]
    struct TestJob {
        v: Arc<AtomicU32>,
        finish: Arc<AtomicBool>,
        value_to_add: u32,
    }

    type TestNotificaton = ();

    impl AsyncJob for TestJob {
        type Notification = TestNotificaton;
        type Progress = ();

        fn run(
            &mut self,
            _params: RunParams<Self::Notification, Self::Progress>,
        ) -> Result<Self::Notification> {
            println!("[job] wait");

            while !self.finish.load(Ordering::SeqCst) {
                std::thread::yield_now();
            }

            println!("[job] sleep");

            thread::sleep(Duration::from_millis(100));

            println!("[job] done sleeping");

            let res = self.v.fetch_add(self.value_to_add, Ordering::SeqCst);

            println!("[job] value: {res}");

            Ok(())
        }
    }

    #[test]
    fn test_overwrite() {
        let (sender, receiver) = unbounded();

        let mut job: AsyncSingleJob<TestJob> = AsyncSingleJob::new(sender);

        let task = TestJob {
            v: Arc::new(AtomicU32::new(1)),
            finish: Arc::new(AtomicBool::new(false)),
            value_to_add: 1,
        };

        assert!(job.spawn(task.clone()));
        task.finish.store(true, Ordering::SeqCst);
        thread::sleep(Duration::from_millis(10));

        for _ in 0..5 {
            println!("spawn");
            assert!(!job.spawn(task.clone()));
        }

        println!("recv");
        receiver.recv().unwrap();
        receiver.recv().unwrap();
        assert!(receiver.is_empty());

        assert_eq!(task.v.load(std::sync::atomic::Ordering::SeqCst), 3);
    }

    fn wait_for_job(job: &AsyncSingleJob<TestJob>) {
        while job.is_pending() {
            thread::sleep(Duration::from_millis(10));
        }
    }

    #[test]
    fn test_cancel() {
        let (sender, receiver) = unbounded();

        let mut job: AsyncSingleJob<TestJob> = AsyncSingleJob::new(sender);

        let task = TestJob {
            v: Arc::new(AtomicU32::new(1)),
            finish: Arc::new(AtomicBool::new(false)),
            value_to_add: 1,
        };

        assert!(job.spawn(task.clone()));
        task.finish.store(true, Ordering::SeqCst);
        thread::sleep(Duration::from_millis(10));

        for _ in 0..5 {
            println!("spawn");
            assert!(!job.spawn(task.clone()));
        }

        println!("cancel");
        assert!(job.cancel());

        task.finish.store(true, Ordering::SeqCst);

        wait_for_job(&job);

        println!("recv");
        receiver.recv().unwrap();
        println!("received");

        assert_eq!(task.v.load(std::sync::atomic::Ordering::SeqCst), 2);
    }
}

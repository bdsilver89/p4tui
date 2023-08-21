mod app;
mod args;
mod commandbar;
mod components;
mod input;
mod keys;
mod notify_mutex;
mod strings;
mod tabbar;
mod tabs;
mod ui;
mod version;

use std::{
    io::{self, Stdout},
    path::PathBuf,
};

use anyhow::{bail, Context, Result};
use app::App;
use args::process_cmdline;
use crossbeam_channel::{Receiver, Select};
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use input::{Input, InputEvent, InputState};
use keys::KeyConfig;
use ratatui::{backend::CrosstermBackend, prelude::Backend, Terminal};
use ui::style::Theme;

#[derive(Clone)]
pub enum QueueEvent {
    Notify,
    InputEvent(InputEvent),
}

fn main() -> Result<()> {
    let cliargs = process_cmdline()?;

    let key_config = KeyConfig::init()
        .map_err(|e| eprintln!("KeyConfig loading error: {e}"))
        .unwrap_or_default();
    let theme = Theme::init(&cliargs.theme);

    let mut terminal = setup_terminal().context("setup terminal failed")?;
    run(cliargs.cwd, &mut terminal, key_config, theme).context("app loop failed")?;
    shutdown_terminal(&mut terminal).context("restore terminal failed")?;
    Ok(())
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>> {
    let mut stdout = io::stdout();
    enable_raw_mode().context("failed to enable raw mode")?;
    execute!(stdout, EnterAlternateScreen).context("unable to enter alternate screen")?;
    Terminal::new(CrosstermBackend::new(stdout)).context("creating terminal failed")
}

fn shutdown_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    disable_raw_mode().context("failed to disable raw mode")?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)
        .context("unable to switch to main screen")?;
    terminal.show_cursor().context("unable to show cursor")
}

fn run(
    cwd: PathBuf,
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    key_config: KeyConfig,
    theme: Theme,
) -> Result<()> {
    let input = Input::new();

    let rx_input = input.receiver();

    let mut first_update = true;
    let mut app = App::new(cwd, input, key_config, theme);
    loop {
        let event = if first_update {
            first_update = false;
            QueueEvent::Notify
        } else {
            select_event(&rx_input)?
        };

        match event {
            QueueEvent::Notify => {
                app.update()?;
            }
            QueueEvent::InputEvent(ev) => {
                if matches!(ev, InputEvent::State(InputState::Polling)) {
                    terminal.hide_cursor()?;
                }
                app.event(ev)?;
            }
        }

        draw(terminal, &app)?;

        if app.is_quit() {
            break;
        }
    }
    Ok(())
}

fn select_event(rx_input: &Receiver<InputEvent>) -> Result<QueueEvent> {
    let mut sel = Select::new();

    sel.recv(rx_input);

    let oper = sel.select();
    let index = oper.index();

    let ev = match index {
        0 => oper.recv(rx_input).map(QueueEvent::InputEvent),
        _ => bail!("unknown select source"),
    }?;

    Ok(ev)
}

fn draw<B: Backend>(terminal: &mut Terminal<B>, app: &App) -> io::Result<()> {
    terminal.draw(|f| {
        if let Err(e) = app.draw(f) {
            log::error!("failed to draw: {:?}", e);
        }
    })?;
    Ok(())
}

// fn should_quit() -> Result<bool> {
//     if crossterm::event::poll(Duration::from_millis(250)).context("event poll failed")? {
//         if let crossterm::event::Event::Key(key) =
//             crossterm::event::read().context("event read failed")?
//         {
//             return Ok(crossterm::event::KeyCode::Char('q') == key.code);
//         }
//     }
//     Ok(false)
// }

mod changelist;
mod command;
mod diff;
mod filelist;
mod help;
mod utils;

pub use command::{CommandInfo, CommandText};
pub use diff::DiffComponent;
pub use help::HelpComponent;

use anyhow::Result;
use crossterm::event::Event;
use ratatui::{backend::Backend, layout::Rect, Frame};

#[macro_export]
macro_rules! accessors {
    ($self:ident, [$($element:ident),+]) => {
        fn components(& $self) -> Vec<&dyn Component> {
            vec![$(&$self.$element,)+]
        }

        fn components_mut(&mut $self) -> Vec<&mut dyn Component> {
            vec![$(&mut $self.$element,)+]
        }
    };
}

#[macro_export]
macro_rules! any_popup_visible {
    ($self:ident, [$($element:ident),+]) => {
        fn any_popup_visible(& $self) -> bool{
            ($($self.$element.is_visible()) || +)
        }
    };
}

#[macro_export]
macro_rules! draw_popups {
    ($self:ident, [$($element:ident),+]) => {
        fn draw_popups<B: Backend>(& $self, mut f: &mut Frame<B>) -> Result<()>{
            let size = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Min(1),
                    // Constraint::Length($self.cmdbar.borrow().height()),
                ]
                .as_ref(),
            )
            .split(f.size())[0];

            ($($self.$element.draw(&mut f, size)?) , +);

            return Ok(());
        }
    };
}

#[macro_export]
macro_rules! setup_popups {
    ($self:ident, [$($element:ident),+]) => {
        $crate::any_popup_visible!($self, [$($element),+]);
        $crate::draw_popups!($self, [$($element),+]);
    };
}

pub fn event_pump(ev: &Event, components: &mut [&mut dyn Component]) -> Result<EventState> {
    for c in components {
        if c.event(ev)?.is_consumed() {
            return Ok(EventState::Consumed);
        }
    }

    Ok(EventState::NotConsumed)
}

#[derive(Copy, Clone)]
pub enum ScrollType {
    Up,
    Down,
    Home,
    End,
    PageUp,
    PageDown,
}

#[derive(Copy, Clone)]
pub enum HorizontalScrollType {
    Left,
    Right,
}

#[derive(PartialEq, Eq)]
pub enum CommandBlocking {
    Blocking,
    PassingOn,
}

pub fn visibility_blocking<T: Component>(comp: &T) -> CommandBlocking {
    if comp.is_visible() {
        CommandBlocking::Blocking
    } else {
        CommandBlocking::PassingOn
    }
}

pub trait DrawableComponent {
    fn draw<B: Backend>(&self, f: &mut Frame<B>, rect: Rect) -> Result<()>;
}

#[derive(PartialEq, Eq)]
pub enum EventState {
    Consumed,
    NotConsumed,
}

impl EventState {
    pub fn is_consumed(&self) -> bool {
        *self == Self::Consumed
    }
}

impl From<bool> for EventState {
    fn from(value: bool) -> Self {
        if value {
            Self::Consumed
        } else {
            Self::NotConsumed
        }
    }
}

pub trait Component {
    fn commands(&self, out: &mut Vec<CommandInfo>, force_all: bool) -> CommandBlocking;

    fn event(&mut self, ev: &Event) -> Result<EventState>;

    fn focused(&self) -> bool {
        false
    }

    fn focus(&mut self, _focus: bool) {}

    fn is_visible(&self) -> bool {
        true
    }

    fn hide(&mut self) {}

    fn show(&mut self) -> Result<()> {
        Ok(())
    }

    fn toggle_visible(&mut self) -> Result<()> {
        if self.is_visible() {
            self.hide();
            Ok(())
        } else {
            self.show()
        }
    }
}

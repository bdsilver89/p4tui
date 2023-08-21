use anyhow::Result;
use crossterm::event::Event;

use crate::{keys::SharedKeyConfig, ui::style::SharedTheme};

use super::{
    utils::scroll_vertical::VerticalScroll, visibility_blocking, CommandBlocking, CommandInfo,
    Component, DrawableComponent, EventState,
};

use ratatui::{
    backend::Backend,
    layout::Rect,
    text::Span,
    widgets::{Block, Borders},
    Frame,
};

pub struct DiffComponent {
    focused: bool,
    vertical_scroll: VerticalScroll,
    // horizontal_scroll
    key_config: SharedKeyConfig,
    theme: SharedTheme,
}

impl DiffComponent {
    pub fn new(key_config: SharedKeyConfig, theme: SharedTheme) -> Self {
        Self {
            focused: false,
            vertical_scroll: VerticalScroll::new(),
            key_config,
            theme,
        }
    }

    fn can_scroll(&self) -> bool {
        // TODO: fix!
        false
    }

    pub fn clear(&mut self, pending: bool) {
        self.vertical_scroll.reset();
        // self.horizontal_scroll.reset();
    }

    pub fn update(&mut self) {}
}

impl DrawableComponent for DiffComponent {
    fn draw<B: Backend>(&self, f: &mut Frame<B>, r: Rect) -> Result<()> {
        // self.vertical_scroll.update()
        // self.horizontal_scroll.update()

        let diff = Block::default()
            .title(Span::styled("Diff", self.theme.title(self.focused)))
            .borders(Borders::ALL)
            .border_style(self.theme.block(self.focused));

        f.render_widget(diff, r);
        Ok(())
    }
}

impl Component for DiffComponent {
    fn commands(&self, out: &mut Vec<CommandInfo>, force_all: bool) -> CommandBlocking {
        CommandBlocking::PassingOn
    }

    fn event(&mut self, ev: &Event) -> Result<EventState> {
        Ok(EventState::NotConsumed)
    }

    fn focused(&self) -> bool {
        self.focused
    }
    fn focus(&mut self, focus: bool) {
        self.focused = focus;
    }
}

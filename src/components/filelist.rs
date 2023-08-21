use anyhow::Result;
use crossterm::event::Event;
use ratatui::{
    backend::Backend,
    layout::Rect,
    text::Span,
    widgets::{Block, Borders},
    Frame,
};

use crate::{keys::SharedKeyConfig, ui::style::SharedTheme};

use std::path::PathBuf;

use super::{CommandBlocking, CommandInfo, Component, DrawableComponent, EventState};

pub struct FileListComponent {
    focused: bool,
    title: String,
    changelist: Option<u32>,
    files: Vec<PathBuf>,
    key_config: SharedKeyConfig,
    theme: SharedTheme,
}

impl FileListComponent {
    pub fn new(title: String, key_config: SharedKeyConfig, theme: SharedTheme) -> Self {
        Self {
            focused: false,
            title,
            changelist: Option::None,
            files: Vec::new(),
            key_config,
            theme,
        }
    }
}

impl DrawableComponent for FileListComponent {
    fn draw<B: Backend>(&self, f: &mut Frame<B>, r: Rect) -> Result<()> {
        let b = Block::default()
            .title(Span::styled(
                self.title.as_str(),
                self.theme.title(self.focused),
            ))
            .borders(Borders::ALL)
            .border_style(self.theme.block(self.focused));

        f.render_widget(b, r);
        Ok(())
    }
}

impl Component for FileListComponent {
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
        self.focused = focus
    }
}

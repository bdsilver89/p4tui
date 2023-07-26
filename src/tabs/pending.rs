use crate::{
    components::{
        visibility_blocking, CommandBlocking, CommandInfo, Component, DiffComponent,
        DrawableComponent, EventState,
    },
    keys::SharedKeyConfig,
    ui::style::SharedTheme,
};

use anyhow::Result;
use crossterm::event::Event;
use ratatui::{
    backend::Backend,
    layout::{Direction, Layout, Rect},
    prelude::Constraint,
    text::Span,
    widgets::{Block, Borders},
    Frame,
};

pub struct PendingTab {
    visible: bool,
    diff: DiffComponent,
    key_config: SharedKeyConfig,
    theme: SharedTheme,
}

impl DrawableComponent for PendingTab {
    fn draw<B: Backend>(&self, f: &mut Frame<B>, rect: Rect) -> Result<()> {
        let rects = std::rc::Rc::new([rect]);

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(rects[0]);

        let left_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage(34),
                    Constraint::Percentage(33),
                    Constraint::Percentage(33),
                ]
                .as_ref(),
            )
            .split(chunks[0]);

        // TODO: replace with components!

        let cl = Block::default()
            .title(Span::raw("Changelist"))
            .borders(Borders::ALL);

        f.render_widget(cl, left_chunks[0]);

        let unshelved = Block::default()
            .title(Span::raw("Unshelved Files"))
            .borders(Borders::ALL);

        f.render_widget(unshelved, left_chunks[1]);

        let shelved = Block::default()
            .title(Span::raw("Shelved Files"))
            .borders(Borders::ALL);

        f.render_widget(shelved, left_chunks[2]);

        self.diff.draw(f, chunks[1])?;

        Ok(())
    }
}

impl PendingTab {
    pub fn new(key_config: SharedKeyConfig, theme: SharedTheme) -> Self {
        Self {
            visible: true,
            diff: DiffComponent::new(key_config.clone(), theme.clone()),
            key_config,
            theme,
        }
    }

    pub fn update(&mut self) -> Result<()> {
        Ok(())
    }
}

impl Component for PendingTab {
    fn commands(&self, out: &mut Vec<CommandInfo>, force_all: bool) -> CommandBlocking {
        visibility_blocking(self)
    }

    fn event(&mut self, ev: &Event) -> Result<EventState> {
        Ok(EventState::NotConsumed)
    }

    fn is_visible(&self) -> bool {
        self.visible
    }

    fn hide(&mut self) {
        self.visible = false;
    }

    fn show(&mut self) -> Result<()> {
        self.visible = true;

        self.update()?;

        Ok(())
    }
}

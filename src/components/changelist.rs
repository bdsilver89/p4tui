use std::cell::Cell;

use anyhow::Result;
use asyncp4::sync;
use crossterm::event::{Event, KeyEvent};

use crate::{
    keys::{key_match, SharedKeyConfig},
    ui::style::SharedTheme,
};

use ratatui::{
    backend::Backend,
    layout::Rect,
    text::Span,
    widgets::{Block, Borders},
    Frame,
};

use super::{CommandBlocking, CommandInfo, Component, DrawableComponent, EventState, ScrollType};

pub struct ChangelistComponent {
    focused: bool,
    status: sync::ChangelistStatus,
    changelists: Vec<u32>,
    selection: u16,
    current_height: Cell<u16>,
    key_config: SharedKeyConfig,
    theme: SharedTheme,
}

impl ChangelistComponent {
    pub fn new(
        status: sync::ChangelistStatus,
        key_config: SharedKeyConfig,
        theme: SharedTheme,
    ) -> Self {
        Self {
            status,
            focused: false,
            key_config,
            theme,
            changelists: Vec::new(),
            selection: 0,
            current_height: Cell::new(0),
        }
    }

    pub fn update(&mut self) -> Result<()> {
        Ok(())
    }

    pub fn get_changelists(&self) -> &Vec<u32> {
        &self.changelists
    }

    pub fn get_selected(&self) -> Option<u32> {
        self.changelists
            .get(self.selection as usize)
            .map(|c| c.clone())
    }

    fn move_event(&mut self, e: &KeyEvent) -> Result<EventState> {
        if key_match(e, self.key_config.keys.move_down) {
            return self.move_selection(ScrollType::Down).map(Into::into);
        } else if key_match(e, self.key_config.keys.move_up) {
            return self.move_selection(ScrollType::Up).map(Into::into);
        }

        Ok(EventState::NotConsumed)
    }

    pub fn move_selection(&mut self, scroll: ScrollType) -> Result<bool> {
        let new_selection = match scroll {
            ScrollType::Up => self.selection.saturating_add(1),
            ScrollType::Down => self.selection.saturating_sub(1),
            ScrollType::PageDown => self.selection.saturating_add(self.current_height.get()),
            ScrollType::PageUp => self.selection.saturating_add(self.current_height.get()),
            ScrollType::Home => 0,
            ScrollType::End => self.changelists.len().try_into()?,
        };

        self.set_selection(new_selection)?;

        Ok(true)
    }

    fn set_selection(&mut self, selection: u16) -> Result<()> {
        let num_changelists: u16 = self.changelists.len().try_into()?;
        let num_changelists = num_changelists.saturating_sub(1);

        let selection = if selection > num_changelists {
            num_changelists
        } else {
            selection
        };

        self.selection = selection;
        Ok(())
    }
}

impl DrawableComponent for ChangelistComponent {
    fn draw<B: Backend>(&self, f: &mut Frame<B>, r: Rect) -> Result<()> {
        let b = Block::default()
            .title(Span::styled("Changelists", self.theme.title(self.focused)))
            .borders(Borders::ALL)
            .border_style(self.theme.block(self.focused));

        f.render_widget(b, r);
        Ok(())
    }
}

impl Component for ChangelistComponent {
    fn commands(&self, out: &mut Vec<CommandInfo>, force_all: bool) -> CommandBlocking {
        CommandBlocking::PassingOn
    }

    fn event(&mut self, ev: &Event) -> Result<EventState> {
        if let Event::Key(e) = ev {
            if self.move_event(e)?.is_consumed() {
                return Ok(EventState::Consumed);
            }
        }
        Ok(EventState::NotConsumed)
    }

    fn focused(&self) -> bool {
        self.focused
    }

    fn focus(&mut self, focus: bool) {
        self.focused = focus;
    }
}

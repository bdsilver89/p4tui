use std::{borrow::Cow, path::PathBuf};

use crate::{keys::SharedKeyConfig, strings, ui::style::SharedTheme};

use unicode_width::UnicodeWidthStr;

use ratatui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Margin, Rect},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Tabs},
    Frame,
};

pub struct TabBar {
    cwd: PathBuf,
    selected_tab: usize,
    theme: SharedTheme,
    key_config: SharedKeyConfig,
}

impl TabBar {
    pub const fn new(key_config: SharedKeyConfig, theme: SharedTheme, cwd: PathBuf) -> Self {
        Self {
            cwd,
            key_config,
            theme,
            selected_tab: 0,
        }
    }

    pub fn set_selected_tab(&mut self, tab: usize) {
        self.selected_tab = tab;
    }

    pub fn draw<B: Backend>(&self, f: &mut Frame<B>, r: Rect) {
        const DIVIDER_PAD_SPACES: usize = 2;
        const SIDE_PADS: usize = 2;
        const MARGIN_LEFT_RIGHT: usize = 2;

        let r = r.inner(&Margin {
            vertical: 0,
            horizontal: 1,
        });

        let tab_labels = [
            Span::raw(strings::tab_pending(&self.key_config)),
            Span::raw(strings::tab_submitted(&self.key_config)),
        ];
        let divider = strings::tab_divider();

        let tabs_len: usize = tab_labels.iter().map(Span::width).sum::<usize>()
            + tab_labels.len().saturating_sub(1) * (divider.width() + DIVIDER_PAD_SPACES)
            + SIDE_PADS
            + MARGIN_LEFT_RIGHT;

        let left_right = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Length(u16::try_from(tabs_len).unwrap_or(r.width)),
                Constraint::Min(0),
            ])
            .split(r);

        let table_area = r;
        let text_area = left_right[1];

        let tabs = tab_labels.into_iter().map(Line::from).collect();

        f.render_widget(
            Tabs::new(tabs)
                .block(
                    Block::default()
                        .borders(Borders::BOTTOM)
                        .border_style(self.theme.block(false)),
                )
                .style(self.theme.tab(false))
                .highlight_style(self.theme.tab(true))
                .divider(divider)
                .select(self.selected_tab),
            table_area,
        );

        f.render_widget(
            Paragraph::new(Line::from(vec![Span::styled(
                strings::ellipsis_trim_start("this/is/a/tmp/path", text_area.width as usize),
                self.theme.title(true),
            )]))
            .alignment(Alignment::Right),
            text_area,
        );
    }
}

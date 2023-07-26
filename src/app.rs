use std::{borrow::BorrowMut, cell::RefCell, os::raw::c_ushort, path::PathBuf, rc::Rc};

use crate::{
    accessors,
    commandbar::CommandBar,
    components::{event_pump, Component, DrawableComponent, HelpComponent},
    input::{Input, InputEvent},
    keys::{key_match, KeyConfig, SharedKeyConfig},
    setup_popups,
    tabbar::TabBar,
    tabs::{PendingTab, SubmittedTab},
    ui::style::{SharedTheme, Theme},
};
use anyhow::{bail, Result};
use crossterm::event::{Event, KeyEvent};
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    Frame,
};

#[derive(Clone)]
pub enum QuitState {
    None,
    Close,
}

pub struct App {
    cwd: PathBuf,
    do_quit: QuitState,
    key_config: SharedKeyConfig,
    theme: SharedTheme,
    input: Input,
    tabbar: RefCell<TabBar>,
    cmdbar: RefCell<CommandBar>,
    tab: usize,
    pending_tab: PendingTab,
    submitted_tab: SubmittedTab,
    help: HelpComponent,
}

impl App {
    pub fn new(cwd: PathBuf, input: Input, key_config: KeyConfig, theme: Theme) -> Self {
        let key_config = Rc::new(key_config);
        let theme = Rc::new(theme);
        Self {
            do_quit: QuitState::None,
            tab: 0,
            tabbar: RefCell::new(TabBar::new(key_config.clone(), theme.clone(), cwd.clone())),
            cmdbar: RefCell::new(CommandBar::new(key_config.clone(), theme.clone())),
            pending_tab: PendingTab::new(key_config.clone(), theme.clone()),
            submitted_tab: SubmittedTab::new(key_config.clone(), theme.clone()),
            help: HelpComponent::new(key_config.clone(), theme.clone()),
            cwd,
            input,
            key_config,
            theme,
        }
    }
    pub fn draw<B: Backend>(&self, f: &mut Frame<B>) -> Result<()> {
        let fsize = f.size();
        // let greeting = ratatui::widgets::Paragraph::new("Hello World! (press 'q' to quit)");
        // f.render_widget(greeting, f.size());

        // self.cmdbar.borrow_mut().refresh_width(fsize.width);

        let chunks_main = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(2),
                    Constraint::Min(2),
                    Constraint::Length(self.cmdbar.borrow().height()),
                ]
                .as_ref(),
            )
            .split(fsize);

        self.cmdbar.borrow().draw(f, chunks_main[2]);

        self.tabbar.borrow().draw(f, chunks_main[0]);

        match self.tab {
            0 => self.pending_tab.draw(f, chunks_main[1])?,
            1 => self.submitted_tab.draw(f, chunks_main[1])?,
            _ => bail!("unknown tab"),
        }

        self.draw_popups(f)?;

        Ok(())
    }

    pub fn update(&mut self) -> Result<()> {
        self.pending_tab.update()?;

        self.update_commands();

        Ok(())
    }

    pub fn event(&mut self, ev: InputEvent) -> Result<()> {
        if let InputEvent::Input(ev) = ev {
            if self.check_hard_exit(&ev) || self.check_quit(&ev) {
                return Ok(());
            }

            if event_pump(&ev, self.components_mut().as_mut_slice())?.is_consumed() {
            } else if let Event::Key(k) = &ev {
                if key_match(k, self.key_config.keys.tab_pending)
                    || key_match(k, self.key_config.keys.tab_submitted)
                {
                    self.switch_tab(k)?;
                }
            }
        }

        Ok(())
    }

    pub fn is_quit(&self) -> bool {
        !matches!(self.do_quit, QuitState::None) || self.input.is_aborted()
    }
}

impl App {
    accessors!(self, [help]);

    setup_popups!(self, [help]);

    fn check_quit(&mut self, ev: &Event) -> bool {
        if let Event::Key(e) = ev {
            if key_match(e, self.key_config.keys.quit) {
                self.do_quit = QuitState::Close;
                return true;
            }
        }
        false
    }

    fn check_hard_exit(&mut self, ev: &Event) -> bool {
        if let Event::Key(e) = ev {
            if key_match(e, self.key_config.keys.exit) {
                self.do_quit = QuitState::Close;
                return true;
            }
        }
        false
    }

    fn get_tabs(&mut self) -> Vec<&mut dyn Component> {
        vec![&mut self.pending_tab, &mut self.submitted_tab]
    }

    fn switch_tab(&mut self, k: &KeyEvent) -> Result<()> {
        if key_match(k, self.key_config.keys.tab_pending) {
            self.set_tab(0)?;
        } else if key_match(k, self.key_config.keys.tab_submitted) {
            self.set_tab(1)?;
        }

        Ok(())
    }

    fn set_tab(&mut self, tab: usize) -> Result<()> {
        let tabs = self.get_tabs();
        for (i, t) in tabs.into_iter().enumerate() {
            if tab == i {
                t.show()?;
            } else {
                t.hide();
            }
        }

        self.tab = tab;
        self.tabbar.borrow_mut().set_selected_tab(tab);

        Ok(())
    }

    fn update_commands(&mut self) {
        // if self.help.is_visible() {
        //     self.help.set_cmds(self.commands(true));
        // }
    }
}

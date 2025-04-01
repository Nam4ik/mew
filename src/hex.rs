use std::{fs, io};
use std::io::stdout;
use ratatui::{backend::CrosstermBackend, Terminal};
use ratatui::widgets::{ScrollbarState, ScrollDirection};
use crossterm::{execute, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}};
use crate::{input, render};

pub struct HexEditor {
    filename: String,
    data: Vec<u8>,
    cursor: usize,
    terminal: Terminal<CrosstermBackend<std::io::Stdout>,>,
    scrollbar_state: ScrollbarState,
    scroll_position: usize
}

impl HexEditor {

    pub fn new(filename: String, data: Vec<u8>) -> io::Result<Self> {
        enable_raw_mode()?;
        execute!(stdout(), EnterAlternateScreen)?;
        let terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
        let line_count = (data.len() + 15) / 16;
        let scrollbar_state = ScrollbarState::default()
            .content_length(line_count)
            .viewport_content_length(10);

            Ok(Self {
                filename,
                data,
                cursor: 0,
                terminal,
                scrollbar_state,
                scroll_position: 0,
            })
    }


    pub fn run(&mut self) -> io::Result<()> {
        while input::handle_input(
            &mut self.cursor,
            &mut self.data,
            &mut self.scroll_position
        )? {
            let total_lines = (self.data.len() + 15) / 16;
            let visible_lines = self.terminal.size()?.height as usize;


            self.scrollbar_state = ScrollbarState::new(total_lines)
                .viewport_content_length(visible_lines)
                .position(self.scroll_position);


            render::draw(
                &mut self.terminal,
                &self.data,
                self.cursor,
                &self.filename,
                self.scroll_position,
                &mut self.scrollbar_state
            )?;
        }
        self.cleanup()
    }

    pub fn cleanup(&mut self) -> io::Result<()> {
        disable_raw_mode()?;
        execute!(stdout(), LeaveAlternateScreen)?;
        fs::write(&self.filename, &self.data)?;
        Ok(())
    }

}

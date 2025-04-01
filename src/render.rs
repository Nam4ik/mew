use std::io;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{
        Block, Borders, Paragraph, ScrollbarState, Scrollbar, ScrollbarOrientation,
    },
    Terminal,
    prelude::Alignment,
    symbols,
};

pub fn draw(
    terminal: &mut Terminal<impl ratatui::backend::Backend>,
    data: &[u8],
    cursor: usize,
    filename: &str,
    scroll_position: usize,
    scrollbar_state: &mut ScrollbarState,
) -> io::Result<()> {
    terminal.draw(|frame| {
        let size = frame.size();
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Length(1)])
            .split(size);


        let total_lines = (data.len() + 15) / 16;
        let visible_height = layout[0].height as usize;

        *scrollbar_state = ScrollbarState::new(total_lines)
            .viewport_content_length(visible_height)
            .position(scroll_position);

        let lines: Vec<Line> = data
            .chunks(16)
            .enumerate()
            .skip(scroll_position)
            .take(visible_height)
            .map(|(i, chunk)| {
                let addr = format!("{:08X} │ ", i * 16);
                let mut hex_spans = Vec::new();
                let mut ascii = String::new();

                for (j, &byte) in chunk.iter().enumerate() {
                    let global_pos = i * 16 + j;
                    let nibbles = format!("{:02X}", byte);

                    for (k, c) in nibbles.chars().enumerate() {
                        let is_cursor = (global_pos * 2 + k) == cursor;
                        let style = if is_cursor {
                            Style::default().bg(Color::White).fg(Color::Black)
                        } else {
                            Style::default()
                        };
                        hex_spans.push(Span::styled(c.to_string(), style));
                    }


                    if j < 15 {
                        hex_spans.push(Span::raw(if j % 4 == 3 { "   " } else { " " }));
                    }


                    ascii.push(if byte.is_ascii_graphic() || byte == b' ' {
                        byte as char
                    } else {
                        '.'
                    });
                }


                if chunk.len() < 16 {
                    let missing = 16 - chunk.len();
                    hex_spans.push(Span::raw(" ".repeat(missing * 3)));
                    ascii.push_str(&".".repeat(missing));
                }

                Line::from(vec![
                    Span::raw(addr),
                    Span::raw(hex_spans.iter().map(|s| s.content.as_ref()).collect::<String>()),
                    Span::raw(" │ "),
                    Span::raw(ascii),
                ])
            })
            .collect();


        frame.render_widget(
            Paragraph::new(lines)
                .block(Block::default().borders(Borders::ALL)),
            layout[0]
        );


        frame.render_stateful_widget(
            Scrollbar::default()
                .orientation(ScrollbarOrientation::VerticalRight)
                .symbols(symbols::scrollbar::VERTICAL),
            layout[0],
            scrollbar_state,
        );


        let progress = if data.is_empty() {
            0
        } else {
            (cursor / 2 * 100) / data.len()
        };

        let status = format!(
            "{} │ Offset: 0x{:08X} │ {}%",
            filename,
            cursor / 2,
            progress
        );

        frame.render_widget(
            Paragraph::new(status)
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::TOP)),
            layout[1]
        );
    })?;
    Ok(())
}

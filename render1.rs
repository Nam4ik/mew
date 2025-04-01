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
    scrollbar_state: &mut ScrollbarState,
) -> io::Result<()> {
    terminal.draw(|frame| {
        let size = frame.size();
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Length(1)])
            .split(size);

        // Исправление: Устанавливаем длину содержимого скролла
        let content_length = data.len() * 2; // 2 символа на байт
        scrollbar_state.content_length(content_length);
        
        let visible_lines = layout[0].height as usize;
        scrollbar_state.viewport_content_length(visible_lines);

        let lines: Vec<Line> = data.chunks(16).enumerate().map(|(i, chunk)| {
            let addr = Span::raw(format!("{:08x} │ ", i * 16));
            let mut hex: Vec<Span> = Vec::new();
            let mut ascii_repr = String::new();

            for (j, &byte) in chunk.iter().enumerate() {
                let hex_str = format!("{:02x}", byte);
                let byte_index = i * 16 + j; // Индекс байта в данных
                let cursor_byte = cursor / 2; // Байт, на котором находится курсор

                for (k, ch) in hex_str.chars().enumerate() {
                    let cur_idx = byte_index * 2 + k; // Индекс символа в hex-строке
                    let style = if cur_idx == cursor {
                        Style::default().bg(Color::White).fg(Color::Black)
                    } else {
                        Style::default()
                    };
                    hex.push(Span::styled(ch.to_string(), style));
                }

                if j < 15 {
                    let space = if j % 4 == 3 { "   " } else { " " };
                    hex.push(Span::raw(space));
                }

                ascii_repr.push(if byte.is_ascii_graphic() || byte == b' ' { 
                    byte as char 
                } else { 
                    '.' 
                });
            }

            let missing_bytes = 16 - chunk.len();
            let padding = missing_bytes * 3;
            let spaces = " ".repeat(padding);

            let ascii = Span::raw(format!("{} │ {}", spaces, ascii_repr));

            Line::from(
                vec![addr]
                    .into_iter()
                    .chain(hex)
                    .chain(std::iter::once(ascii))
                    .collect::<Vec<_>>()
            )
        }).collect();

        frame.render_widget(
            Paragraph::new(lines).block(Block::default().borders(Borders::ALL)),
            layout[0]
        );
        
        frame.render_stateful_widget(
            Scrollbar::default()
                .orientation(ScrollbarOrientation::VerticalRight)
                .symbols(symbols::scrollbar::VERTICAL),
            layout[0],
            scrollbar_state,
        );

        // Исправление: Корректное вычисление прогресса
        let progress = if data.is_empty() {
            100
        } else {
            let cursor_byte = cursor / 2;
            (cursor_byte * 100).saturating_div(data.len())
        };
        
        let status_text = format!(
            "{} \t 0x{:08x} \t {:3}%", 
            filename, 
            cursor / 2, 
            progress
        );
        
        let status_bar = Paragraph::new(
            Line::from(Span::raw(status_text))
        ).alignment(Alignment::Center);
        
        frame.render_widget(status_bar, layout[1]);
    })?;
    Ok(())
}
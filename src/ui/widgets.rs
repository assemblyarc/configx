use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
};

pub const CLR_BG: Color = Color::Rgb(15, 15, 23);
pub const CLR_SURFACE: Color = Color::Rgb(24, 24, 37);
pub const CLR_BORDER: Color = Color::Rgb(88, 91, 112);
pub const CLR_ACCENT: Color = Color::Rgb(137, 180, 250);
pub const CLR_GREEN: Color = Color::Rgb(166, 227, 161);
pub const CLR_YELLOW: Color = Color::Rgb(249, 226, 175);
pub const CLR_MUTED: Color = Color::Rgb(108, 112, 134);
pub const CLR_TEXT: Color = Color::Rgb(205, 214, 244);
pub const CLR_SUBTEXT: Color = Color::Rgb(166, 173, 200);

pub fn render_chrome(f: &mut Frame, title: &str, hint: &str, step_info: &str) -> Rect {
    let area = f.area();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(area);

    let header_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(CLR_ACCENT))
        .style(Style::default().bg(CLR_BG));

    let logo = Line::from(vec![
        Span::styled(
            " ⚙  configx ",
            Style::default().fg(CLR_ACCENT).add_modifier(Modifier::BOLD),
        ),
        Span::styled("· ", Style::default().fg(CLR_MUTED)),
        Span::styled("Neovim Config Wizard", Style::default().fg(CLR_SUBTEXT)),
        Span::styled(format!("  {step_info}"), Style::default().fg(CLR_MUTED)),
    ]);

    let title_line = Line::from(vec![Span::styled(
        format!(" {title} "),
        Style::default().fg(CLR_TEXT).add_modifier(Modifier::BOLD),
    )]);

    let header = Paragraph::new(vec![logo, title_line])
        .block(header_block)
        .wrap(Wrap { trim: false });

    f.render_widget(header, chunks[0]);

    let footer_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(CLR_BORDER))
        .style(Style::default().bg(CLR_BG));

    let footer = Paragraph::new(Line::from(vec![Span::styled(
        format!(" {hint} "),
        Style::default().fg(CLR_MUTED),
    )]))
    .block(footer_block);

    f.render_widget(footer, chunks[2]);

    chunks[1]
}

pub fn render_multiselect(
    f: &mut Frame,
    area: Rect,
    title: &str,
    items: &[String],
    selected: &[usize],
    cursor: usize,
) {
    let block = Block::default()
        .title(Span::styled(
            format!(" {title} "),
            Style::default().fg(CLR_ACCENT).add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(CLR_BORDER))
        .style(Style::default().bg(CLR_SURFACE));

    let lines: Vec<Line> = items
        .iter()
        .enumerate()
        .map(|(i, label)| {
            let is_selected = selected.contains(&i);
            let is_cursor = i == cursor;

            let checkbox = if is_selected { " ● " } else { " ○ " };
            let checkbox_color = if is_selected { CLR_GREEN } else { CLR_MUTED };

            let label_style = if is_cursor {
                Style::default()
                    .fg(CLR_TEXT)
                    .bg(Color::Rgb(40, 40, 60))
                    .add_modifier(Modifier::BOLD)
            } else if is_selected {
                Style::default().fg(CLR_TEXT)
            } else {
                Style::default().fg(CLR_SUBTEXT)
            };

            let cursor_indicator = if is_cursor { "▶ " } else { "  " };

            Line::from(vec![
                Span::styled(cursor_indicator, Style::default().fg(CLR_ACCENT)),
                Span::styled(checkbox, Style::default().fg(checkbox_color)),
                Span::styled(label.as_str(), label_style),
            ])
        })
        .collect();

    let paragraph = Paragraph::new(lines)
        .block(block)
        .wrap(Wrap { trim: false });

    f.render_widget(paragraph, area);
}

pub fn render_radio<T: std::fmt::Display>(
    f: &mut Frame,
    area: Rect,
    title: &str,
    items: &[T],
    selected_index: usize,
    cursor: usize,
) {
    let block = Block::default()
        .title(Span::styled(
            format!(" {title} "),
            Style::default().fg(CLR_ACCENT).add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(CLR_BORDER))
        .style(Style::default().bg(CLR_SURFACE));

    let lines: Vec<Line> = items
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let is_selected = i == selected_index;
            let is_cursor = i == cursor;

            let radio = if is_selected { " ◉ " } else { " ○ " };
            let radio_color = if is_selected { CLR_ACCENT } else { CLR_MUTED };

            let label_style = if is_cursor {
                Style::default()
                    .fg(CLR_TEXT)
                    .bg(Color::Rgb(40, 40, 60))
                    .add_modifier(Modifier::BOLD)
            } else if is_selected {
                Style::default().fg(CLR_TEXT)
            } else {
                Style::default().fg(CLR_SUBTEXT)
            };

            let cursor_indicator = if is_cursor { "▶ " } else { "  " };

            Line::from(vec![
                Span::styled(cursor_indicator, Style::default().fg(CLR_ACCENT)),
                Span::styled(radio, Style::default().fg(radio_color)),
                Span::styled(item.to_string(), label_style),
            ])
        })
        .collect();

    let paragraph = Paragraph::new(lines)
        .block(block)
        .wrap(Wrap { trim: false });

    f.render_widget(paragraph, area);
}

pub fn render_colorscheme_picker(
    f: &mut Frame,
    area: Rect,
    schemes: &[crate::registry::colorschemes::Colorscheme],
    selected_index: usize,
    cursor: usize,
) {
    let block = Block::default()
        .title(Span::styled(
            " Colorscheme ",
            Style::default().fg(CLR_ACCENT).add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(CLR_BORDER))
        .style(Style::default().bg(CLR_SURFACE));

    let lines: Vec<Line> = schemes
        .iter()
        .enumerate()
        .map(|(i, scheme)| {
            let is_selected = i == selected_index;
            let is_cursor = i == cursor;

            let radio = if is_selected { " ◉ " } else { " ○ " };
            let radio_color = if is_selected { CLR_ACCENT } else { CLR_MUTED };
            let cursor_indicator = if is_cursor { "▶ " } else { "  " };

            let name_style = if is_cursor {
                Style::default()
                    .fg(CLR_TEXT)
                    .bg(Color::Rgb(40, 40, 60))
                    .add_modifier(Modifier::BOLD)
            } else if is_selected {
                Style::default().fg(CLR_TEXT)
            } else {
                Style::default().fg(CLR_SUBTEXT)
            };

            let mut spans = vec![
                Span::styled(cursor_indicator, Style::default().fg(CLR_ACCENT)),
                Span::styled(radio, Style::default().fg(radio_color)),
                Span::styled(format!("{:<20}", scheme.name), name_style),
                Span::raw("  "),
            ];

            for &(r, g, b) in scheme.swatches {
                spans.push(Span::styled("██", Style::default().fg(Color::Rgb(r, g, b))));
            }

            spans.push(Span::raw("  "));
            spans.push(Span::styled(
                scheme.description,
                Style::default().fg(CLR_MUTED),
            ));

            Line::from(spans)
        })
        .collect();

    let paragraph = Paragraph::new(lines)
        .block(block)
        .wrap(Wrap { trim: false });

    f.render_widget(paragraph, area);
}

pub fn render_centred_text(f: &mut Frame, area: Rect, lines: Vec<Line>) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(CLR_ACCENT))
        .style(Style::default().bg(CLR_SURFACE));

    let paragraph = Paragraph::new(lines)
        .block(block)
        .wrap(Wrap { trim: false });

    f.render_widget(paragraph, area);
}

pub fn render_summary(f: &mut Frame, area: Rect, rows: Vec<(&str, String)>) {
    let block = Block::default()
        .title(Span::styled(
            " Configuration Summary ",
            Style::default().fg(CLR_ACCENT).add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(CLR_BORDER))
        .style(Style::default().bg(CLR_SURFACE));

    let lines: Vec<Line> = rows
        .iter()
        .map(|(key, val)| {
            Line::from(vec![
                Span::raw("  "),
                Span::styled(
                    format!("{:<22}", key),
                    Style::default().fg(CLR_ACCENT).add_modifier(Modifier::BOLD),
                ),
                Span::styled(val.as_str(), Style::default().fg(CLR_TEXT)),
            ])
        })
        .collect();

    let paragraph = Paragraph::new(lines)
        .block(block)
        .wrap(Wrap { trim: false });

    f.render_widget(paragraph, area);
}

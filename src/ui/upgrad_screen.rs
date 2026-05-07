use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, Paragraph},
};

fn centered_rect(width: u16, height: u16, area: Rect) -> Rect {
    let x = area.x + area.width.saturating_sub(width) / 2;
    let y = area.y + area.height.saturating_sub(height) / 2;
    Rect::new(x, y, width.min(area.width), height.min(area.height))
}

pub fn draw(f: &mut Frame, current_pkg: &str, progress: usize, total: usize, logs: &[String]) {
    let area = centered_rect(70, 20, f.area());
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // title
            Constraint::Length(3), // progress bar
            Constraint::Length(3), // current package
            Constraint::Min(5),    // logs
        ])
        .split(area);

    // ── Title ──
    let title = Paragraph::new(Line::from(vec![
        Span::styled(
            " 📦 Upgrading packages... ",
            Style::default()
                .fg(Color::Black)
                .bg(Color::LightMagenta)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("  "),
        Span::styled(
            format!("{}/{}", progress, total),
            Style::default().fg(Color::White),
        ),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray)),
    );
    f.render_widget(title, chunks[0]);

    // ── Progress bar ──

    let percent = if total > 0 {
        (progress as f64 / total as f64).min(1.0)
    } else {
        0.0
    };

    let gauge = Gauge::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray)),
        )
        .gauge_style(Style::default().fg(Color::LightMagenta).bg(Color::Black))
        .ratio(percent)
        .label(format!("{}%", (percent * 100.0) as u16));
    f.render_widget(gauge, chunks[1]);

    // ── Current package ──
    let pkg_line = Paragraph::new(Line::from(vec![
        Span::styled("  Setting up: ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            current_pkg,
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray)),
    );
    f.render_widget(pkg_line, chunks[2]);
    // ── Logs ──
    let visible: Vec<Line> = logs
        .iter()
        .rev()
        .take(10)
        .rev()
        .map(|l| {
            Line::from(Span::styled(
                l.as_str(),
                Style::default().fg(Color::DarkGray),
            ))
        })
        .collect();
    let log_block = Paragraph::new(visible).block(
        Block::default()
            .title(" Logs ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray)),
    );
    f.render_widget(log_block, chunks[3]);
}

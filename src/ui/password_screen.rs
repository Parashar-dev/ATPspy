use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};


fn centered_rect(width: u16, height: u16, area: Rect) -> Rect {
    let x = area.x + area.width.saturating_sub(width) / 2;
    let y = area.y + area.height.saturating_sub(height) / 2;
    Rect::new(x, y, width.min(area.width), height.min(area.height))
}

pub fn draw(f: &mut Frame, password_len: usize ,error:&str) {
    // Full screen vertical layout
    let center = centered_rect(60, 16, f.area());

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(8),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(center);

    // ── Big ASCII art logo ──
    let logo = vec![
        Line::from(Span::styled(
            r" █████╗ ████████╗██████╗ ███████╗██████╗ ██╗   ██╗",
            Style::default()
                .fg(Color::LightMagenta)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            r"██╔══██╗╚══██╔══╝██╔══██╗██╔════╝██╔══██╗╚██╗ ██╔╝",
            Style::default()
                .fg(Color::LightMagenta)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            r"███████║   ██║   ██████╔╝███████╗██████╔╝ ╚████╔╝ ",
            Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            r"██╔══██║   ██║   ██╔═══╝ ╚════██║██╔═══╝   ╚██╔╝  ",
            Style::default().fg(Color::Magenta),
        )),
        Line::from(Span::styled(
            r"██║  ██║   ██║   ██║     ███████║██║        ██║   ",
            Style::default().fg(Color::DarkGray),
        )),
        Line::from(Span::styled(
            r"╚═╝  ╚═╝   ╚═╝   ╚═╝     ╚══════╝╚═╝        ╚═╝   ",
            Style::default().fg(Color::DarkGray),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "Your APT updates, decoded.",
            Style::default().fg(Color::DarkGray),
        )),
    ];

    let logo_widget = Paragraph::new(logo).alignment(Alignment::Center);
    f.render_widget(logo_widget, chunks[0]);

    // ── Prompt ──
    let prompt = Paragraph::new(Span::styled(
        " 🔒 Enter sudo password:",
        Style::default().fg(Color::White),
    ));
    f.render_widget(prompt, chunks[2]);

    // ── Password input ──
    let masked = "● ".repeat(password_len);
    let display = if password_len == 0 {
        Span::styled("  type here...", Style::default().fg(Color::DarkGray))
    } else {
        Span::styled(format!("  {}", masked), Style::default().fg(Color::Cyan))
    };

    let input = Paragraph::new(Line::from(display)).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::LightMagenta)),
    );
    f.render_widget(input, chunks[3]);

    // ── Footer hint ──
    let hint = Paragraph::new(Span::styled(
        "  Enter: submit  │  Esc: quit",
        Style::default().fg(Color::DarkGray),
    ));
    // ── Error message ──
    if !error.is_empty() {
        let err_widget = Paragraph::new(Span::styled(
            format!("  {}", error),
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        ));
        f.render_widget(err_widget, chunks[4]); // Min(0) chunk use karo
    }

    f.render_widget(hint, chunks[4]);
}

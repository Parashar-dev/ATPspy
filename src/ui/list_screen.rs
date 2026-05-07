use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};

use crate::models::{Package, RiskLevel};

pub fn draw(f: &mut Frame, packages: &[Package], selected_idx: usize) {
    let outer = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(6),
            Constraint::Min(5),
            Constraint::Length(3),
        ])
        .split(f.area());

    //header code //
    let selected_count = packages.iter().filter(|p| p.selected).count();
    let critical = packages
        .iter()
        .filter(|p| matches!(p.risk_level, RiskLevel::Critical | RiskLevel::High))
        .count();
    let medium = packages
        .iter()
        .filter(|p| matches!(p.risk_level, RiskLevel::Medium))
        .count();
    let low = packages
        .iter()
        .filter(|p| matches!(p.risk_level, RiskLevel::Low))
        .count();

           let header_text = vec![
        Line::from(Span::styled(
            r" ▄▀█ ▀█▀ █▀█ █▀ █▀█ █▄█",
            Style::default().fg(Color::LightMagenta).add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            r" █▀█  █  █▀▀ ▄█ █▀▀  █ ",
            Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(vec![
            Span::raw("  "),
            Span::styled(format!("{} pkgs", packages.len()), Style::default().fg(Color::White)),
            Span::raw("  │  "),
            Span::styled(format!("🔴 {}", critical), Style::default().fg(Color::Red)),
            Span::raw(" "),
            Span::styled(format!("🟡 {}", medium), Style::default().fg(Color::Yellow)),
            Span::raw(" "),
            Span::styled(format!("🟢 {}", low), Style::default().fg(Color::Green)),
            Span::raw("  │  "),
            Span::styled(format!("✓ {} selected", selected_count), Style::default().fg(Color::Cyan)),
        ]),
    ];



    let header = Paragraph::new(header_text).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan)),
    );
    f.render_widget(header, outer[0]);

    //left side package list

    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(45), Constraint::Percentage(55)])
        .split(outer[1]);

    let items: Vec<ListItem> = packages
        .iter()
        .map(|pkg| {
            let checkbox = if pkg.selected { "[✓]" } else { "[ ]" };
            let cb_color = if pkg.selected {
                Color::Cyan
            } else {
                Color::DarkGray
            };

            ListItem::new(Line::from(vec![
                Span::styled(format!("{}", checkbox), Style::default().fg(cb_color)),
                Span::raw(format!("{}", pkg.risk_level.symbol())),
                Span::styled(
                    format!("{:<28}", pkg.name),
                    Style::default().fg(Color::DarkGray),
                ),
                Span::styled(&pkg.repo, Style::default().fg(Color::DarkGray)),
            ]))
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .title("Packages")
                .borders(Borders::ALL)
                .border_style(
                    Style::default()
                        .fg(Color::LightMagenta)
                        .add_modifier(Modifier::BOLD),
                ),
        )
        .highlight_symbol("▸ ")
        .highlight_style(
            Style::default()
                .bg(Color::LightMagenta)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        );

    let mut state = ListState::default();
    state.select(Some(selected_idx));
    f.render_stateful_widget(list, main_chunks[0], &mut state);

    //right side package detailed area

    let detail = if let Some(pkg) = packages.get(selected_idx) {
        vec![
            Line::from(""),
            Line::from(vec![
                Span::styled("Name:     ", Style::default().fg(Color::DarkGray)),
                Span::styled(&pkg.name, Style::default().fg(Color::Cyan)),
            ]),
            Line::from(vec![
                Span::styled("  Risk:     ", Style::default().fg(Color::DarkGray)),
                Span::raw(format!(
                    "{} {}",
                    pkg.risk_level.symbol(),
                    pkg.risk_level.label()
                )),
            ]),
            Line::from(vec![
                Span::styled("Repo:     ", Style::default().fg(Color::DarkGray)),
                Span::styled(&pkg.repo, Style::default().fg(Color::Cyan)),
            ]),
            Line::from(vec![
                Span::styled("Current:  ", Style::default().fg(Color::DarkGray)),
                Span::raw(&pkg.current_version),
            ]),
            Line::from(vec![
                Span::styled("New:      ", Style::default().fg(Color::DarkGray)),
                Span::styled(&pkg.new_version, Style::default().fg(Color::Green)),
            ]),
        ]
    } else {
        vec![Line::from("NO package selected ")]
    };

    let detail_block = Block::default()
        .title("Details")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));

    f.render_widget(Paragraph::new(detail).block(detail_block), main_chunks[1]);

    //footer

    let footer = Paragraph::new(Line::from(Span::styled(
        "  ↑↓/jk: nav │ Space: select │ a: all │ d: none │ u: upgrade │ q: quit",
        Style::default().fg(Color::DarkGray),
    )))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray)),
    );
    f.render_widget(footer, outer[2]);
}

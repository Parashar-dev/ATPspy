

use ratatui::{
    layout::{Constraint,Direction,Layout},
    style::{Color,Modifier,Style},
    text::{Line,Span},
    widgets::{Block , Borders, List , ListItem ,ListState ,Paragraph },
    Frame,
};

use crate::models::{Package};


pub fn draw(f: &mut Frame ,packages: &[Package], selected_idx:usize) {

    let chunks =Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([Constraint::Percentage(50),Constraint::Percentage(50)])
        .split(f.area());


//left side package list 


let items : Vec<ListItem> = packages
    .iter()
    .map(|pkg| {
        ListItem::new(Line::from(vec![
            Span::styled(
                format!("{:<28}",pkg.name),
                Style::default().fg(Color::DarkGray),
            ),
            Span::styled(
               &pkg.repo,
               Style::default().fg(Color::DarkGray),
            )
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
                    .add_modifier(Modifier::BOLD)
            )                                                   
    )                                                            
    .highlight_symbol("▸ ")                                     
    .highlight_style(
        Style::default()
            .bg(Color::LightMagenta)
            .fg(Color::Black)
            .add_modifier(Modifier::BOLD)
    );

let mut state = ListState::default();
state.select(Some(selected_idx));
f.render_stateful_widget(list , chunks[0],&mut state);

//right side package detailed area

let detail = if let Some(pkg) = packages.get(selected_idx){
    vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("Name:     ", Style::default().fg(Color::DarkGray)),
            Span::styled(&pkg.name, Style::default().fg(Color::Cyan)),
        ]),
        Line::from(vec![
            Span::styled("Repo:     ", Style::default().fg(Color::DarkGray)),
            Span::styled(&pkg.repo, Style::default().fg(Color::Cyan))
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
}else {
    vec![Line::from("NO package selected ")]
};

let detail_block = Block::default()
    .title("Details")
    .borders(Borders::ALL)
    .border_style(Style::default().fg(Color::DarkGray));

f.render_widget(Paragraph::new(detail).block(detail_block), chunks[1]);

}
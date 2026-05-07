use ratatui::{
    layout::{Constraint,Direction,Layout,Rect},
    style::{Color,Modifier,Style},
    text::{Line,Span},
    widgets::Paragraph,
    Frame,
};

const SPINNER: &[&str]= &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

fn center_rect(width:u16,height:u16 , area:Rect)-> Rect{
    let x = area.x +area.width.saturating_sub(width)/2;
    let y = area.y +area.height.saturating_sub(height)/2;
    Rect:: new(x, y, width.min(area.width), height.min(area.height))
}

pub  fn draw(f: &mut Frame , tick : usize , logs:&[String]){
    let center = center_rect(60, 14 , f.area());
    let  chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(1),
            Constraint::Min(5),
        ])
        .split(center);

    // spinner 

    let frame = SPINNER[tick% SPINNER.len()];
    let spinner_line= Line::from(vec![
        Span::styled(
            format!("  {}  ", frame),
            Style::default().fg(Color::LightMagenta).add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            "Scanning packages....", 
            Style::default().fg(Color::White),
        )
    ]);
    let spinner = Paragraph::new(spinner_line);
    f.render_widget(spinner, chunks[0]);


    // logs

    let visible_logs: Vec<Line> = logs
        .iter()
        .rev()
        .take(8)
        .rev()
        .map(|l| Line::from(Span::styled(l.as_str(), Style::default().fg(Color::DarkGray))))
        .collect();
    let log_widget =Paragraph::new(visible_logs);
    f.render_widget(log_widget, chunks[2]);



}
use ratatui::{
    layout::{Constraint , Direction ,Layout , Rect},
    style::{Color,Modifier,Style},
    text::{Line ,Span},
    widgets::{Block ,Borders,Paragraph},
    Frame
};

fn centered_box(width: u16, height:u16 ,area:Rect) -> Rect{
    let x=area.x+area.width.saturating_sub(width)/2;
    let y =area.y +area.height.saturating_sub(height)/2;
    Rect::new(x,y,width.min(area.width),height.min(area.height))
}

pub fn draw(f: &mut Frame , password_len:usize){
    let box_area = centered_box(50 , 10 ,f.area());

    let block=Block::default()
        .title("ATP - SPY")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));


    let inner  = block.inner(box_area);
    f.render_widget(block , box_area );

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(inner);

    let prompt = Paragraph::new("Enter your sudo password");
    f.render_widget(prompt , chunks[1]);

    let masked = "●".repeat(password_len);
    let display = if password_len ==0{
         Span::styled("  type here...", Style::default().fg(Color::DarkGray))
    } else {
        Span::styled(format!("  {}", masked), Style::default().fg(Color::Cyan))
    };

    let input =Paragraph :: new(Line::from(display))
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::DarkGray)));
    f.render_widget(input,chunks[3]);
    }

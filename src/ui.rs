use ratatui::{prelude::*, widgets::*};
use crate::app::{App, CurrentScreen};

fn gen_title() -> ratatui::widgets::Paragraph<'static> {
    let title_block  = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        "GTAEFk Economy General Map Device n23",
        Style::default().fg(Color::Green),
    ))
        .block(title_block);

    title
}

pub fn ui(f: &mut Frame, app: &App) {

    let v_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(f.size());

    // let title_block: Block = Block::default()
    //     .borders(Borders::ALL)
    //     .style(Style::default());
    //
    // let title: Paragraph = Paragraph::new(Text::styled(
    //     "GTAEFk Economy General Map Device n23",
    //     Style::default().fg(Color::Green),
    // ))
    //     .block(title_block);

    let current_navigation_text: Vec<Span> = vec![
        // The first half of the text
        match app.current_screen {
            CurrentScreen::Main => {
                Span::styled("Mode: Map Display", Style::default().fg(Color::Green))
            }
            CurrentScreen::Config => {
                Span::styled("Mode: Map Configuration", Style::default().fg(Color::Green))
            }
            CurrentScreen::Edit => {
                Span::styled("Mode: Map Edit", Style::default().fg(Color::Yellow))
            }
            CurrentScreen::Exit => {
                Span::styled("Mode: Save", Style::default().fg(Color::LightRed))
            }
            CurrentScreen::Start => {
                Span::styled("Mode: Start", Style::default().fg(Color::LightRed))
            }
            CurrentScreen::Loader => {
                Span::styled("Mode: Save", Style::default().fg(Color::LightRed))
            }
        }
            .to_owned(),
        // A white divider bar to separate the two sections
        Span::styled(" | ", Style::default().fg(Color::White)),
        match app.current_screen {
            CurrentScreen::Main => {
                Span::styled("Mode: Map Display", Style::default().fg(Color::Green))
            }
            CurrentScreen::Config => {
                Span::styled("Mode: Map Configuration", Style::default().fg(Color::Green))
            }
            CurrentScreen::Edit => {
                Span::styled("Mode: Map Edit", Style::default().fg(Color::Yellow))
            }
            CurrentScreen::Exit => {
                Span::styled("Mode: Save", Style::default().fg(Color::LightRed))
            }
            CurrentScreen::Start => {
                Span::styled("Keys: Start", Style::default().fg(Color::LightRed))
            }
            CurrentScreen::Loader => {
                Span::styled("Mode: Save", Style::default().fg(Color::LightRed))
            }
        }
    ];

    let mode_footer = Paragraph::new(Line::from(current_navigation_text))
        .block(Block::default().borders(Borders::ALL));

    let current_keys_hint = {
        match app.current_screen {
            CurrentScreen::Main => {
                Span::styled("Keys: Map Display", Style::default().fg(Color::Green))
            }
            CurrentScreen::Config => {
                Span::styled("Keys: Map Configuration", Style::default().fg(Color::Green))
            }
            CurrentScreen::Edit => {
                Span::styled("Keys: Map Edit", Style::default().fg(Color::Yellow))
            }
            CurrentScreen::Exit => {
                Span::styled("Mode: Save", Style::default().fg(Color::LightRed))
            }
            CurrentScreen::Start => {
                Span::styled("Mode: Save", Style::default().fg(Color::LightRed))
            }
            CurrentScreen::Loader => {
                Span::styled("Mode: Save", Style::default().fg(Color::LightRed))
            }
        }
    };

    let key_notes_footer = Paragraph::new(Line::from(current_keys_hint))
        .block(Block::default().borders(Borders::ALL));

    let footer_split = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(v_layout[2]);
    
    f.render_widget(gen_title(), v_layout[0]);
    f.render_widget(Span::raw("This should be a grid..."), v_layout[1]);
    f.render_widget(mode_footer, footer_split[0]);
    f.render_widget(key_notes_footer, footer_split[1]);
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}
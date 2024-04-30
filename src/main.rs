use std::error::Error;
use crossterm::event::*;
use crossterm::{event, execute}; //we could just grab it all but we aren't right now
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use std::io;
use ratatui::backend::*;
use ratatui::Terminal;

// grab our App mod via the standard way, so its just named app and in the same hierarchy I guess
mod app;
mod ui;

use app::*;
use crate::ui::ui;

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stderr = io::stderr(); // This is a special case. Normally using stdout is fine
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    // create default app struct
    let mut app = App::default();
    let res = run_app(&mut terminal, &mut app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;

    terminal.show_cursor()?;

    if let Ok(do_print) = res {
        if do_print {
            // app.print_json()?;
            println!("Foo->");
        }
    } else if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> io::Result<bool> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        // start to handle key press events
        // TODO add an exit key press response here
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                // Skip events that are not KeyEventKind::Press
                continue;
            }
            match app.current_screen {
                CurrentScreen::Config => match key.code {
                    KeyCode::Char('c') => {
                        app.current_screen = CurrentScreen::Config;
                    }
                    KeyCode::Char('q') => {
                        app.current_screen = CurrentScreen::Exit;
                    }
                    _ => {}
                }
                CurrentScreen::Main => match key.code {
                    KeyCode::Char('m') => {
                        app.current_screen = CurrentScreen::Main;
                        // app.currently_editing = Some(CurrentlyEditing::Key);
                    }
                    KeyCode::Char('q') => {
                        app.current_screen = CurrentScreen::Exit;
                    }
                    _ => {}
                },
                CurrentScreen::Exit => match key.code {
                    KeyCode::Char('y') => {
                        return Ok(true);
                    }
                    KeyCode::Char('n') | KeyCode::Char('q') => {
                        return Ok(false);
                    }
                    _ => {}
                },
                CurrentScreen::Edit if key.kind == KeyEventKind::Press => {
                    match key.code {
                        KeyCode::Enter => {
                            // app.save_key_value();
                            // app.current_screen =
                            //     CurrentScreen::Main;
                        }
                        KeyCode::Esc => {
                            app.current_screen = CurrentScreen::Main;
                        }
                        KeyCode::Tab => {
                            // app.toggle_editing();
                        }
                        KeyCode::Char('q') => {
                            app.current_screen = CurrentScreen::Exit;
                        }
                        KeyCode::Char(value) => {
                           if app.ap_chars.contains(&value) {
                               println!("HOORAH!!!");
                               app.key_input = value;
                           }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }
}


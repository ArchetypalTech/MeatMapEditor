use std::io;
use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use crate::app;

/*
*  Holds the state of the application and is updated in between
*  each render cycle. Used to build up the map array that then
*  is turned into yml.
*  Valid chars are: ['.', 'o', 'x', '-', '|']
*  There is no logical difference between a 'o', 'x' char as they denote
*  respectively a room and a path, logically both have exits and objects i.e.
*  they are the same thing, but it seems that conceptually when drawing a map
*  this separation might well be useful.
*
 */
#[derive(Debug, Default)]
pub struct App {
    pub(crate) current_screen: CurrentScreen::Config,
    key_input: String::new(),
    current_cell: Pos::Default(),
    map_chars: vec!['.', 'x', 'o', '-', '|'],
    exit: bool,
}

#[derive(Debug)]
pub enum CurrentScreen {
    #[default]
    Config,
    Main,
    Editing,
    Exiting,
}

#[derive(Debug, Default)]
pub struct Pos {
    x_pos: u8,
    y_pos: u8,
}

#[derive(Debug, Default)]
pub struct Cell {
    coord: Pos::Default(),
    value: String::new()
}

impl App {

    pub fn save_key_value(&mut self) {
        self.key_input = String::new();
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        print!("Handle key event.... APP");
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Left => self.decrement_counter(),
            KeyCode::Right => self.increment_counter(),
            _ => {}
        }
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }
}
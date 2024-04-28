
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
#[derive(Debug)]
pub struct App {
    pub(crate) current_screen: CurrentScreen,
    pub(crate) key_input: char,
    pub(crate) ap_chars: Vec<char>,
    pub(crate) exit: bool,
    pub(crate) current_cell: Cell,
}

#[derive(Debug, Default)]
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
    coord: Pos,
    value: char,
}

impl Default for App {
    fn default() -> Self {
        Self {
            current_screen: CurrentScreen::default(),  // Uses the default from the enum
            key_input: '\0',                          // Default to a null character
            current_cell: Cell::default(),            // Uses the manually implemented default
            ap_chars: Vec::new(),                     // Default to an empty vector
            exit: false,                              // Default to false
        }
    }
}
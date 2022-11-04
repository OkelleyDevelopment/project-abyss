//! The UI structures and associated functions

use self::window::AbyssWindow;

pub mod window;

/// Creates and returns an AbyssWindow
///
/// Current implementation relies on the Pancurses window.
pub fn init_screen() -> AbyssWindow {
    AbyssWindow::new()
}

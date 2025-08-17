// LABYRINTH
// Printing to console, and debug.

// UI - Visualisation of the features on the tiles
// cf. vars defs.


use super::grid;

/// UI - Visualisation of the status of the tiles.
pub const LABYRINTH_UI_TILES: grid::UiTiles = grid::UiTiles {
    on: "░░",
    off: "██",
    void: "▒▒"
};



/// DEBUG - Define kind of info that are available
#[derive(PartialEq)]
pub enum DebugLogging {
    All,
    Minimal,
    None,
}
/// DEBUG - Which info to actually display.
pub const DEBUG_LOGGING: DebugLogging = DebugLogging::All;

/// According to DEBUG_LOGGING, print given text, or not.
pub fn dp(text: String, level_min: DebugLogging) -> () {
    if DEBUG_LOGGING != DebugLogging::None {
        if DEBUG_LOGGING == level_min || DEBUG_LOGGING == DebugLogging::All {
            print!("{}", text);
        }
    }
}
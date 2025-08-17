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

/// DEBUG - Activate.
pub const DEBUG_ON: bool = true;

/// DEBUG - Define kind of info that are available
#[derive(PartialEq)]
pub enum DebugLogging {
    All,
    Minimal,
    None,
}
/// DEBUG - Which info to actually display.
pub const DEBUG_LOGGING: DebugLogging = DebugLogging::Minimal;
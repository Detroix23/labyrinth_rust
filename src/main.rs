// LABYRINTH


use std::{collections::HashMap, io};

// Imports
use grid::{self, Grid, GridKind, Tile, TileFeatures, TileState, UiTiles};
use rand::{rng, Rng, seq::{self, IndexedRandom}};


// Vars
/// Private struct to locate the generator.
#[derive(Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

/// Private enum for directions.
#[derive(PartialEq)]
enum OrdinalDirections {
    North,
    East,
    South,
    West,
}

/*
// Useless.
/// Relative cords for neighbours in a arc of radius 2, the field of view pointing Y.
const NEIGHBOURS_ARC_Y_2: [Position; 8] = [
    Position { x: -2, y: 0}, Position { x: -1, y: 1}, 
    Position { x: 0, y: 2}, Position { x: 1, y: 1},
    Position { x: 2, y: 0}, Position { x: -1, y: 0},
    Position { x: 0, y: 1}, Position { x: 1, y: 0},
];
/// Relative cords for neighbours in a arc of radius 2, the field of view pointing X.
const NEIGHBOURS_ARC_X_2: [Position; 8] = [
    Position { x: 0, y: 2}, Position { x: 1, y: 1}, 
    Position { x: 2, y: 0}, Position { x: 1, y: -1},
    Position { x: 0, y: -2}, Position { x: 0, y: 1},
    Position { x: 1, y: 0}, Position { x: 0, y: -1},
];
*/

const NEIGHBOURS_ARC_Y_1: [Position; 5] = [
    Position { x: -1, y: 0}, Position { x: -1, y: 1}, 
    Position { x: 0, y: 1}, Position { x: 1, y: 1},
    Position { x: 1, y: 0},      
];

/// Relative cords for neighbours in a arc of radius 2, the field of view pointing X.
const NEIGHBOURS_ARC_X_1: [Position; 5] = [
    Position { x: 0, y: 1}, Position { x: 1, y: 1},
    Position { x: 1, y: 0}, Position { x: 1, y: -1},
    Position { x: 0, y: -1},
];

/// Behaviour of the engine when stuck
pub enum StuckReaction {
    OneStepBack,
    RandomPosition,
}

/// DEFAULT - Size
pub const DEFAULT_SIZE: usize = 32;
/// DEFAULT - Iteration limit
pub const DEFAULT_ITERATION_LIMIT: usize = 0;

// UI - Visualisation of the features on the tiles
// cf. vars defs.

/// UI - Visualisation of the status of the tiles.
pub const LABYRINTH_UI_TILES: UiTiles = UiTiles {
    on: "░░",
    off: "██",
    void: "▒▒"
};

/// DEBUG - Activate.           d
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

/// # Labyrinth generator; memory based, no recursion.
/// Take random directions and saved the path in a vector. It "hits a wall" if the tile after wich its facing is a path. When stuck, go back one step reading its memory.
/// Iteration limit to zero to disable the limit.
pub fn generator_random_memory_based(grid_size: usize, iteration_limit: usize, labyrinth_ui_features: &HashMap<TileFeatures, &'static str>) -> Grid {
    if DEBUG_LOGGING == DebugLogging::Minimal || DEBUG_LOGGING == DebugLogging::All {println!("## Generating labyrinth.");}
    // Init
    let grid_default_state: bool = false;
    let grid_default_features: Vec<TileFeatures> = Vec::new();
    let grid_kind: GridKind = GridKind::Squares;
    let stuck_reaction: StuckReaction = StuckReaction::RandomPosition;
    let mut grid_labyrinth: Grid = Grid::new(grid_kind, grid_size, grid_default_state);  
    let mut counter: usize = 0; 

    // Start at the middle of the grid
    let mut generator_position: Position = Position { x: grid_labyrinth.size.x as i32 / 2, y: grid_labyrinth.size.y as i32 / 2 };
    grid_labyrinth.update_tile(
        generator_position.x, 
        generator_position.y, 
        !grid_default_state, 
        vec![TileFeatures::Named("Entrance")]
    );
    let mut generator_path: Vec<Position> = vec![generator_position];
    let mut generator_index: usize = 0;
    if DEBUG_LOGGING == DebugLogging::Minimal || DEBUG_LOGGING == DebugLogging::All {println!("- Vars initalized.\n- Starting main loop.");}
    
    // Generator, end when the generator has backed up totaly.
    while generator_path.len() > 0 && (iteration_limit < 1 || counter < iteration_limit) {
        counter += 1usize;
        let mut good_path: bool = false;
        let mut available_directions: Vec<OrdinalDirections> = vec![OrdinalDirections::North, OrdinalDirections::East, OrdinalDirections::South, OrdinalDirections::West];

        if DEBUG_LOGGING == DebugLogging::All {print!(" - Iter {}; ", counter);}
        // Expect to find a good path; if not, if all direction are blocked
        while !good_path && available_directions.len() > 0 {
            let mut offset_x: i32 = 0;
            let mut offset_y: i32 = 0;
            let direction: &OrdinalDirections  = available_directions.choose(&mut rand::rng()).expect("(!) - Something went wrong with the random choice");
            match direction {
                OrdinalDirections::North => offset_y = 1,
                OrdinalDirections::East => offset_x = 1,
                OrdinalDirections::South => offset_y = -1,
                OrdinalDirections::West => offset_x = -1,
            };
            if DEBUG_LOGGING == DebugLogging::All {print!("Dir, off: x={}, y={}; ", offset_x, offset_y);}

            // Move like it is good.
            generator_position = Position { x: generator_position.x + offset_x, y: generator_position.y + offset_y};
            let generator_position_state: TileState = grid_labyrinth.state_tile(generator_position.x, generator_position.y);
            // Check neighbours, with the "field of view", according to the offset.
            let mut generator_neighbours_pass: bool = true;
            if offset_x == 0 {
                for neighbour in NEIGHBOURS_ARC_Y_1 {
                    if grid_labyrinth.state_tile(generator_position.x + neighbour.x, generator_position.y + offset_y * neighbour.y) != TileState::Off {
                        generator_neighbours_pass = false;
                    }
                }
            } else {
                for neighbour in NEIGHBOURS_ARC_X_1 {
                    if grid_labyrinth.state_tile(generator_position.x + offset_x * neighbour.x, generator_position.y + neighbour.y) != TileState::Off {
                        generator_neighbours_pass = false;
                    }
                }
            }
            

            if DEBUG_LOGGING == DebugLogging::All {print!("Gene pos: x={}, y={}; ", generator_position.x, generator_position.y);}
            
            if generator_position_state == TileState::Off && generator_neighbours_pass {
                grid_labyrinth.update_tile(generator_position.x, generator_position.y, !grid_default_state, grid_default_features.clone());
                good_path = true;
            } else {
                // Nevermind, tile was not good, go back to the original tile.
                generator_position = Position { x: generator_position.x - offset_x, y: generator_position.y - offset_y};
                available_directions.remove(available_directions.iter().position(|d| d == direction).expect("(!) - Can't find direction."));
            }
        }

        if good_path {
            generator_path.push(generator_position);
            generator_index += 1;
            if DEBUG_LOGGING == DebugLogging::All {print!("Good path. ");}
        } else {
            if generator_index > 0 {
                generator_index -= 1;
            }

            let new_branch_start: usize;
            match stuck_reaction {
                // Method branch-random
                StuckReaction::RandomPosition => {
                    generator_path.remove(generator_index);

                    if generator_path.len() > 0 {
                        new_branch_start = rng().random_range(0..generator_path.len());
                        if DEBUG_LOGGING == DebugLogging::All {print!("Stuck: rewinding (RP). ");}
                        generator_position = generator_path[new_branch_start];
                    } else {
                        if DEBUG_LOGGING == DebugLogging::Minimal || DEBUG_LOGGING == DebugLogging::All {println!("- Reached end. ");}
                        generator_position = Position {x: 0, y: 0};
                    }
                },
                // Method branch-one-step-backward (as a default)
                _ => {
                    generator_path.pop();
                    
                    if generator_path.len() > 0 {
                        new_branch_start = generator_path.len() - 1;
                        if DEBUG_LOGGING == DebugLogging::All {print!("Stuck: rewinding (OSP). ");}
                        generator_position = generator_path[new_branch_start];
                    } else {
                        if DEBUG_LOGGING == DebugLogging::Minimal || DEBUG_LOGGING == DebugLogging::All {println!("- Reached end. ");}
                        generator_position = Position {x: 0, y: 0};
                    }           
                },
            }

            
        }
        if iteration_limit >= 1 && counter >= iteration_limit {
            if DEBUG_LOGGING == DebugLogging::Minimal || DEBUG_LOGGING == DebugLogging::All {println!("- Iteration limit reached ({}). ", iteration_limit);}
        }
        if DEBUG_LOGGING == DebugLogging::All {
            println!("");
            grid_labyrinth.display_inline(&LABYRINTH_UI_TILES, &labyrinth_ui_features);
        }
        
    }

    grid_labyrinth
}

fn main() {
    println!("# Labyrinth.");

    println!("## Initialization.");
    // Vars
    let mut labyrinth_size_input: String = String::new();
    let mut iteration_limit_input: String = String::new();
    let labyrinth_ui_features: HashMap<TileFeatures, &'static str> = HashMap::from([
        (TileFeatures::Named("Entrance"), "IN"),
        (TileFeatures::Named("Exit"), "EX"),
    ]);
    // User input
    println!("## User input.");
    println!("- Labyrinth size [N+]({DEFAULT_SIZE}): ");
    io::stdin()
        .read_line(&mut labyrinth_size_input)
        .expect("(X) - Can't read line.");
    let labyrinth_size: usize = match labyrinth_size_input.trim().parse() {
        Ok(num) => num,
        Err(_) => DEFAULT_SIZE,
    };
    println!("- Iteration limit [0 = No limit/ N+]({DEFAULT_ITERATION_LIMIT}): ");
    io::stdin()
        .read_line(&mut iteration_limit_input)
        .expect("(X) - Can't read line.");
    
    let iteration_limit: usize = match iteration_limit_input.trim().parse() {
        Ok(num) => num,
        Err(_) => DEFAULT_ITERATION_LIMIT,
    };

    // Results
    let labyrinth: Grid = generator_random_memory_based(labyrinth_size, iteration_limit, &labyrinth_ui_features);
    println!("\n## Results - Labyrinth: ");
    labyrinth.display_inline(&LABYRINTH_UI_TILES, &labyrinth_ui_features);

    // Prevent window of closing
    println!("\nPress enter to exit... ");
    let _ = io::stdin().read_line(&mut String::new()).expect("(X) - Can't read line; closing anyway.");
}



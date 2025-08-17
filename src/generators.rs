// LABYRINTH
// Generators

use super::{HashMap, seq::IndexedRandom, rng, Rng};
use super::{basics, ui, grid};

/// # Labyrinth generator; memory based, no recursion.
/// Take random directions and saved the path in a vector. It "hits a wall" if the tile after wich its facing is a path. When stuck, go back one step reading its memory.
/// Iteration limit to zero to disable the limit.
pub fn random_memory_based(
    grid_size: usize, 
    iteration_limit: usize, 
    labyrinth_ui_features: &HashMap<grid::TileFeatures, 
    &'static str>
) -> grid::Grid {
    if ui::DEBUG_LOGGING == ui::DebugLogging::Minimal || ui::DEBUG_LOGGING == ui::DebugLogging::All {println!("## Generating labyrinth.");}
    // Init & generator settings
    let grid_default_state: bool = false;
    let grid_default_features: Vec<grid::TileFeatures> = Vec::new();
    let grid_kind: grid::GridKind = grid::GridKind::Squares;
    let stuck_reaction: basics::StuckReaction = basics::StuckReaction::RandomPosition;
    let mut grid_labyrinth: grid::Grid = grid::Grid::new(grid_kind, grid_size, grid_default_state);  
    let mut counter: usize = 0; 

    // Start at the middle of the grid
    let mut generator_position: basics::Position = basics::Position { x: grid_labyrinth.size.x as i32 / 2, y: grid_labyrinth.size.y as i32 / 2 };
    grid_labyrinth.update_tile(
        generator_position.x, 
        generator_position.y, 
        !grid_default_state, 
        vec![grid::TileFeatures::Named("Entrance")]
    );
    let mut generator_path: Vec<basics::Position> = vec![generator_position];
    let mut generator_index: usize = 0;
    if ui::DEBUG_LOGGING == ui::DebugLogging::Minimal || ui::DEBUG_LOGGING == ui::DebugLogging::All {println!("- Vars initalized.\n- Starting main loop.");}
    
    // Generator, end when the generator has backed up totaly.
    while generator_path.len() > 0 && (iteration_limit < 1 || counter < iteration_limit) {
        counter += 1usize;
        let mut good_path: bool = false;
        let mut available_directions: Vec<basics::OrdinalDirections> = vec![basics::OrdinalDirections::North, basics::OrdinalDirections::East, basics::OrdinalDirections::South, basics::OrdinalDirections::West];

        if ui::DEBUG_LOGGING == ui::DebugLogging::All {print!(" - Iter {}; ", counter);}
        // Expect to find a good path; if not, if all direction are blocked
        while !good_path && available_directions.len() > 0 {
            let mut offset_x: i32 = 0;
            let mut offset_y: i32 = 0;
            let direction: &basics::OrdinalDirections  = available_directions.choose(&mut rand::rng()).expect("(!) - Something went wrong with the random choice");
            match direction {
                basics::OrdinalDirections::North => offset_y = 1,
                basics::OrdinalDirections::East => offset_x = 1,
                basics::OrdinalDirections::South => offset_y = -1,
                basics::OrdinalDirections::West => offset_x = -1,
            };
            if ui::DEBUG_LOGGING == ui::DebugLogging::All {print!("Dir, off: x={}, y={}; ", offset_x, offset_y);}

            // Move like it is good.
            generator_position = basics::Position { x: generator_position.x + offset_x, y: generator_position.y + offset_y};
            let generator_position_state: grid::TileState = grid_labyrinth.state_tile(generator_position.x, generator_position.y);
            // Check neighbours, with the "field of view", according to the offset.
            let mut generator_neighbours_pass: bool = true;
            if offset_x == 0 {
                for neighbour in basics::NEIGHBOURS_ARC_Y_1 {
                    if grid_labyrinth.state_tile(generator_position.x + neighbour.x, generator_position.y + offset_y * neighbour.y) != grid::TileState::Off {
                        generator_neighbours_pass = false;
                    }
                }
            } else {
                for neighbour in basics::NEIGHBOURS_ARC_X_1 {
                    if grid_labyrinth.state_tile(generator_position.x + offset_x * neighbour.x, generator_position.y + neighbour.y) != grid::TileState::Off {
                        generator_neighbours_pass = false;
                    }
                }
            }
            

            if ui::DEBUG_LOGGING == ui::DebugLogging::All {print!("Gene pos: x={}, y={}; ", generator_position.x, generator_position.y);}
            
            if generator_position_state == grid::TileState::Off && generator_neighbours_pass {
                grid_labyrinth.update_tile(generator_position.x, generator_position.y, !grid_default_state, grid_default_features.clone());
                good_path = true;
            } else {
                // Nevermind, tile was not good, go back to the original tile.
                generator_position = basics::Position { x: generator_position.x - offset_x, y: generator_position.y - offset_y};
                available_directions.remove(available_directions.iter().position(|d| d == direction).expect("(!) - Can't find direction."));
            }
        }

        if good_path {
            generator_path.push(generator_position);
            generator_index += 1;
            if ui::DEBUG_LOGGING == ui::DebugLogging::All {print!("Good path. ");}
        } else {
            generator_path.remove(generator_index);
            match stuck_reaction {
                // Method branch-random
                basics::StuckReaction::RandomPosition => {
                    if generator_path.len() > 0 {
                        generator_index = rng().random_range(0..generator_path.len());
                        if ui::DEBUG_LOGGING == ui::DebugLogging::All {print!("Stuck: rewinding (RP). ");}
                        generator_position = generator_path[generator_index];
                    } else {
                        if ui::DEBUG_LOGGING == ui::DebugLogging::Minimal || ui::DEBUG_LOGGING == ui::DebugLogging::All {println!("- Reached end. ");}
                        generator_position = basics::Position {x: 0, y: 0};
                    }
                },
                // Method branch-one-step-backward (as a default)
                _ => {
                    if generator_path.len() > 0 {
                        generator_index -= 1;
                        if ui::DEBUG_LOGGING == ui::DebugLogging::All {print!("Stuck: rewinding (OSP). ");}
                        generator_position = generator_path[generator_index];
                    } else {
                        if ui::DEBUG_LOGGING == ui::DebugLogging::Minimal || ui::DEBUG_LOGGING == ui::DebugLogging::All {println!("- Reached end. ");}
                        generator_position = basics::Position {x: 0, y: 0};
                    }           
                },
            }

            
        }
        if iteration_limit >= 1 && counter >= iteration_limit {
            if ui::DEBUG_LOGGING == ui::DebugLogging::Minimal || ui::DEBUG_LOGGING == ui::DebugLogging::All {println!("- Iteration limit reached ({}). ", iteration_limit);}
        }
        if ui::DEBUG_LOGGING == ui::DebugLogging::All {
            println!("");
            grid_labyrinth.display_inline(&ui::LABYRINTH_UI_TILES, &labyrinth_ui_features);
        }
        
    }

    grid_labyrinth
}
// LABYRINTH
// Generators

use crate::basics::Subordination;

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
    let stuck_reaction: basics::StuckReaction = basics::StuckReaction::RandomPosition;
    let grid_default_features: Vec<grid::TileFeatures> = Vec::new();
    let grid_kind: grid::GridKind = grid::GridKind::Squares;
    let islets: basics::Islet = basics::Islet::Yes(0.01f32);
    let subordination: basics::Subordination = basics::Subordination::Yes(0.01f32);

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
            let mut offset_x: i8 = 0;
            let mut offset_y: i8 = 0;
            let direction: &basics::OrdinalDirections = available_directions.choose(&mut rand::rng()).expect("(!) - Something went wrong with the random choice");
            let mut generator_on_border: bool = false;
            match direction {
                basics::OrdinalDirections::North => offset_y = 1,
                basics::OrdinalDirections::East => offset_x = 1,
                basics::OrdinalDirections::South => offset_y = -1,
                basics::OrdinalDirections::West => offset_x = -1,
            };
            ui::dp(format!("Dir, off: x={}, y={}; ", offset_x, offset_y), ui::DebugLogging::All);

            // Move like it is good.
            generator_position = basics::Position { x: generator_position.x + offset_x as i32, y: generator_position.y + offset_y as i32};
            let generator_position_state: grid::TileState = grid_labyrinth.state_tile(generator_position.x, generator_position.y);
            
            // Check neighbours, with the "field of view", according to the offset.
            let mut generator_neighbours_pass: bool = true;
            let generator_arc: [basics::Position; 5];
            if offset_x == 0 {
                generator_arc = basics::NEIGHBOURS_ARC_Y_1;
            } else {
                generator_arc = basics::NEIGHBOURS_ARC_X_1;
            }
            for neighbour in generator_arc {
                let arc_direction_x: i8;
                let arc_direction_y: i8;
                if offset_x == -1 {
                    arc_direction_x = -1;
                } else {
                    arc_direction_x = 1;
                } if offset_y == -1 {
                    arc_direction_y = -1;
                } else {
                    arc_direction_y = 1;
                }

                let tile_state: grid::TileState = grid_labyrinth.state_tile(
                    generator_position.x + arc_direction_x as i32 * neighbour.x, 
                    generator_position.y + arc_direction_y as i32 * neighbour.y
                );
                match tile_state {
                    grid::TileState::On => {
                        generator_neighbours_pass = false;
                        ui::dp(format!("On({};{}), ", neighbour.x, neighbour.y), ui::DebugLogging::All);
                    },
                    grid::TileState::Void => {
                        generator_neighbours_pass = false;
                        generator_on_border = true;
                        ui::dp(format!("Void({};{}), ", neighbour.x, neighbour.y), ui::DebugLogging::All);
                    },
                    grid::TileState::Off => {
                        ui::dp(format!("Off({};{}), ", neighbour.x, neighbour.y), ui::DebugLogging::All);
                    }
                }
            }

            // Apply chance of islet, so reverting the check
            if let basics::Islet::Yes(p) = islets 
                && !generator_neighbours_pass 
                && !generator_on_border {
                if rand::random::<f32>() <= p {
                    generator_neighbours_pass = true;
                    ui::dp(format!("Allowed islet {};", p), ui::DebugLogging::All);
                }
            }

            if ui::DEBUG_LOGGING == ui::DebugLogging::All {print!("Gene pos: x={}, y={}; ", generator_position.x, generator_position.y);}
            
            if let grid::TileState::Off = generator_position_state && generator_neighbours_pass {
                // No neighbours, or allowed to create an islet.
                grid_labyrinth.update_tile(generator_position.x, generator_position.y, !grid_default_state, grid_default_features.clone());
                good_path = true;
            } else {
                // Nevermind, tile was not good, go back to the original tile.
                generator_position = basics::Position { 
                    x: generator_position.x - offset_x as i32, 
                    y: generator_position.y - offset_y as i32};
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
                        generator_position = basics::Position {
                            x: 0, 
                            y: 0
                        };
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
// LABYRINTH

// Imports
use std::{collections::HashMap, io};
use std::time::{Duration, Instant};
use grid::{self, Grid, TileFeatures};
use rand::{seq, rng, Rng};

mod basics;
mod ui;
mod generators;
mod file_handler;




/// DEFAULT - Write file
pub const DEFAULT_WRITE_TO_FILE: bool = false;
/// DEFAULT - Size
pub const DEFAULT_SIZE: usize = 32;
/// DEFAULT - Iteration limit
pub const DEFAULT_ITERATION_LIMIT: usize = 0;





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
    let time_grmb_start: Instant = Instant::now();
    let labyrinth: Grid = generators::random_memory_based(labyrinth_size, iteration_limit, &labyrinth_ui_features);
    let time_grmb_duration: Duration = time_grmb_start.elapsed();

    println!("\n## Results - Labyrinth: ");
    // labyrinth.display_inline(&LABYRINTH_UI_TILES, &labyrinth_ui_features);
    let labyrinth_string: String = labyrinth.to_string(&ui::LABYRINTH_UI_TILES, &labyrinth_ui_features);
    println!("{}", labyrinth_string);
    ui::dp(format!("- Generation time: {:?}\n", time_grmb_duration), ui::DebugLogging::Minimal);

    // Log
    if DEFAULT_WRITE_TO_FILE {
        file_handler::new_labyrinth(format!(
            "{}- Generation time: {:?}", 
            labyrinth_string, time_grmb_duration
        ));
    }

    // Prevent window of closing
    println!("\nPress Enter to exit... ");
    let _ = io::stdin().read_line(&mut String::new()).expect("(X) - Can't read line; closing anyway.");
}

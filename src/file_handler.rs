// LABYRINTH
// Write files

use std::fs::{self, File};
use std::io::{Write};
use std::path::PathBuf;
use chrono::Utc;
use regex::Regex;

pub const LABYRINTH_DIRECTORY: &'static str = "./generated/";


pub fn directory_initialize(dir_path: &PathBuf) {
    if !dir_path.exists() {
        std::fs::create_dir_all(&dir_path).unwrap();
        println!("(+) file_handler.rs - Directory created");
    } else {
        // println!("(~) file_handler.rs - Directory already present");
    }
    
}

pub fn new_labyrinth(labyrinth_string: String) {
    let regex_laby: Regex = Regex::new(r"laby[0-9]+.txt").unwrap();
    let regex_number: Regex = Regex::new(r"[0-9]+").unwrap();

    // Check directory
    let directory: PathBuf = PathBuf::from(LABYRINTH_DIRECTORY);
    directory_initialize(&directory);

    // Find the id to name the file
    let files_all: Vec<String> = match fs::read_dir(&directory) {
        Ok(files) => {
            let mut files_vector: Vec<String> = vec![];
            for file in files {
                files_vector.push(
                    String::from(
                        file
                            .unwrap()
                            .path()
                            .file_name()
                            .unwrap()
                            .to_os_string()
                            .to_str()
                            .expect("(!) file_handler.rs - Excpected str conversion.")
                    )
                );
                
            }
            files_vector
        }
        Err(_) => vec![],
    };
    
    let mut file_id: usize = 0;
    for file  in files_all {
        if regex_laby.is_match(&file) {
            let file_id_found: usize = regex_number
                .find(&file)
                .unwrap()
                .as_str()
                .parse::<usize>()
                .expect("(X) - ID Found is not a number");
            if file_id_found >= file_id {
                file_id = file_id_found + 1usize;
            }
        }
    }

    // Create the file
    let path_string: String = format!("{}laby{}.txt", LABYRINTH_DIRECTORY, file_id);
    let path: PathBuf = PathBuf::from(path_string);
    let path_display: std::path::Display<'_> = path.display();

    let mut file = match File::create(&path) {
        Ok(file) => file,
        Err(reason) => panic!("(X) file_handler.rs - Couldn't create the log file. Path: {}, Reason: {}", path_display, reason)
    };

    let labyrinth_string: String = format!("Labyrinth. {}\n{}", Utc::now(), labyrinth_string);
    match file.write_all(labyrinth_string.as_bytes()) {
        Ok(_) => println!("(+) file_handler.rs - Labyrinth saved in {}", path_display),
        Err(reason) => panic!("(X) file_handler.rs - Coulnd't write. Path: {}, Reason: {}", path_display, reason)
    };
}

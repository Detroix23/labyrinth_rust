// LABYRINTH
// Basic constants and stuctures.


// Vars
/// Private struct to locate the generator.
#[derive(Clone, Copy)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

/// Private enum for directions.
#[derive(PartialEq)]
pub enum OrdinalDirections {
    North,
    East,
    South,
    West,
}

pub const NEIGHBOURS_ARC_Y_1: [Position; 5] = [
    Position { x: -1, y: 0}, Position { x: -1, y: 1}, 
    Position { x: 0, y: 1}, Position { x: 1, y: 1},
    Position { x: 1, y: 0},      
];

/// Relative cords for neighbours in a arc of radius 2, the field of view pointing X.
pub const NEIGHBOURS_ARC_X_1: [Position; 5] = [
    Position { x: 0, y: 1}, Position { x: 1, y: 1},
    Position { x: 1, y: 0}, Position { x: 1, y: -1},
    Position { x: 0, y: -1},
];



/// Behaviour of the engine when stuck
pub enum StuckReaction {
    OneStepBack,
    RandomPosition,
}

/// Behaviour when the engine encouter a wall: can it join "properly" two paths.
pub enum Islet {
    No,
    Yes(f32),
}

/// If the generator can completely ignore the pathing rules, and, if yes, the probability.
pub enum Subordination {
    No,
    Yes(f32)
}


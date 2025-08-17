// LABYRINTH
// Basic constants, stuctures, and default basic generator values.


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
/// Relative cords for neighbours in a arc of radius 1, the field of view pointing Y.
pub const NEIGHBOURS_ARC_Y_1: [Position; 5] = [
    Position { x: -1, y: 0}, Position { x: -1, y: 1}, 
    Position { x: 0, y: 1}, Position { x: 1, y: 1},
    Position { x: 1, y: 0},      
];
/// Relative cords for neighbours in a arc of radius 1, the field of view pointing X.
pub const NEIGHBOURS_ARC_X_1: [Position; 5] = [
    Position { x: 0, y: 1}, Position { x: 1, y: 1},
    Position { x: 1, y: 0}, Position { x: 1, y: -1},
    Position { x: 0, y: -1},
];
/// Relative cords for Islet neighbours in a arc of radius 2, the field of view pointing X.
pub const NEIGHBOURS_ARC_XI_1: [Position; 2] = [
    Position { x : 0, y : 1}, Position { x : 0, y : -1}
];
/// Relative cords for Islet neighbours in a arc of radius 2, the field of view pointing Y.
pub const NEIGHBOURS_ARC_YI_1: [Position; 2] = [
    Position { x : 1, y : 0}, Position { x : -1, y : 0}
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
pub enum Unsubordination {
    No,
    Yes(f32)
}

/// DEFAULT - Write file
pub const DEFAULT_WRITE_TO_FILE: bool = false;
/// DEFAULT - Size
pub const DEFAULT_SIZE: usize = 32;
/// DEFAULT - Iteration limit
pub const DEFAULT_ITERATION_LIMIT: usize = 0;
/// DEFAULT - Generator behaviour, if allow to create islet by doings "bridges".
pub const DEFAULT_ISLET: Islet = Islet::Yes(0.01f32);
/// DEFAULT - Generator behaviour, unsubordination or to ignore all rules of wall contact, except exits.
pub const DEFAULT_UNSUBORDINATION: Unsubordination = Unsubordination::No;
/// DEFAULT - Generator behaviour.
pub const DEFAULT_STUCK: StuckReaction = StuckReaction::RandomPosition;
/// DEFAULT - Generator behaviour.
pub const DEFAULT_STATE: bool = false;
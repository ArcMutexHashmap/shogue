
use std::default;

use bevy::prelude::{State, States, default};

use crate::floor::Floor;
pub struct PlayingState {
    level : u32,
    timer : u64,
    levels : Vec<Floor>,
}
#[derive(States, Debug, Hash, PartialEq, Eq, Default, Clone)]
pub enum GameState {
    #[default]
    Menu,
    Playing,
    Paused,
    GameOver,
    Quit,
}
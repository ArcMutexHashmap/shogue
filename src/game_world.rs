use bevy::prelude::*;
use crate::grid::Grid;


#[derive(Resource)]
pub struct GameWorld {
    //list of grids loaded in the game
    pub grids: Vec<Grid>,
    //the entity that is the player
    pub player: Option<Entity>,
}

impl GameWorld {
    pub fn new() -> Self {
        GameWorld {
            grids: Vec::new(),
            player: None,
        }
    }
}
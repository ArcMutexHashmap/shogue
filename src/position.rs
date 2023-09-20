use bevy::prelude::*;

//positions are floats even though we are using a grid. This is because we want to be able to move from one tile to another smoothly.


#[derive(Debug, Clone, PartialEq, Component,Default)]
pub struct Position(pub f32, pub f32, pub usize);

#[derive(Debug, Clone, PartialEq, Component,Default)]
pub struct Velocity(pub f32, pub f32);


//utility functions for positions, converting between grid and world coordinates (usize and f32)
//distance between two positions, etc.
impl Position {
    pub fn to_grid(&self) -> (usize, usize) {
        (self.0 as usize, self.1 as usize)
    }
    pub fn from_grid(grid_pos : (usize, usize), level : usize) -> Position {
        Position(grid_pos.0 as f32, grid_pos.1 as f32, level)
    }
    pub fn distance(&self, other : &Position) -> f32 {
        let x = self.0 - other.0;
        let y = self.1 - other.1;
        (x*x + y*y).sqrt()
    }
    pub fn to_vec2(&self) -> Vec2 {
        Vec2::new(self.0, self.1)
    }
    pub fn from_vec2(vec : Vec2, level : usize) -> Position {
        Position(vec.x, vec.y, level)
    }
}
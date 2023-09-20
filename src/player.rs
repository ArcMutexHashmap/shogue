use bevy::prelude::*;

#[derive(Default)]
pub enum PlayerClass {
    #[default]
    Warrior,
    Wizard,
}

//The player is a component
#[derive(Component,Default)]
pub struct Player {
    pub class : PlayerClass,
}


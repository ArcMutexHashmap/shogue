use std::default;

use bevy::prelude::*;

use crate::{position::Position, actors::Actor};

#[derive(Component,Default)]
//Corresponds to the creature's current health
pub struct Health(pub u32);

#[derive(Component)]
//Corresponds to the creature's level
pub struct Level(
    pub u32
);
impl default::Default for Level {
    fn default() -> Self {
        Level(1)
    }
}




#[derive(Component,Default)]
pub enum CreatureType {
    #[default]
    Human,
    Elf,
    Dwarf,
    Orc,
    Troll,
    Goblin, 
    Skeleton,
    Zombie,
    Vampire,
    Werewolf,
    Dragon,
    Demon,
    Angel,
    Devil,
    God,
    DeepElf,
    DeepDwarf,
}

pub struct CreatureArchetype {
    //The creature's max health
    pub max_health: u32,
    //The creature's attack, used to calculate damage
    pub attack: u32,
    //The number of dice to roll for attack
    pub attack_rolls: u32,
    //Calculate the attack based on the creature's level on the formula: actual_attack = rolls * floor(attack * attack_scaling ^ level)
    pub attack_scaling: f32,
    //Calculate the max health based on the creature's level on the formula: actual_max_health = floor(max_health * hp_scaling ^ level)
    pub hp_scaling: f32,
    //String for the creature's type's name
    pub type_name: &'static str,
    //String for the creature's type's sprite. Temporary until I figure out how to animated sprites
    pub sprite: &'static str,

}

impl CreatureType {
    pub fn get_stats(&self) -> &'static CreatureArchetype {
        match self {
            _ => &CreatureArchetype {
                max_health: 10,
                attack: 1,
                attack_rolls: 1,
                attack_scaling: 1.1,
                hp_scaling: 1.1,
                type_name: "Human",
                sprite: "player/base/demigod_male.png",
            },
        }
        
    }
}

#[derive(Bundle,Default)]
pub struct CreatureBundle {
    pub creature_type: CreatureType,
    pub position: Position,
    #[bundle()]
    pub sp_bundle: SpriteBundle,
    pub actor: Actor,
}


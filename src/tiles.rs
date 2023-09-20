use bevy::prelude::*;

#[derive(Copy, Clone, PartialEq, Debug, Eq, Hash)]
pub enum TileType {
    Wall,
    Floor,
    DownStairs,
    UpStairs,
    Fountain,
    Water,
    Lava,
}

impl TileType {
    pub fn is_walkable(self) -> bool {
        match self {
            TileType::Wall => false,
            TileType::Floor => true,
            TileType::DownStairs => true,
            TileType::UpStairs => true,
            TileType::Fountain => true,
            TileType::Water => false,
            TileType::Lava => true,
        }
    }

    pub fn is_transparent(self) -> bool {
        match self {
            TileType::Wall => false,
            TileType::Floor => true,
            TileType::DownStairs => true,
            TileType::UpStairs => true,
            TileType::Fountain => true,
            TileType::Water => true,
            TileType::Lava => false,
        }
    }
    pub fn get_texture_str(self) -> &'static str {
        match self {
            TileType::Wall => "dungeon/wall/catacombs_0.png",
            TileType::Floor => "dungeon/floor/cobble_blood_1_new.png",
            TileType::DownStairs => "dungeon/gateways/stone_stairs_down.png",
            TileType::UpStairs => "dungeon/gateways/stone_stairs_up.png",
            TileType::Fountain => "dungeon/blue_fountain.png",
            TileType::Water => "dungeon/water/deep_water_2.png",
            TileType::Lava => "dungeon/floor/lava_0.png",
            _ => "dungeon/floor/white_marble_0.png",
        }
    }
}


//Tile component
#[derive(Copy, Clone, PartialEq, Debug, Eq, Hash,Component)]
pub struct Tile {
    pub tile_type: TileType,
    pub x: u32,
    pub y: u32,
    pub level: usize,
}

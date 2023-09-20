use crate::grid::Grid;


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FloorType {
    Dungeon,
    Town,
    BossRoom,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Floor {
    pub grid : Grid,
    pub floor_type : FloorType,
    pub level : u32,
}


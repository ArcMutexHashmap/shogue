

use crate::tiles::TileType;

use rand::{Rng, seq::SliceRandom};

pub const GRID_SIZE: usize = 64;


#[derive( Clone, PartialEq, Eq, Hash)]
pub struct Grid {
    pub tiles: [[TileType; GRID_SIZE]; GRID_SIZE],
}



impl Grid {
    pub fn new_floor_grid() -> Self {
        let tiles = [[TileType::Floor; GRID_SIZE]; GRID_SIZE];
        Self { tiles: tiles }
    }

    pub fn load_from_string(input : &str) -> Self {
        let mut tiles = [[TileType::Floor; GRID_SIZE]; GRID_SIZE];
        let mut x = 0;
        let mut y = 0;
        for c in input.chars() {
            if c == '\n' {
                x = 0;
                y += 1;
            } else {
                match c {
                    '#' => tiles[x][y] = TileType::Wall,
                    '.' => tiles[x][y] = TileType::Floor,
                    '>' => tiles[x][y] = TileType::DownStairs,
                    '<' => tiles[x][y] = TileType::UpStairs,
                    'F' => tiles[x][y] = TileType::Fountain,
                    ' ' => tiles[x][y] = TileType::Water,
                    'L' => tiles[x][y] = TileType::Lava,
                    _ => (),
                }
                x += 1;
            }
        }
        Self { tiles: tiles }

    }

    pub fn new_cell_automata_grid(initial_density_prob : f32, iterations : u32) -> Self {
        //It is an old and fairly well documented trick to use cellular automata to generate cave-like structures.
        // The basic idea is to fill the first map randomly, then repeatedly create new maps using the 4-5 rule: 
        //a tile becomes a wall if it was a wall and 4 or more of its eight neighbors were walls, or if it was not a wall and 5 or more neighbors were. Put more succinctly,
        // a tile is a wall if the 3x3 region centered on it contained at least 5 walls. 
        //Each iteration makes each tile more like its neighbors, and the amount of overall "noise" is gradually reduced.

        //first, we need to create a grid of tiles
        let mut tiles = [[TileType::Floor; GRID_SIZE]; GRID_SIZE];
        //then, we need to fill the grid with random walls based on the initial density probability
        let mut rng = rand::thread_rng();
        for x in 0..GRID_SIZE {
            for y in 0..GRID_SIZE {
                let random_number = rng.gen_range(0.0..1.0);
                if random_number < initial_density_prob {
                    tiles[x][y] = TileType::Wall;
                }
            }
        }
        //then, we need to iterate over the grid and apply the 4-5 rule
        for _ in 0..iterations {
            let mut new_tiles = [[TileType::Floor; GRID_SIZE]; GRID_SIZE];
            for x in 0..GRID_SIZE {
                for y in 0..GRID_SIZE {
                    let mut wall_count = 0;
                    for x_offset in -1..=1 {
                        for y_offset in -1..=1 {
                            if x_offset == 0 && y_offset == 0 {
                                continue;
                            }
                            let x_pos = x as i32 + x_offset;
                            let y_pos = y as i32 + y_offset;
                            if x_pos < 0 || x_pos >= GRID_SIZE as i32 || y_pos < 0 || y_pos >= GRID_SIZE as i32 {
                                continue;
                            }
                            if tiles[x_pos as usize][y_pos as usize] == TileType::Wall {
                                wall_count += 1;
                            }
                        }
                    }
                    if tiles[x][y] == TileType::Wall {
                        if wall_count >= 4 {
                            new_tiles[x][y] = TileType::Wall;
                        } else {
                            new_tiles[x][y] = TileType::Floor;
                        }
                    } else {
                        if wall_count >= 5 {
                            new_tiles[x][y] = TileType::Wall;
                        } else {
                            new_tiles[x][y] = TileType::Floor;
                        }
                    }
                }
            }
            tiles = new_tiles;
        }
        Self { tiles: tiles }
    }

    pub fn new_room_based_grid(
        room_count : u32, 
        min_room_size : (usize, usize), 
        max_room_size: (usize, usize),
        ) -> Self {
        //first, we need to create a grid of tiles
        let mut tiles = [[TileType::Wall; GRID_SIZE]; GRID_SIZE];
        //then, we need to create a list of rooms, which are tuples of tuples 
        let mut rooms : Vec<((usize, usize),(usize, usize))> = Vec::new();
        //randomly place rooms, making sure they don't overlap
        let mut rng = rand::thread_rng();
        let mut rooms_planned = 0;
        let mut attempts = 0;
        while rooms_planned < room_count && attempts < 1000 {
            let room_width = rng.gen_range(min_room_size.0..max_room_size.0);
            let room_height = rng.gen_range(min_room_size.1..max_room_size.1);
            let room_x = rng.gen_range(0..GRID_SIZE - room_width);
            let room_y = rng.gen_range(0..GRID_SIZE - room_height);
            let mut room_overlaps = false;
            for room in &rooms {
                if room_x < room.0.0 + room.1.0 && room_x + room_width > room.0.0 && room_y < room.0.1 + room.1.1 && room_y + room_height > room.0.1 {
                    room_overlaps = true;
                    break;
                }
            }
            if !room_overlaps {
                rooms.push(((room_x, room_y), (room_width, room_height)));
                rooms_planned += 1;
            }
            attempts += 1;
        }
        //then, we need to iterate over the rooms and place them in the grid
        for room in &rooms {
            for x in room.0.0..room.0.0 + room.1.0 {
                for y in room.0.1..room.0.1 + room.1.1 {
                    tiles[x][y] = TileType::Floor;
                }
            }
        }
        //next create an spanning tree of the rooms, starting with a random room
        //stree is a list of edges (room1, room2), where room1 and room2 are indices into the rooms list
        let mut stree = Vec::new();
        let mut visited = vec![false; rooms.len()];
        let mut queue = Vec::new();
        queue.push(0);
        //shuffle the rooms list
        rooms.shuffle(&mut rng);
        //crate the spanning tree
        while queue.len() > 0 {
            let current = queue.pop().unwrap();
            visited[current] = true;
            for i in 0..rooms.len() {
                if i == current {
                    continue;
                }
                else if visited[i] {
                    continue;
                } else {
                    queue.push(i);
                    stree.push((current, i));
                    break;
                }
            }
        }
        //finally, iterate over the spanning tree and create hallways between the rooms in the tree.
        //The target should be a random point in the room, and the source should be a random point in the other room
        //go sideways first, then up or down   
        for edge in &stree {
            let room1 = &rooms[edge.0];
            let room2 = &rooms[edge.1];
            let mut rng = rand::thread_rng();
            let mut room1_target = (rng.gen_range(room1.0.0..room1.0.0 + room1.1.0), rng.gen_range(room1.0.1..room1.0.1 + room1.1.1));
            let mut room2_target = (rng.gen_range(room2.0.0..room2.0.0 + room2.1.0), rng.gen_range(room2.0.1..room2.0.1 + room2.1.1));
            //go sideways first
            if room1_target.0 < room2_target.0 {
                for x in room1_target.0..room2_target.0 {
                    tiles[x][room1_target.1] = TileType::Floor;
                }
            } else {
                for x in room2_target.0..room1_target.0 {
                    tiles[x][room1_target.1] = TileType::Floor;
                }
            }
            //make sure the corner is a floor tile
            tiles[room1_target.0][room1_target.1] = TileType::Floor;
            //then go up or down
            if room1_target.1 < room2_target.1 {
                for y in room1_target.1..room2_target.1 {
                    tiles[room2_target.0][y] = TileType::Floor;
                }
            } else {
                for y in room2_target.1..room1_target.1 {
                    tiles[room2_target.0][y] = TileType::Floor;
                }
            }
        }
        Self { tiles: tiles }
    }


    pub fn place_stairs(&mut self) {
        //place up stairs randomly, and down stairs randomly. Replace down stairs if the up stairs are not reachable from the down stairs
        let mut rng = rand::thread_rng();
        //place up stairs
        let mut upstairs_coords = (rng.gen_range(0..GRID_SIZE), rng.gen_range(0..GRID_SIZE));
        //make sure it's on a floor tile
        while self.tiles[upstairs_coords.0][upstairs_coords.1] != TileType::Floor {
            upstairs_coords = (rng.gen_range(0..GRID_SIZE), rng.gen_range(0..GRID_SIZE));
        }
        let mut downstairs_coords = (rng.gen_range(0..GRID_SIZE), rng.gen_range(0..GRID_SIZE));
        while downstairs_coords != upstairs_coords && !self.is_a_reachable_from_b(upstairs_coords, downstairs_coords) {
            downstairs_coords = (rng.gen_range(0..GRID_SIZE), rng.gen_range(0..GRID_SIZE));
        }
        //place both stairs
        self.tiles[upstairs_coords.0][upstairs_coords.1] = TileType::UpStairs;
        self.tiles[downstairs_coords.0][downstairs_coords.1] = TileType::DownStairs;
    }
    pub fn pretty_print_grid(&self) {
        for y in 0..GRID_SIZE {
            for x in 0..GRID_SIZE {
                match self.tiles[x][y] {
                    TileType::Wall => print!("#"),
                    TileType::Floor => print!("."),
                    TileType::DownStairs => print!(">"),
                    TileType::UpStairs => print!("<"),
                    TileType::Fountain => print!("F"),
                    TileType::Water => print!(" "),
                    TileType::Lava => print!("L"),
                }
            }
            println!();
        }
    }
    pub fn is_a_reachable_from_b(&self, a : (usize, usize), b : (usize, usize)) -> bool {
        //we will use a flood fill algorithm to check if a is reachable from b, discounting walls, lava, and air
        let mut visited = [[false; GRID_SIZE]; GRID_SIZE];
        let mut queue = Vec::new();
        queue.push(b);
        while queue.len() > 0 {
            let current = queue.pop().unwrap();
            if current == a {
                return true;
            }
            visited[current.0][current.1] = true;
            for x_offset in -1..=1 {
                for y_offset in -1..=1 {
                    if x_offset == 0 && y_offset == 0 {
                        continue;
                    }
                    let x_pos = current.0 as i32 + x_offset;
                    let y_pos = current.1 as i32 + y_offset;
                    if x_pos < 0 || x_pos >= GRID_SIZE as i32 || y_pos < 0 || y_pos >= GRID_SIZE as i32 {
                        continue;
                    }
                    if visited[x_pos as usize][y_pos as usize] {
                        continue;
                    }
                    if self.tiles[x_pos as usize][y_pos as usize] == TileType::Wall || self.tiles[x_pos as usize][y_pos as usize] == TileType::Lava || self.tiles[x_pos as usize][y_pos as usize] == TileType::Water {
                        continue;
                    }
                    queue.push((x_pos as usize, y_pos as usize));
                }
            }
        }
        return false;
    }
}
mod save_data;
mod game_state;
mod actors;
mod tiles;
mod floor;
mod grid;
mod position;
mod player;
mod main_menu_plugin;
mod volume;
mod game_world;
mod display_quality;
mod game_plugin;
mod util;
mod creature;
use bevy::prelude::*;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn main() {
    //create a grid
    let mut grid = grid::Grid::new_room_based_grid(8, (5,5),(20, 10));
    //let mut grid = grid::Grid::new_cell_automata_grid(0.5, 5);
    grid.place_stairs();
    grid.pretty_print_grid();
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(display_quality::DisplayQuality::High)
        .insert_resource(volume::Volume(5))
        .add_systems(Startup, setup)
        .add_plugins(main_menu_plugin::menu::MenuPlugin)
        .add_plugin(game_plugin::game::GamePlugin)
        .add_state::<game_state::GameState>()
        .run()
    ;
}

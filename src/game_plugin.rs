pub mod game {

    use crate::actors::Actor;
    use crate::game_world::GameWorld;
    use crate::grid::GRID_SIZE;
    use crate::player::Player;
    use crate::position::{Position, Velocity};
    use crate::tiles::Tile;
    use bevy::input::mouse::MouseMotion;
    use bevy::window::PrimaryWindow;
    use bevy::{prelude::*, sprite};
    pub struct GamePlugin;
    use crate::creature::*;
    use crate::game_state::GameState;
    use crate::util::despawn_screen;

    #[derive(Event)]
    pub struct MovementEvent {
        pub actor: Entity,
        pub new_pos: Position,
    }

    #[derive(Event)]
    pub struct RenderGrid;

    pub fn setup(mut commands: Commands) {
        //reset the camera
        //commands.spawn(Camera2dBundle::default());
    }
    pub fn set_player_camera(
        mut commands: Commands,
        player: Query<(&Player, &Position, &Sprite)>,
        mut camera: Query<&mut Transform, With<Camera>>,
    ) {
        //set the camera to the player position
        let player_pos = player.iter().next().unwrap().1;
        let mut camera_transform = camera.iter_mut().next().unwrap();
        camera_transform.translation.x = player_pos.0;
        camera_transform.translation.y = player_pos.1;
    }
    //update the actors' transforms based on their positions.
    pub fn update_actors_transforms(
        mut commands: Commands,
        mut actors: Query<(Entity, &Position, &Sprite, &mut Transform)>,
        mut player: Query<(&Player, &Position, &Sprite)>,
    ) {
        for (_, pos, _, mut transform) in actors.iter_mut() {
            transform.translation.x = pos.0;
            transform.translation.y = pos.1;
            //set invisible if not on the same level as the player
            let player_pos = player.iter().next().unwrap().1;
            println!("player level: {}, actor level: {}", player_pos.2, pos.2);
            if player_pos.2 != pos.2 {
                transform.scale = Vec3::new(0.0, 0.0, 0.0);
            } else {
                transform.scale = Vec3::new(1.0, 1.0, 1.0);
            }
        }
    }


    fn control_player(
        keys: Res<Input<KeyCode>>,
        mut movement_event_writer: EventWriter<MovementEvent>,
        mut player: Query<(Entity, &Player, &mut Velocity)>,
        time: Res<Time>,
        grid: Res<GameWorld>,
    ) {
        let mut new_velocity =  player.iter_mut().next().unwrap().2.clone();
        if keys.pressed(KeyCode::W) {
            new_velocity.1 += 100.0 * time.delta_seconds();
        }
        if keys.pressed(KeyCode::S) {
            new_velocity.1 -= 100.0 * time.delta_seconds();
        }
        if keys.pressed(KeyCode::A) {
            new_velocity.0 -= 100.0 * time.delta_seconds();
        }
        if keys.pressed(KeyCode::D) {
            new_velocity.0 += 100.0 * time.delta_seconds();
        }
        //set the player's velocity
        for (_, _, mut velocity) in player.iter_mut() {
            velocity.0 = new_velocity.0;
            velocity.1 = new_velocity.1;
        }

    }

    fn handle_movement_event_with_collisions(
        mut event_reader: EventReader<MovementEvent>,
        mut actors: Query<(Entity, &mut Position, &Actor, Option<&mut Velocity>)>,
        game_world: Res<GameWorld>,
    ) {
        for event in event_reader.iter() {
            let mut actor_pos = actors.get_mut(event.actor).unwrap().1;
            let mut x = 0;
            let mut y = 0;
            let mut was_colliding = None;
            for row in game_world.grids.get(actor_pos.2).unwrap().tiles {
                for tile_type in row {
                    if !tile_type.is_walkable() {
                        let box_size = Vec2::new(32.0, 32.0);
                        let tile_pos = Vec2::new(x as f32 * 32.0, y as f32 * 32.0);
                        if let Some(collision) = sprite::collide_aabb::collide(
                            Vec3::new(event.new_pos.0 , event.new_pos.1, 0.0),
                            box_size,
                            Vec3::new(tile_pos.x, tile_pos.y, 0.0),
                            box_size,
                        ) {
                            was_colliding = Some(collision);
                            //set velocity to -1.0 * velocity
                            
                            break;
                        }
                    }
                    x += 1;
                }
                x = 0;
                y += 1;
            }
            //check if out of bounds
            if event.new_pos.0 < 0.0
                || event.new_pos.0 > 32.0 * GRID_SIZE as f32
                || event.new_pos.1 < 0.0
                || event.new_pos.1 > 32.0 * GRID_SIZE as f32
            {
                was_colliding = Some(sprite::collide_aabb::Collision::Inside);
            }
            if was_colliding.is_none() {
                println!("moving actor");
                actor_pos.0 = event.new_pos.0;
                actor_pos.1 = event.new_pos.1;
            } else if  let Some(mut vel) = actors.get_mut(event.actor).unwrap().3 {
                                match was_colliding.unwrap() {
                                    sprite::collide_aabb::Collision::Left => {
                                        vel.0 = -1.0 * vel.0;
                                    }
                                    sprite::collide_aabb::Collision::Right => {
                                        vel.0 = -1.0 * vel.0;
                                    }
                                    sprite::collide_aabb::Collision::Top => {
                                        vel.1 = -1.0 * vel.1;
                                    }
                                    sprite::collide_aabb::Collision::Bottom => {
                                        vel.1 = -1.0 * vel.1;
                                    }
                                    _ => {
                                        vel.0 = -1.0 * vel.0;
                                        vel.1 = -1.0 * vel.1;
                                    }
                                }

            }
        }
    }

    pub fn handle_velocity(
        mut actors: Query<(Entity, & Position, &Actor, &Velocity)>,
        mut movement_event_writer: EventWriter<MovementEvent>,
        time: Res<Time>,
    ) {
        for (e, pos, actor, vel) in actors.iter_mut() {
            let mut new_pos = pos.clone();
            new_pos.0 += vel.0 * time.delta_seconds();
            new_pos.1 += vel.1 * time.delta_seconds();
            movement_event_writer.send(MovementEvent {
                actor: e,
                new_pos: new_pos,
            });
        }
        
    }

    pub fn rotate_player_sprite_based_on_mouse(
        mut player: Query<(&Player, &mut Transform)>,
        q_windows: Query<&Window, With<PrimaryWindow>>,
        ) {
            if let Some(position) = q_windows.single().cursor_position() {
                let player_pos = player.iter_mut().next().unwrap().1.clone();
                let mut player_transform = player.iter_mut().next().unwrap().1;
                let center_of_screen = Vec2::new(q_windows.single().width() / 2.0, q_windows.single().height() / 2.0);
                let mouse_pos = Vec2::new(position.x, position.y) - center_of_screen;
                println!("mouse pos: {:?}", mouse_pos);
                let direction = mouse_pos;
                //normalize the direction
                let direction = direction.normalize();
                let angle = direction.y.atan2(direction.x);
                player_transform.rotation = Quat::from_rotation_z(-angle);
            } else {
                println!("Cursor is not in the game window.");
            }
    }

    pub fn render_grid(
        mut ev_render_grid: EventReader<RenderGrid>,
        mut commands: Commands,
        player: Query<(&Player, &Position)>,
        tiles: Query<(&Tile, Option<&Sprite>, Entity)>,
        mut camera: Query<&mut Transform, With<Camera>>,
        game_world: Res<GameWorld>,
        asset_server: Res<AssetServer>,
    ) {
        //set the camera to the player position
        let player_pos = player.iter().next().unwrap().1;
        let mut camera_transform = camera.iter_mut().next().unwrap();
        camera_transform.translation.x = player_pos.0;
        camera_transform.translation.y = player_pos.1;

        //despawn the tiles
        for (_, _, e) in tiles.iter() {
            commands.entity(e).despawn_recursive();
        }
        //draw the tiles
        let mut x = 0;
        let mut y = 0;
        let level = player.iter().next().unwrap().1 .2;
        for row in game_world.grids.get(level).unwrap().tiles {
            for tile_type in row {
                let tile = Tile {
                    tile_type: tile_type,
                    x: x,
                    y: y,
                    level: level,
                };
                let texture_handle = asset_server.load(tile.tile_type.get_texture_str());
                commands.spawn((
                    SpriteBundle {
                        texture: texture_handle.into(),
                        transform: Transform::from_xyz((x as f32 * 32.0), (y as f32 * 32.0), -1.0),
                        ..Default::default()
                    },
                    tile,
                ));

                x += 1;
            }
            x = 0;
            y += 1;
        }
    }
    pub fn send_render_grid_event(mut ev_render_grid: EventWriter<RenderGrid>) {
        ev_render_grid.send(RenderGrid {});
    }
    impl Plugin for GamePlugin {
        fn build(&self, app: &mut App) {
            app.add_systems(Startup, setup);
            let asset_server = app.world.get_resource::<AssetServer>().unwrap().clone();

            let mut game_world = GameWorld::new();
            //generate the first level
            let mut grid = crate::grid::Grid::new_cell_automata_grid(0.5, 5);
            grid.place_stairs();
            //find the player spawn point on the up stairs
            let mut player_spawn = (0, 0);
            for (y, row) in grid.tiles.iter().enumerate() {
                for (x, tile) in row.iter().enumerate() {
                    if *tile == crate::tiles::TileType::UpStairs {
                        player_spawn = (x, y);
                    }
                }
            }
            game_world.grids.push(grid);
            let pid = app
                .world
                .spawn((
                    CreatureBundle {
                        position: Position(
                            player_spawn.0 as f32 * 32.0,
                            player_spawn.1 as f32 * 32.0,
                            0,
                        ),
                        sp_bundle: SpriteBundle {
                            texture: asset_server.load(CreatureType::Human.get_stats().sprite),
                            transform: Transform::from_xyz(
                                (player_spawn.0 as f32 * 32.0),
                                (player_spawn.1 as f32 * 32.0),
                                1.0,
                            ),
                            visibility: Visibility::Visible,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    Player {
                        ..Default::default()
                    },
                    Velocity(0.0, 0.0),
                ))
                .id();
            //insert the player into the game world
            game_world.player = Some(pid);
            app.world.insert_resource(game_world);
            //ambient light
            app.insert_resource(AmbientLight {
                color: Color::rgb(0.0, 0.5, 0.0),
                brightness: 1.0,
                ..Default::default()
            });
            //render the grid
            app.add_event::<RenderGrid>();
            app.add_event::<MovementEvent>();
            app.add_systems(Update, render_grid.run_if(on_event::<RenderGrid>()));
            app.add_systems(
                Update,
                update_actors_transforms.run_if(in_state(GameState::Playing)),
            );
            app.add_systems(Update, control_player.run_if(in_state(GameState::Playing)));
            app.add_systems(Update, rotate_player_sprite_based_on_mouse.run_if(in_state(GameState::Playing)));
            app.add_systems(
                Update,
                handle_movement_event_with_collisions.run_if(in_state(GameState::Playing)),
            );
            app.add_systems(Update, handle_velocity.run_if(in_state(GameState::Playing)));
            app.add_systems(
                Update,
                set_player_camera.run_if(in_state(GameState::Playing)),
            );
            //despawn tiles OnExit(Playing)
            app.add_systems(OnEnter(GameState::Playing), setup);
            app.add_systems(OnExit(GameState::Playing), despawn_screen::<Tile>);
            app.add_systems(OnExit(GameState::Playing), despawn_screen::<Actor>);
            //spawn a render grid event on enter playing
            app.add_systems(OnEnter(GameState::Playing), send_render_grid_event);
        }
    }
}

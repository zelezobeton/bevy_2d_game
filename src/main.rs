/*
TODO:
- Break down rocks
- Chop down trees

DONE:
- Animate character
- Show grid cursor
- Make grid
- Let camera follow the player
- Add colisions
*/

use rand::Rng;

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;

mod player;
use player::Player;

#[derive(Component)]
struct Grid {
    tile_size: f32,
    placements: Vec<(String, (i32, i32))>,
}

impl Grid {
    fn new(tile_size: f32) -> Self {
        Self {
            tile_size,
            placements: vec![],
        }
    }

    fn place_object(&mut self, name: String, pos: (i32, i32)) {
        self.placements.push((name, pos));
    }

    fn is_free(&self, pos: (i32, i32)) -> bool {
        let mut result = true;
        for placement in self.placements.iter() {
            let (_string, tuple) = placement;
            if *tuple == pos {
                result = false;
                break;
            }
        }
        result
    }

    fn world_to_grid(&self, pos: Vec2) -> (i32, i32) {
        let a = (pos.x / self.tile_size) as i32;
        let b = (pos.y / self.tile_size) as i32;
        (a, b)
    }

    fn grid_to_world(&self, pos: (i32, i32)) -> Vec2 {
        let a = pos.0 as f32 * self.tile_size + (self.tile_size / 2.0);
        let b = pos.1 as f32 * self.tile_size + (self.tile_size / 2.0);
        Vec2::new(a, b)
    }

    fn cursor_to_world(&self, pos: Vec2) -> Vec2 {
        let a = (pos.x / self.tile_size).floor() * self.tile_size + (self.tile_size / 2.0);
        let b = (pos.y / self.tile_size).floor() * self.tile_size + (self.tile_size / 2.0);
        Vec2::new(a, b)
    }
}

#[derive(Component)]
struct MainCamera;

#[derive(Component)]
struct Cursor;

#[derive(Component)]
struct Tree;

#[derive(Component)]
struct Damage {
    value: i32,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.5, 0.0)))
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Anarchy".into(),
                        resolution: (640.0, 480.0).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
            RapierDebugRenderPlugin::default(),
            player::PlayerPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(PostStartup, (spawn_trees, spawn_rocks))
        .add_systems(Update, (move_camera, move_cursor, chop_tree))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default()).insert(MainCamera);

    commands.spawn(Grid::new(50.0));

    // Setup cursor
    commands
        .spawn(SceneBundle {
            transform: Transform {
                translation: Vec3::ZERO,
                ..default()
            },
            ..default()
        })
        .insert(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0., 0.25, 0.),
                custom_size: Some(Vec2::new(50.0, 50.0)),
                ..default()
            },
            ..default()
        })
        .insert(Cursor);
}

fn move_cursor(
    primary_query: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), (With<MainCamera>, Without<Player>)>,
    mut cursor_transform: Query<&mut Transform, With<Cursor>>,
    grid_query: Query<&Grid>,
) {
    let (camera, camera_transform) = q_camera.single();
    let Ok(primary) = primary_query.get_single() else {
        return;
    };
    if let Some(screen_pos) = primary.cursor_position() {
        let pos = camera
            .viewport_to_world(camera_transform, screen_pos)
            .and_then(|ray| {
                ray.intersect_plane(Vec3::ZERO, Vec3::Z)
                    .map(|distance| ray.get_point(distance))
            });

        let grid = grid_query.single();
        let pos2 = grid.cursor_to_world(pos.unwrap().truncate());
        cursor_transform.single_mut().translation = Vec3::new(pos2.x, pos2.y, 0.);
    }
}

fn move_camera(
    mut camera_transform: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
    mut player_transform: Query<&Transform, (With<Player>, Without<MainCamera>)>,
) {
    let player_pos = player_transform.single_mut().translation;
    let new_camera_pos = player_pos;

    // Interpolated camera movement
    camera_transform.single_mut().translation = camera_transform
        .single_mut()
        .translation
        .lerp(new_camera_pos, 0.2);
}

fn chop_tree(
    player_query: Query<&Transform, With<Player>>,
    mut tree_query: Query<(Entity, &Transform, &mut Damage), With<Tree>>,
    input: Res<Input<KeyCode>>,
    mut commands: Commands,
) {
    if input.just_pressed(KeyCode::Space) {
        let player = player_query.single();

        // Find nearest tree
        let mut nearest_entity: Option<(Entity, f32)> = None;
        for (entity, transform, _) in tree_query.iter() {
            match nearest_entity {
                None => {
                    let distance = transform.translation.distance(player.translation);
                    nearest_entity = Some((entity, distance));
                },
                Some((_, distance2)) => {
                    let distance = transform.translation.distance(player.translation);
                    if distance < distance2 {
                        nearest_entity = Some((entity, distance));
                    }
                }
            }
        }

        // Chop down nearest tree
        for (entity, _, mut damage) in tree_query.iter_mut() {
            match nearest_entity {
                Some((entity2, distance2)) => {
                    if entity == entity2 {
                        if distance2 < 80.0 {
                            damage.value -= 1;
                        }
                        if damage.value == 0 {
                            commands.entity(entity).despawn_recursive();
                        }
                    }
                },
                None => {}
            }
        }
    }
}

fn spawn_trees(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut grid_query: Query<&mut Grid>,
) {
    let mut rng = rand::thread_rng();
    let mut trees_num = 0;
    loop {
        let x = rng.gen_range(-7..=7);
        let y = rng.gen_range(-7..=7);
        let mut grid = grid_query.single_mut();
        if grid.is_free((x, y)) {
            grid.place_object("tree".to_string(), (x, y));
            let v = grid.grid_to_world((x, y));
            let texture = asset_server.load("tree.png");
            commands.spawn((
                SpriteBundle {
                    texture,
                    transform: Transform::from_xyz(v.x, v.y, 0.0),
                    ..default()
                },
                Collider::ball(25.0),
                Tree,
                Damage { value: 3 },
            ));
            trees_num += 1
        } else {
            continue;
        }
        if trees_num == 10 {
            break;
        }
    }
}

fn spawn_rocks(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut grid_query: Query<&mut Grid>,
) {
    let mut rng = rand::thread_rng();
    let mut rocks_num = 0;
    loop {
        let x = rng.gen_range(-7..=7);
        let y = rng.gen_range(-7..=7);
        let mut grid = grid_query.single_mut();
        if grid.is_free((x, y)) {
            grid.place_object("rock".to_string(), (x, y));
            let v = grid.grid_to_world((x, y));
            let texture = asset_server.load("rock.png");
            commands.spawn((
                SpriteBundle {
                    texture,
                    transform: Transform::from_xyz(v.x, v.y, 0.0),
                    ..default()
                },
                Collider::ball(25.0),
            ));
            rocks_num += 1
        } else {
            continue;
        }
        if rocks_num == 10 {
            break;
        }
    }
}

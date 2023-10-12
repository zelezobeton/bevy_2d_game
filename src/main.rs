/*
TODO:
- Animate character

DONE:
- Show grid cursor
- Make grid
- Let camera follow the player
- Add colisions
*/

use rand::Rng;

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;

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

    fn cursor_to_world(&self, pos: Vec2) -> Vec2{
        let a = (pos.x / self.tile_size).floor() * self.tile_size + (self.tile_size / 2.0);
        let b = (pos.y / self.tile_size).floor() * self.tile_size + (self.tile_size / 2.0);
        Vec2::new(a, b)
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
struct MainCamera;

#[derive(Component)]
struct Cursor;

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
        ))
        .add_systems(Startup, setup)
        .add_systems(PostStartup, (spawn_trees, spawn_rocks))
        .add_systems(Update, (character_movement, move_camera, move_cursor))
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands.spawn(Camera2dBundle::default()).insert(MainCamera);

    let texture = asset_server.load("punk.png");

    commands
        .spawn((
            SpriteBundle {
                texture,
                ..default()
            },
            Player,
        ))
        .insert(KinematicCharacterController::default())
        .insert(RigidBody::KinematicVelocityBased)
        .insert(Collider::capsule_y(25.0, 25.0));

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
    grid_query: Query<& Grid>,
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

fn character_movement(
    mut controller_query: Query<&mut KinematicCharacterController, With<Player>>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let SPEED = 150.0;
    let mut controller = controller_query.single_mut();
    if input.pressed(KeyCode::W) {
        controller.translation = Some(Vec2::new(0.0, SPEED * time.delta_seconds()));
    }
    if input.pressed(KeyCode::S) {
        controller.translation = Some(Vec2::new(0.0, -SPEED * time.delta_seconds()));
    }
    if input.pressed(KeyCode::D) {
        controller.translation = Some(Vec2::new(SPEED * time.delta_seconds(), 0.0));
    }
    if input.pressed(KeyCode::A) {
        controller.translation = Some(Vec2::new(-SPEED * time.delta_seconds(), 0.0));
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
        if trees_num == 10 {
            break;
        }
        let x = rng.gen_range(-7..=7);
        let y = rng.gen_range(-7..=7);
        let mut grid = grid_query.single_mut();
        if grid.is_free((x, y)) {
            grid.place_object("tree".to_string(), (x, y));
            let v = grid.grid_to_world((x, y));
            let texture = asset_server.load("tree.png");
            commands
                .spawn(SpriteBundle {
                    texture,
                    transform: Transform::from_xyz(v.x, v.y, 0.0),
                    ..default()
                })
                .insert(Collider::capsule_y(25.0, 25.0));
            trees_num += 1
        } else {
            continue;
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
        if rocks_num == 10 {
            break;
        }
        let x = rng.gen_range(-7..=7);
        let y = rng.gen_range(-7..=7);
        let mut grid = grid_query.single_mut();
        if grid.is_free((x, y)) {
            grid.place_object("rock".to_string(), (x, y));
            let v = grid.grid_to_world((x, y));
            let texture = asset_server.load("rock.png");
            commands
                .spawn(SpriteBundle {
                    texture,
                    transform: Transform::from_xyz(v.x, v.y, 0.0),
                    ..default()
                })
                .insert(Collider::ball(25.0));
            rocks_num += 1
        } else {
            continue;
        }
    }
}

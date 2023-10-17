/*
TODO:
- Make store
- Build houses wall by wall and walkable
- Grow plants

DONE:
- Add buildable cottage
- Make UI
- Break down rocks
- Chop down trees
- Animate character
- Show grid cursor
- Make grid
- Let camera follow the player
- Add colisions
*/
use std::collections::HashMap;

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;

mod player;
mod ui;
use player::{AnimationIndices, Movement, Player};
use ui::HouseButton;

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

    fn place_object(&mut self, name: String, pos: Vec2) {
        self.placements.push((name, self.world_to_grid(pos)));
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
pub struct Cursor;

#[derive(Component, PartialEq)]
enum WorldObject {
    Tree,
    Rock,
}

#[derive(Component, PartialEq, Eq, Hash, Debug, Clone, Copy)]
enum PickableObject {
    Wood,
    Rocks,
}

#[derive(Component, PartialEq, Eq, Hash, Debug, Clone, Copy)]
enum Tool {
    Axe,
    Pickaxe,
}

#[derive(Component, PartialEq, Eq, Hash, Debug, Clone, Copy)]
enum Recipe {
    House,
}

#[derive(Component)]
struct Damage {
    value: i32,
}

#[derive(Component)]
pub struct Inventory {
    items: HashMap<PickableObject, i32>,
    tools: HashMap<Tool, bool>,
    recipes: HashMap<Recipe, bool>,
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
                        resolution: (1280.0, 720.0).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
            // RapierDebugRenderPlugin::default(),
            player::PlayerPlugin,
            ui::UiPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(PostStartup, (spawn_trees, spawn_rocks))
        .add_systems(
            Update,
            (
                move_camera,
                break_object,
                pickup_object,
                move_cursor,
                drop_house,
            ),
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default()).insert(MainCamera);

    commands.spawn(Grid::new(50.0));

    commands.spawn(Inventory {
        items: HashMap::from([(PickableObject::Wood, 0), (PickableObject::Rocks, 0)]),
        tools: HashMap::from([(Tool::Axe, false), (Tool::Pickaxe, false)]),
        recipes: HashMap::from([(Recipe::House, false)]),
    });

    // Setup cursor
    commands
        .spawn(SceneBundle {
            transform: Transform {
                translation: Vec3::ZERO,
                ..default()
            },
            ..default()
        })
        // .insert(SpriteBundle {
        //     sprite: Sprite {
        //         color: Color::rgb(0., 0.25, 0.),
        //         custom_size: Some(Vec2::new(50.0, 50.0)),
        //         ..default()
        //     },
        //     ..default()
        // })
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

fn drop_house(
    cursor: Query<(Entity, &Transform), With<Cursor>>,
    mut commands: Commands,
    mouse: Res<Input<MouseButton>>,
    query: Query<&Sprite>,
    asset_server: Res<AssetServer>,
    mut inv_query: Query<&mut Inventory>,
    text_query: Query<Entity, With<HouseButton>>,
) {
    if query.contains(cursor.single().0) {
        if mouse.just_pressed(MouseButton::Left) {
            commands.entity(cursor.single().0).remove::<Sprite>();

            let texture = asset_server.load("cottage.png");
            commands.spawn((
                SpriteBundle {
                    transform: *cursor.single().1,
                    texture,
                    ..default()
                },
                Collider::cuboid(95.0, 95.0),
            ));

            inv_query
                .single_mut()
                .items
                .entry(PickableObject::Rocks)
                .and_modify(|count| *count -= 5);

            inv_query
                .single_mut()
                .items
                .entry(PickableObject::Wood)
                .and_modify(|count| *count -= 5);

            commands.entity(text_query.single()).despawn_descendants();
        }
    }
}

fn pickup_object(
    rapier_context: Res<RapierContext>,
    object_query: Query<(Entity, &Transform, &PickableObject)>,
    mut inv_query: Query<&mut Inventory>,
    mut commands: Commands,
) {
    for (entity, transform, object) in object_query.iter() {
        let shape = Collider::cuboid(1.0, 2.0);
        let shape_pos = transform.translation.truncate();
        let shape_rot = 0.0;
        let shape_vel = Vec2::new(0.1, 0.1);
        let max_toi = 0.0;
        let filter = QueryFilter::default();

        if let Some((_entity, _hit)) =
            rapier_context.cast_shape(shape_pos, shape_rot, shape_vel, &shape, max_toi, filter)
        {
            inv_query
                .single_mut()
                .items
                .entry(*object)
                .and_modify(|count| *count += 1);
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn break_object(
    player_query: Query<&Transform, With<Player>>,
    mut object_query: Query<(Entity, &Transform, &mut Damage, &WorldObject)>,
    input: Res<Input<KeyCode>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    inv_query: Query<&Inventory>,
    mut anim_query: Query<
        (
            &mut AnimationIndices,
            &mut TextureAtlasSprite,
            &mut Movement,
        ),
        With<Player>,
    >,
) {
    if input.just_pressed(KeyCode::Space) {
        // Find nearest object
        let player = player_query.single();
        let mut nearest_entity: Option<(Entity, f32)> = None;
        for (entity, transform, _, _) in object_query.iter() {
            match nearest_entity {
                None => {
                    let distance = transform.translation.distance(player.translation);
                    nearest_entity = Some((entity, distance));
                }
                Some((_, distance2)) => {
                    let distance = transform.translation.distance(player.translation);
                    if distance < distance2 {
                        nearest_entity = Some((entity, distance));
                    }
                }
            }
        }

        // Break down nearest object
        for (entity, transform, mut damage, object) in object_query.iter_mut() {
            match nearest_entity {
                Some((entity2, distance2)) => {
                    if entity == entity2 {
                        if distance2 < 80.0 {
                            let (mut anim_indices, mut sprite, mut movement) =
                                anim_query.single_mut();
                            if object == &WorldObject::Tree && inv_query.single().tools[&Tool::Axe]
                            {
                                damage.value -= 1;
                                match *movement {
                                    Movement::Up => {
                                        anim_indices.first = 19;
                                        anim_indices.last = 19;
                                        *sprite = TextureAtlasSprite::new(19);
                                    }
                                    Movement::Down => {
                                        anim_indices.first = 16;
                                        anim_indices.last = 16;
                                        *sprite = TextureAtlasSprite::new(16);
                                    }
                                    Movement::Left => {
                                        anim_indices.first = 17;
                                        anim_indices.last = 17;
                                        *sprite = TextureAtlasSprite::new(17);
                                    }
                                    Movement::Right => {
                                        anim_indices.first = 18;
                                        anim_indices.last = 18;
                                        *sprite = TextureAtlasSprite::new(18);
                                    }
                                    Movement::Working => {}
                                    Movement::None => {
                                        anim_indices.first = 16;
                                        anim_indices.last = 16;
                                        *sprite = TextureAtlasSprite::new(16);
                                    }
                                }
                                *movement = Movement::Working;
                            }
                            if object == &WorldObject::Rock
                                && inv_query.single().tools[&Tool::Pickaxe]
                            {
                                damage.value -= 1;
                                match *movement {
                                    Movement::Up => {
                                        anim_indices.first = 19;
                                        anim_indices.last = 19;
                                        *sprite = TextureAtlasSprite::new(19);
                                    }
                                    Movement::Down => {
                                        anim_indices.first = 20;
                                        anim_indices.last = 20;
                                        *sprite = TextureAtlasSprite::new(20);
                                    }
                                    Movement::Left => {
                                        anim_indices.first = 21;
                                        anim_indices.last = 21;
                                        *sprite = TextureAtlasSprite::new(21);
                                    }
                                    Movement::Right => {
                                        anim_indices.first = 22;
                                        anim_indices.last = 22;
                                        *sprite = TextureAtlasSprite::new(22);
                                    }
                                    Movement::Working => {}
                                    Movement::None => {
                                        anim_indices.first = 20;
                                        anim_indices.last = 20;
                                        *sprite = TextureAtlasSprite::new(20);
                                    }
                                }
                                *movement = Movement::Working;
                            }
                        }

                        if damage.value == 0 {
                            commands.entity(entity).despawn_recursive();

                            // Spawn wood
                            if *object == WorldObject::Tree {
                                let texture = asset_server.load("wood.png");
                                commands.spawn((
                                    SpriteBundle {
                                        texture,
                                        transform: *transform,
                                        ..default()
                                    },
                                    PickableObject::Wood,
                                ));
                            }

                            // Spawn rocks
                            if *object == WorldObject::Rock {
                                let texture = asset_server.load("rocks.png");
                                commands.spawn((
                                    SpriteBundle {
                                        texture,
                                        transform: *transform,
                                        ..default()
                                    },
                                    PickableObject::Rocks,
                                ));
                            }
                        }
                    }
                }
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
    let positions = [
        [200.0, 150.0],
        [350.0, 0.0],
        [350.0, 200.0],
        [-250.0, 100.0],
        [-150.0, 150.0],
        [300.0, -250.0],
        [400.0, -150.0],
        [-200.0, -100.0],
        [-400.0, -250.0],
        [-100.0, -300.0],
    ];
    for pos in positions {
        let mut grid = grid_query.single_mut();
        grid.place_object("tree".to_string(), Vec2::new(pos[0], pos[1]));
        let texture = asset_server.load("tree.png");
        commands.spawn((
            SpriteBundle {
                texture,
                transform: Transform::from_xyz(pos[0], pos[1], 0.0),
                ..default()
            },
            Collider::ball(25.0),
            WorldObject::Tree,
            Damage { value: 3 },
        ));
    }
}

fn spawn_rocks(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut grid_query: Query<&mut Grid>,
) {
    let positions = [
        [50.0, 50.0],
        [300.0, 350.0],
        [200.0, 250.0],
        [-150.0, 300.0],
        [-400.0, 100.0],
        [250.0, -150.0],
        [50.0, -250.0],
        [-300.0, -150.0],
        [-250.0, -400.0],
        [-450.0, -50.0],
    ];
    for pos in positions {
        let mut grid = grid_query.single_mut();
        grid.place_object("rock".to_string(), Vec2::new(pos[0], pos[1]));
        let texture = asset_server.load("rock.png");
        commands.spawn((
            SpriteBundle {
                texture,
                transform: Transform::from_xyz(pos[0], pos[1], 0.0),
                ..default()
            },
            Collider::ball(25.0),
            WorldObject::Rock,
            Damage { value: 2 },
        ));
    }
}
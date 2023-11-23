/*
TODO:
- Grow plants

DONE:
- Make seed buttons clickable
- Make store
- Build houses wall by wall and walkable
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
use std::time::Instant;

use rand::Rng;

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;

mod hud_ui;
mod menu_ui;
mod player;
mod store_ui;
use hud_ui::{House, OnCursor};
use player::{AnimationIndices, Movement, Player};

#[derive(Component)]
pub struct Grid {
    tile_size: f32,
    placements: Vec<(Entity, (i32, i32), WorldObject)>,
}

impl Grid {
    fn new(tile_size: f32) -> Self {
        Self {
            tile_size,
            placements: vec![],
        }
    }

    fn remove_object(&mut self, pos: Vec2) {
        let mut index: Option<usize> = None;
        for (index2, (_, pos2, _)) in self.placements.iter().enumerate() {
            if self.world_to_grid(pos) == *pos2 {
                index = Some(index2);
            }
        }
        match index {
            Some(idx) => {
                self.placements.remove(idx);
            }
            None => {}
        }
    }

    fn get_object(&self, pos: Vec2) -> Option<(Entity, (i32, i32), WorldObject)> {
        for (entity, pos2, object) in self.placements.iter() {
            if self.world_to_grid(pos) == *pos2 {
                return Some((*entity, *pos2, *object));
            }
        }
        return None;
    }

    fn place_object(&mut self, entity: Entity, pos: Vec2, object: WorldObject) {
        self.placements
            .push((entity, self.world_to_grid(pos), object));
    }

    fn is_free(&self, pos: Vec2) -> bool {
        let mut result = true;
        for placement in self.placements.iter() {
            let (_, tuple, _) = placement;
            if *tuple == self.world_to_grid(pos) {
                result = false;
                break;
            }
        }
        result
    }

    fn world_to_grid(&self, pos: Vec2) -> (i32, i32) {
        let a = (pos.x / self.tile_size).floor() as i32;
        let b = (pos.y / self.tile_size).floor() as i32;
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
pub struct Inventory {
    coins: i32,
    items: HashMap<InventoryObject, (bool, i32)>,
    recipes: HashMap<Recipe, Vec<(InventoryObject, i32)>>,
    costs: HashMap<InventoryObject, i32>,
}

impl Inventory {
    fn recipe_satisfied(&self, recipe: Recipe) -> bool {
        let mut satisfied = true;
        for (inventory_object, count) in &self.recipes[&recipe] {
            if &self.items[&inventory_object].1 < count {
                satisfied = false;
            }
        }
        return satisfied;
    }
}

#[derive(Component)]
struct MainCamera;

#[derive(Component)]
pub struct Cursor;

#[derive(Component, PartialEq, Clone, Copy)]
enum WorldObject {
    Tree,
    Rock,
    Grass,
    Flowerbed,
    FlowerbedWithPotatoSeeds,
    FlowerbedWithBeans,
}

#[derive(Component, PartialEq, Eq, Hash, Debug, Clone, Copy)]
enum InventoryObject {
    Axe,
    Pickaxe,
    Hoe,
    Wood,
    Rocks,
    Beans,
    PotatoSeeds,
}

#[derive(Component, PartialEq, Eq, Hash, Debug, Clone)]
struct Recipe(House);

#[derive(Component)]
struct Damage(i32);

#[derive(Component)]
struct YSort(f32);

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AppState {
    #[default]
    Menu,
    InGame,
    Store,
}

#[derive(Component)]
struct GrowStartTime(Instant);

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.5, 0.0)))
        .add_state::<AppState>()
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
            RapierDebugRenderPlugin::default(),
            player::PlayerPlugin,
            hud_ui::HudUiPlugin,
            menu_ui::MenuUiPlugin,
            store_ui::StoreUiPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(PostStartup, (spawn_trees, spawn_rocks, spawn_grass))
        .add_systems(
            Update,
            (
                move_camera,
                break_object,
                pickup_object,
                move_cursor,
                drop_house_parts,
                y_sort,
                dig_flowerbed,
                spread_seed,
                grow_plants
            )
                .run_if(in_state(AppState::InGame)),
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default()).insert(MainCamera);

    commands.spawn(Grid::new(50.0));

    commands.spawn(Inventory {
        coins: 100,
        items: HashMap::from([
            (InventoryObject::Axe, (false, 1)),
            (InventoryObject::Pickaxe, (false, 1)),
            (InventoryObject::Hoe, (false, 1)),
            (InventoryObject::Wood, (false, 20)),
            (InventoryObject::Rocks, (false, 20)),
            (InventoryObject::Beans, (false, 0)),
            (InventoryObject::PotatoSeeds, (false, 0)),
        ]),
        recipes: HashMap::from([
            (
                Recipe(House::Corner1),
                vec![(InventoryObject::Wood, 1), (InventoryObject::Rocks, 1)],
            ),
            (
                Recipe(House::Corner2),
                vec![(InventoryObject::Wood, 1), (InventoryObject::Rocks, 1)],
            ),
            (
                Recipe(House::Corner3),
                vec![(InventoryObject::Wood, 1), (InventoryObject::Rocks, 1)],
            ),
            (
                Recipe(House::Corner4),
                vec![(InventoryObject::Wood, 1), (InventoryObject::Rocks, 1)],
            ),
            (
                Recipe(House::Wall1),
                vec![(InventoryObject::Wood, 1), (InventoryObject::Rocks, 1)],
            ),
            (
                Recipe(House::Wall2),
                vec![(InventoryObject::Wood, 1), (InventoryObject::Rocks, 1)],
            ),
            (
                Recipe(House::Wall3),
                vec![(InventoryObject::Wood, 1), (InventoryObject::Rocks, 1)],
            ),
            (Recipe(House::Door), vec![(InventoryObject::Wood, 1)]),
        ]),
        costs: HashMap::from([
            (InventoryObject::Beans, 20),
            (InventoryObject::PotatoSeeds, 10),
        ]),
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

fn y_sort(mut query: Query<(&mut Transform, &YSort)>) {
    for (mut transform, ysort) in query.iter_mut() {
        // Might need to keep an eye on this, it can't grow smaller than -1000
        transform.translation.z = 0.001 * (-transform.translation.y + (ysort.0 / 2.0));
    }
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
        // println!("{:?}", grid.world_to_grid(pos2));
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

fn drop_house_parts(
    mut commands: Commands,
    mouse: Res<Input<MouseButton>>,
    asset_server: Res<AssetServer>,
    cursor: Query<(Entity, &Transform), With<Cursor>>,
    cursor2: Query<&OnCursor, With<Cursor>>,
    sprite_query: Query<&Sprite>,
    mut inv_query: Query<&mut Inventory>,
    button_query: Query<&House>,
) {
    if sprite_query.contains(cursor.single().0) {
        if mouse.just_pressed(MouseButton::Left) {
            commands.entity(cursor.single().0).remove::<Sprite>();

            for house_part in button_query.iter() {
                // Corner 1
                if *cursor2.single() == OnCursor(House::Corner1)
                    && *house_part == House::Corner1
                    && inv_query.single().recipe_satisfied(Recipe(House::Corner1))
                {
                    let texture = asset_server.load("corner1.png");

                    commands.spawn((
                        SpriteBundle {
                            transform: *cursor.single().1,
                            texture,
                            ..default()
                        },
                        Collider::compound(vec![
                            (Vect::new(-50.0, -25.0), 0.0, Collider::cuboid(25.0, 50.0)),
                            (Vect::new(0.0, -100.0), 0.0, Collider::cuboid(75.0, 25.0)),
                        ]),
                        YSort(250.0),
                    ));

                    inv_query
                        .single_mut()
                        .items
                        .entry(InventoryObject::Rocks)
                        .and_modify(|(_, count)| *count -= 1);

                    inv_query
                        .single_mut()
                        .items
                        .entry(InventoryObject::Wood)
                        .and_modify(|(_, count)| *count -= 1);

                    commands.entity(cursor.single().0).remove::<OnCursor>();
                }
                // Corner 2
                if *cursor2.single() == OnCursor(House::Corner2)
                    && *house_part == House::Corner2
                    && inv_query.single().recipe_satisfied(Recipe(House::Corner2))
                {
                    let texture = asset_server.load("corner2.png");

                    commands.spawn((
                        SpriteBundle {
                            transform: *cursor.single().1,
                            texture,
                            ..default()
                        },
                        Collider::compound(vec![
                            (Vect::new(-50.0, -75.0), 0.0, Collider::cuboid(25.0, 50.0)),
                            (Vect::new(0.0, 0.0), 0.0, Collider::cuboid(75.0, 25.0)),
                        ]),
                        YSort(0.0),
                    ));

                    inv_query
                        .single_mut()
                        .items
                        .entry(InventoryObject::Rocks)
                        .and_modify(|(_, count)| *count -= 1);

                    inv_query
                        .single_mut()
                        .items
                        .entry(InventoryObject::Wood)
                        .and_modify(|(_, count)| *count -= 1);

                    commands.entity(cursor.single().0).remove::<OnCursor>();
                }
                // Corner 3
                if *cursor2.single() == OnCursor(House::Corner3)
                    && *house_part == House::Corner3
                    && inv_query.single().recipe_satisfied(Recipe(House::Corner3))
                {
                    let texture = asset_server.load("corner3.png");

                    commands.spawn((
                        SpriteBundle {
                            transform: *cursor.single().1,
                            texture,
                            ..default()
                        },
                        Collider::compound(vec![
                            (Vect::new(50.0, -75.0), 0.0, Collider::cuboid(25.0, 50.0)),
                            (Vect::new(0.0, 0.0), 0.0, Collider::cuboid(75.0, 25.0)),
                        ]),
                        YSort(0.0),
                    ));

                    inv_query
                        .single_mut()
                        .items
                        .entry(InventoryObject::Rocks)
                        .and_modify(|(_, count)| *count -= 1);

                    inv_query
                        .single_mut()
                        .items
                        .entry(InventoryObject::Wood)
                        .and_modify(|(_, count)| *count -= 1);

                    commands.entity(cursor.single().0).remove::<OnCursor>();
                }
                // Corner 4
                if *cursor2.single() == OnCursor(House::Corner4)
                    && *house_part == House::Corner4
                    && inv_query.single().recipe_satisfied(Recipe(House::Corner4))
                {
                    let texture = asset_server.load("corner4.png");

                    commands.spawn((
                        SpriteBundle {
                            transform: *cursor.single().1,
                            texture,
                            ..default()
                        },
                        Collider::compound(vec![
                            (Vect::new(50.0, -25.0), 0.0, Collider::cuboid(25.0, 50.0)),
                            (Vect::new(0.0, -100.0), 0.0, Collider::cuboid(75.0, 25.0)),
                        ]),
                        YSort(250.0),
                    ));

                    inv_query
                        .single_mut()
                        .items
                        .entry(InventoryObject::Rocks)
                        .and_modify(|(_, count)| *count -= 1);

                    inv_query
                        .single_mut()
                        .items
                        .entry(InventoryObject::Wood)
                        .and_modify(|(_, count)| *count -= 1);

                    commands.entity(cursor.single().0).remove::<OnCursor>();
                }
                // Wall 1
                if *cursor2.single() == OnCursor(House::Wall1)
                    && *house_part == House::Wall1
                    && inv_query.single().recipe_satisfied(Recipe(House::Wall1))
                {
                    let texture = asset_server.load("wall1.png");
                    commands.spawn((
                        SpriteBundle {
                            transform: *cursor.single().1,
                            texture,
                            ..default()
                        },
                        Collider::compound(vec![(
                            Vect::new(0.0, -50.0),
                            0.0,
                            Collider::cuboid(25.0, 75.0),
                        )]),
                        YSort(0.0),
                    ));

                    inv_query
                        .single_mut()
                        .items
                        .entry(InventoryObject::Rocks)
                        .and_modify(|(_, count)| *count -= 1);

                    inv_query
                        .single_mut()
                        .items
                        .entry(InventoryObject::Wood)
                        .and_modify(|(_, count)| *count -= 1);

                    commands.entity(cursor.single().0).remove::<OnCursor>();
                }
                // Wall 2
                if *cursor2.single() == OnCursor(House::Wall2)
                    && *house_part == House::Wall2
                    && inv_query.single().recipe_satisfied(Recipe(House::Wall2))
                {
                    let texture = asset_server.load("wall2.png");
                    commands.spawn((
                        SpriteBundle {
                            transform: *cursor.single().1,
                            texture,
                            ..default()
                        },
                        Collider::compound(vec![(
                            Vect::new(0.0, -50.0),
                            0.0,
                            Collider::cuboid(75.0, 25.0),
                        )]),
                        YSort(0.0),
                    ));

                    inv_query
                        .single_mut()
                        .items
                        .entry(InventoryObject::Rocks)
                        .and_modify(|(_, count)| *count -= 1);

                    inv_query
                        .single_mut()
                        .items
                        .entry(InventoryObject::Wood)
                        .and_modify(|(_, count)| *count -= 1);

                    commands.entity(cursor.single().0).remove::<OnCursor>();
                }
                // Wall 3
                if *cursor2.single() == OnCursor(House::Wall3)
                    && *house_part == House::Wall3
                    && inv_query.single().recipe_satisfied(Recipe(House::Wall3))
                {
                    let texture = asset_server.load("wall3.png");
                    commands.spawn((
                        SpriteBundle {
                            transform: *cursor.single().1,
                            texture,
                            ..default()
                        },
                        Collider::compound(vec![(
                            Vect::new(0.0, -50.0),
                            0.0,
                            Collider::cuboid(25.0, 25.0),
                        )]),
                        YSort(0.0),
                    ));

                    inv_query
                        .single_mut()
                        .items
                        .entry(InventoryObject::Rocks)
                        .and_modify(|(_, count)| *count -= 1);

                    inv_query
                        .single_mut()
                        .items
                        .entry(InventoryObject::Wood)
                        .and_modify(|(_, count)| *count -= 1);

                    commands.entity(cursor.single().0).remove::<OnCursor>();
                }
                // Door
                if *cursor2.single() == OnCursor(House::Door)
                    && *house_part == House::Door
                    && inv_query.single().recipe_satisfied(Recipe(House::Door))
                {
                    let texture = asset_server.load("door.png");
                    commands.spawn((
                        SpriteBundle {
                            transform: *cursor.single().1,
                            texture,
                            ..default()
                        },
                        YSort(0.0),
                    ));

                    inv_query
                        .single_mut()
                        .items
                        .entry(InventoryObject::Rocks)
                        .and_modify(|(_, count)| *count -= 1);

                    inv_query
                        .single_mut()
                        .items
                        .entry(InventoryObject::Wood)
                        .and_modify(|(_, count)| *count -= 1);

                    commands.entity(cursor.single().0).remove::<OnCursor>();
                }
            }
        }
    }
}

fn pickup_object(
    rapier_context: Res<RapierContext>,
    object_query: Query<(Entity, &Transform, &InventoryObject)>,
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
                .and_modify(|(_, count)| *count += 1);
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn grow_plants(mut sprite_query: Query<(&mut Handle<Image>, &GrowStartTime, &WorldObject)>, asset_server: Res<AssetServer>,) {
    for (mut texture, grow_start_time, object) in sprite_query.iter_mut() {
        let time = grow_start_time.0.elapsed().as_secs();
        if time > 10 && time <= 20 {
            *texture = asset_server.load("sprout.png");
        }
        else if time > 20 && time <= 30 {
            if *object == WorldObject::FlowerbedWithBeans {
                *texture = asset_server.load("beans_level2.png");
            }
            else if *object == WorldObject::FlowerbedWithPotatoSeeds {
                *texture = asset_server.load("potatoes_level2.png");
            }
        }
        else if time > 30 {
            if *object == WorldObject::FlowerbedWithBeans {
                *texture = asset_server.load("beans_level3.png");
            }
            else if *object == WorldObject::FlowerbedWithPotatoSeeds {
                *texture = asset_server.load("potatoes_level3.png");
            }
        }
    }
}

fn spread_seed(
    input: Res<Input<KeyCode>>,
    inv_query: Query<&Inventory>,
    mut grid_query: Query<&mut Grid>,
    player_query: Query<&Transform, With<Player>>,
    mut commands: Commands,
    mut sprite_query: Query<(Entity, &mut Handle<Image>), With<WorldObject>>,
    asset_server: Res<AssetServer>,
) {
    if input.just_pressed(KeyCode::Space)
        && (inv_query.single().items[&InventoryObject::Beans].0
            || inv_query.single().items[&InventoryObject::PotatoSeeds].0)
    {
        let mut grid = grid_query.single_mut();
        let player_vec2 = player_query.single().translation.truncate();

        let object = grid.get_object(player_vec2);
        match object {
            Some((entity, _, obj)) => {
                // Remove flowerbed from grid
                if obj == WorldObject::Flowerbed {
                    grid.remove_object(player_vec2);
                }

                for (entity2, mut texture) in sprite_query.iter_mut() {
                    if entity == entity2 {
                        *texture = asset_server.load("flowerbed_with_seeds.png");
                        let now = Instant::now();

                        if inv_query.single().items[&InventoryObject::Beans].0 {
                            grid.place_object(entity, player_vec2, WorldObject::FlowerbedWithBeans);
                            commands.entity(entity).insert((WorldObject::FlowerbedWithBeans, GrowStartTime(now)));
                        }

                        if inv_query.single().items[&InventoryObject::PotatoSeeds].0 {
                            grid.place_object(entity, player_vec2, WorldObject::FlowerbedWithPotatoSeeds);
                            commands.entity(entity).insert((WorldObject::FlowerbedWithPotatoSeeds, GrowStartTime(now)));
                        }
                    }
                }
            }
            None => {}
        }
    }
}

fn spawn_flowerbed(pos: Vec3, commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let texture = asset_server.load("flowerbed.png");
    commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(50.0, 50.0)),
                    ..default()
                },
                texture,
                transform: Transform {
                    translation: pos,
                    ..default()
                },
                ..default()
            },
            WorldObject::Flowerbed,
            YSort(-250.0),
        ))
        .id()
}

fn dig_flowerbed(
    input: Res<Input<KeyCode>>,
    inv_query: Query<&Inventory>,
    mut grid_query: Query<&mut Grid>,
    player_query: Query<&Transform, With<Player>>,
    mut anim_query: Query<
        (
            &mut AnimationIndices,
            &mut TextureAtlasSprite,
            &mut Movement,
        ),
        With<Player>,
    >,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    if input.just_pressed(KeyCode::Space) && inv_query.single().items[&InventoryObject::Hoe].0 {
        let mut grid = grid_query.single_mut();
        let player_vec2 = player_query.single().translation.truncate();

        // Remove grass
        let object = grid.get_object(player_vec2);
        match object {
            Some((entity, _, obj)) => {
                if obj == WorldObject::Grass {
                    commands.entity(entity).despawn();
                    grid.remove_object(player_vec2);
                    return;
                }
            }
            None => {}
        }

        let pos = grid.world_to_grid(player_vec2);
        let (mut anim_indices, mut sprite, mut movement) = anim_query.single_mut();
        match *movement {
            Movement::Up => {
                anim_indices.first = 19;
                anim_indices.last = 19;
                *sprite = TextureAtlasSprite::new(19);

                let pos2 = grid.grid_to_world(pos);
                if grid.is_free(pos2) {
                    let id = spawn_flowerbed(pos2.extend(0.0), &mut commands, &asset_server);
                    grid.place_object(id, pos2, WorldObject::Flowerbed);
                }
            }
            Movement::Down => {
                anim_indices.first = 24;
                anim_indices.last = 24;
                *sprite = TextureAtlasSprite::new(24);

                let pos2 = grid.grid_to_world(pos);
                if grid.is_free(pos2) {
                    let id = spawn_flowerbed(pos2.extend(0.0), &mut commands, &asset_server);
                    grid.place_object(id, pos2, WorldObject::Flowerbed);
                }
            }
            Movement::Left => {
                anim_indices.first = 25;
                anim_indices.last = 25;
                *sprite = TextureAtlasSprite::new(25);

                let pos2 = grid.grid_to_world(pos);
                if grid.is_free(pos2) {
                    let id = spawn_flowerbed(pos2.extend(0.0), &mut commands, &asset_server);
                    grid.place_object(id, pos2, WorldObject::Flowerbed);
                }
            }
            Movement::Right => {
                anim_indices.first = 26;
                anim_indices.last = 26;
                *sprite = TextureAtlasSprite::new(26);

                let pos2 = grid.grid_to_world(pos);
                if grid.is_free(pos2) {
                    let id = spawn_flowerbed(pos2.extend(0.0), &mut commands, &asset_server);
                    grid.place_object(id, pos2, WorldObject::Flowerbed);
                }
            }
            Movement::Working => {}
            Movement::None => {
                anim_indices.first = 24;
                anim_indices.last = 24;
                *sprite = TextureAtlasSprite::new(24);
            }
        }
        *movement = Movement::Working;
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
    mut grid_query: Query<&mut Grid>,
) {
    if input.just_pressed(KeyCode::Space)
        && (inv_query.single().items[&InventoryObject::Axe].0
            || inv_query.single().items[&InventoryObject::Pickaxe].0)
    {
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
                            if object == &WorldObject::Tree
                                && inv_query.single().items[&InventoryObject::Axe].0
                            {
                                damage.0 -= 1;
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
                                && inv_query.single().items[&InventoryObject::Pickaxe].0
                            {
                                damage.0 -= 1;
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

                        if damage.0 == 0 {
                            commands.entity(entity).despawn_recursive();
                            grid_query
                                .single_mut()
                                .remove_object(transform.translation.truncate());

                            // Spawn wood
                            if *object == WorldObject::Tree {
                                let texture = asset_server.load("wood.png");
                                commands.spawn((
                                    SpriteBundle {
                                        texture,
                                        transform: *transform,
                                        ..default()
                                    },
                                    InventoryObject::Wood,
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
                                    InventoryObject::Rocks,
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
    let mut rng = rand::thread_rng();
    let mut trees_num = 0;
    loop {
        let x = rng.gen_range(-700.0..700.0);
        let y = rng.gen_range(-700.0..700.0);
        let vec = Vec2::new(x, y);
        // Don't spawn near player
        if x < 100.0 && x > -100.0 || y < 100.0 && y > -100.0 {
            continue;
        }

        let mut grid = grid_query.single_mut();
        if grid.is_free(vec) {
            let vec2 = grid.grid_to_world(grid.world_to_grid(vec));
            let texture = asset_server.load("tree.png");
            let id = commands
                .spawn((
                    SpriteBundle {
                        texture,
                        transform: Transform::from_xyz(vec2.x, vec2.y, 0.0),
                        ..default()
                    },
                    Collider::convex_hull(&[
                        Vect::new(-20.0, -40.0),
                        Vect::new(20.0, -40.0),
                        Vect::new(-20.0, 0.0),
                        Vect::new(20.0, 0.0),
                    ])
                    .unwrap(),
                    WorldObject::Tree,
                    Damage(3),
                    YSort(0.0),
                ))
                .id();
            grid.place_object(id, vec, WorldObject::Tree);
            trees_num += 1
        } else {
            continue;
        }
        if trees_num == 20 {
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
        let x = rng.gen_range(-700.0..700.0);
        let y = rng.gen_range(-700.0..700.0);
        let vec = Vec2::new(x, y);
        // Don't spawn near player
        if x < 100.0 && x > -100.0 || y < 100.0 && y > -100.0 {
            continue;
        }

        let mut grid = grid_query.single_mut();
        if grid.is_free(vec) {
            let vec2 = grid.grid_to_world(grid.world_to_grid(vec));
            let texture = asset_server.load("rock.png");
            let id = commands
                .spawn((
                    SpriteBundle {
                        texture,
                        transform: Transform::from_xyz(vec2.x, vec2.y, 0.0),
                        ..default()
                    },
                    Collider::convex_hull(&[
                        Vect::new(-20.0, -20.0),
                        Vect::new(20.0, -20.0),
                        Vect::new(-20.0, 0.0),
                        Vect::new(20.0, 0.0),
                    ])
                    .unwrap(),
                    WorldObject::Rock,
                    Damage(2),
                    YSort(0.0),
                ))
                .id();
            grid.place_object(id, vec, WorldObject::Rock);
            rocks_num += 1
        } else {
            continue;
        }
        if rocks_num == 20 {
            break;
        }
    }
}

fn spawn_grass(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut grid_query: Query<&mut Grid>,
) {
    let mut rng = rand::thread_rng();
    let mut grass_num = 0;
    loop {
        let x = rng.gen_range(-700.0..700.0);
        let y = rng.gen_range(-700.0..700.0);
        let vec = Vec2::new(x, y);

        let mut grid = grid_query.single_mut();
        if grid.is_free(vec) {
            let vec2 = grid.grid_to_world(grid.world_to_grid(vec));
            let texture = asset_server.load("grass.png");
            let id = commands
                .spawn((
                    SpriteBundle {
                        texture,
                        transform: Transform::from_xyz(vec2.x, vec2.y, 0.0),
                        ..default()
                    },
                    WorldObject::Grass,
                    YSort(0.0),
                ))
                .id();
            grid.place_object(id, vec, WorldObject::Grass);
            grass_num += 1
        } else {
            continue;
        }
        if grass_num == 20 {
            break;
        }
    }
}

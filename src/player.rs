use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::Grid;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component, PartialEq)]
enum Movement {
    Up,
    Down,
    Left,
    Right,
    None,
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, setup_player)
            .add_systems(Update, (character_movement, animate_sprite));
    }
}

fn character_movement(
    mut controller_query: Query<&mut KinematicCharacterController, With<Player>>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut anim_query: Query<
        (
            &mut AnimationIndices,
            &mut TextureAtlasSprite,
            &mut Movement,
        ),
        With<Player>,
    >,
) {
    let mut x = 0.0;
    let mut y = 0.0;
    if input.pressed(KeyCode::W) {
        // controller.translation = Some(Vec2::new(0.0, SPEED * time.delta_seconds()));
        y = 1.0;
    } else if input.pressed(KeyCode::S) {
        y = -1.0;
    }
    if input.pressed(KeyCode::D) {
        x = 1.0;
    } else if input.pressed(KeyCode::A) {
        x = -1.0;
    }

    let mut controller = controller_query.single_mut();
    let (mut anim_indices, mut sprite, mut movement) = anim_query.single_mut();
    if x == 1.0 && y == 0.0 {
        if *movement != Movement::Right {
            anim_indices.first = 8;
            anim_indices.last = 11;
            *sprite = TextureAtlasSprite::new(8);
        }
        *movement = Movement::Right;
    }
    else if x == 0.0 && y == 1.0 {
        if *movement != Movement::Up {
            anim_indices.first = 4;
            anim_indices.last = 5;
            *sprite = TextureAtlasSprite::new(4);
        }
        *movement = Movement::Up;
    }
    else if x == 0.0 && y == -1.0 {
        if *movement != Movement::Down {
            anim_indices.first = 0;
            anim_indices.last = 1;
            *sprite = TextureAtlasSprite::new(0);
        }
        *movement = Movement::Down;
    }
    else if x == -1.0 && y == 0.0 {
        if *movement != Movement::Left {
            anim_indices.first = 12;
            anim_indices.last = 15;
            *sprite = TextureAtlasSprite::new(12)
        }
        *movement = Movement::Left;
    }
    else if x == 1.0 && y == 1.0 {
        if *movement != Movement::Up {
            anim_indices.first = 4;
            anim_indices.last = 5;
            *sprite = TextureAtlasSprite::new(4);
        }
        *movement = Movement::Up;
    }
    else if x == 1.0 && y == -1.0 {
        if *movement != Movement::Down {
            anim_indices.first = 0;
            anim_indices.last = 1;
            *sprite = TextureAtlasSprite::new(0);
        }
        *movement = Movement::Down;
    }
    else if x == -1.0 && y == 1.0 {
        if *movement != Movement::Up {
            anim_indices.first = 4;
            anim_indices.last = 5;
            *sprite = TextureAtlasSprite::new(4);
        }
        *movement = Movement::Up;
    }
    else if x == -1.0 && y == -1.0 {
        if *movement != Movement::Down {
            anim_indices.first = 0;
            anim_indices.last = 1;
            *sprite = TextureAtlasSprite::new(0);
        }
        *movement = Movement::Down;
    }

    if x == 0.0 && y == 0.0 {
        match *movement {
            Movement::Up => {
                anim_indices.first = 6;
                anim_indices.last = 6;
                *sprite = TextureAtlasSprite::new(6)
            }
            Movement::Down => {
                anim_indices.first = 2;
                anim_indices.last = 2;
                *sprite = TextureAtlasSprite::new(2)
            }
            Movement::Left => {
                anim_indices.first = 13;
                anim_indices.last = 13;
                *sprite = TextureAtlasSprite::new(13)
            }
            Movement::Right => {
                anim_indices.first = 9;
                anim_indices.last = 9;
                *sprite = TextureAtlasSprite::new(9)
            }
            Movement::None => {
                anim_indices.first = 2;
                anim_indices.last = 2;
                *sprite = TextureAtlasSprite::new(2)
            }
        }
    } else {
        const SPEED: f32 = 150.0;
        let v2_norm = Vec2::new(x, y).normalize();
        controller.translation = Some(Vec2::new(v2_norm.x * SPEED * time.delta_seconds(), v2_norm.y * SPEED * time.delta_seconds()))
    }
}

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = if sprite.index == indices.last {
                indices.first
            } else {
                sprite.index + 1
            };
        }
    }
}

fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut grid_query: Query<&mut Grid>,
) {
    let mut grid = grid_query.single_mut();
    let pos = grid.grid_to_world((0, 0));
    grid.place_object("player".to_string(), (0, 0));

    let texture_handle = asset_server.load("spritesheet.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(100.0, 100.0), 4, 4, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    // Use only the subset of sprites in the sheet that make up the run animation
    let animation_indices = AnimationIndices { first: 2, last: 2 };
    commands.spawn((
        Player,
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(animation_indices.first),
            transform: Transform::from_xyz(pos.x, pos.y, 0.0),
            ..default()
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.3, TimerMode::Repeating)),
        KinematicCharacterController::default(),
        RigidBody::KinematicVelocityBased,
        Collider::capsule_y(25.0, 25.0),
        Movement::None,
    ));
}

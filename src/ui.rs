use crate::{Cursor, Inventory, PickableObject, Recipe, Tool};
use bevy::prelude::*;

#[derive(Component)]
pub struct WoodText;

#[derive(Component)]
pub struct RocksText;

#[derive(Component)]
pub struct AxeButton;

#[derive(Component)]
pub struct PickaxeButton;

#[derive(Component)]
pub struct HouseButton;

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, setup_ui).add_systems(
            Update,
            (
                update_wood_text,
                update_rocks_text,
                interact_with_axe_button,
                interact_with_pickaxe_button,
                spawn_house_button,
                interact_with_house_button,
            ),
        );
    }
}

pub fn update_wood_text(
    mut text_query: Query<&mut Text, With<WoodText>>,
    inv_query: Query<&Inventory>,
) {
    let count = inv_query.single().items[&PickableObject::Wood];
    for mut text in text_query.iter_mut() {
        text.sections[0].value = count.to_string();
    }
}

pub fn update_rocks_text(
    mut text_query: Query<&mut Text, With<RocksText>>,
    inv_query: Query<&Inventory>,
) {
    let count = inv_query.single().items[&PickableObject::Rocks];
    for mut text in text_query.iter_mut() {
        text.sections[0].value = count.to_string();
    }
}

fn spawn_house_button(
    mut text_query: Query<Entity, With<HouseButton>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut inv_query: Query<&mut Inventory>,
) {
    if inv_query.single_mut().items[&PickableObject::Wood] >= 5
        && inv_query.single_mut().items[&PickableObject::Rocks] >= 5
        && !inv_query.single_mut().recipes[&Recipe::House]
    {
        commands
            .entity(text_query.single_mut())
            .with_children(|parent| {
                parent.spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Px(50.0),
                            height: Val::Px(50.0),
                            ..default()
                        },
                        background_color: Color::WHITE.into(),
                        ..default()
                    },
                    UiImage::new(asset_server.load("house_icon.png")),
                ));
            });

        inv_query
            .single_mut()
            .recipes
            .entry(Recipe::House)
            .and_modify(|recipe| *recipe = true);
    }
}

pub fn interact_with_house_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<HouseButton>),
    >,
    cursor: Query<(Entity, &Transform), With<Cursor>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = Color::WHITE.into();
                let texture = asset_server.load("cottage.png");
                commands
                    .entity(cursor.single().0)
                    .insert(SpriteBundle {
                        transform: *cursor.single().1,
                        texture,
                        ..default()
                    });
            }
            Interaction::Hovered => {
                *background_color = Color::GRAY.into();
            }
            Interaction::None => {
                *background_color = Color::BLACK.into();
            }
        }
    }
}

pub fn interact_with_axe_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<AxeButton>),
    >,
    mut button_query2: Query<&mut BackgroundColor, (With<PickaxeButton>, Without<AxeButton>)>,
    mut inv_query: Query<&mut Inventory>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = Color::WHITE.into();
                inv_query
                    .single_mut()
                    .tools
                    .entry(Tool::Axe)
                    .and_modify(|using| *using = true);
                inv_query
                    .single_mut()
                    .tools
                    .entry(Tool::Pickaxe)
                    .and_modify(|using| *using = false);
                button_query2.single_mut().0 = Color::BLACK.into();
            }
            Interaction::Hovered => {
                *background_color = Color::GRAY.into();
            }
            Interaction::None => {
                if inv_query.single_mut().tools[&Tool::Axe] {
                    *background_color = Color::WHITE.into();
                } else {
                    *background_color = Color::BLACK.into();
                }
            }
        }
    }
}

pub fn interact_with_pickaxe_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<PickaxeButton>),
    >,
    mut button_query2: Query<&mut BackgroundColor, (With<AxeButton>, Without<PickaxeButton>)>,
    mut inv_query: Query<&mut Inventory>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = Color::WHITE.into();
                inv_query
                    .single_mut()
                    .tools
                    .entry(Tool::Pickaxe)
                    .and_modify(|using| *using = true);
                inv_query
                    .single_mut()
                    .tools
                    .entry(Tool::Axe)
                    .and_modify(|using| *using = false);
                button_query2.single_mut().0 = Color::BLACK.into();
            }
            Interaction::Hovered => {
                *background_color = Color::GRAY.into();
            }
            Interaction::None => {
                if inv_query.single_mut().tools[&Tool::Pickaxe] {
                    *background_color = Color::WHITE.into();
                } else {
                    *background_color = Color::BLACK.into();
                }
            }
        }
    }
}

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                display: Display::Flex,
                height: Val::Percent(100.0),
                width: Val::Percent(100.0),
                ..default()
            },
            ..default()
        })
        // Left column
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(15.0),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                })
                // Axe
                .with_children(|parent| {
                    parent
                        .spawn((
                            ButtonBundle {
                                background_color: Color::BLACK.into(),
                                ..default()
                            },
                            AxeButton,
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                NodeBundle {
                                    style: Style {
                                        width: Val::Px(50.0),
                                        height: Val::Px(50.0),
                                        ..default()
                                    },
                                    background_color: Color::WHITE.into(),
                                    ..default()
                                },
                                UiImage::new(asset_server.load("axe.png")),
                            ));
                        });
                })
                // Pickaxe
                .with_children(|parent| {
                    parent
                        .spawn((
                            ButtonBundle {
                                background_color: Color::BLACK.into(),
                                ..default()
                            },
                            PickaxeButton,
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                NodeBundle {
                                    style: Style {
                                        width: Val::Px(50.0),
                                        height: Val::Px(50.0),
                                        ..default()
                                    },
                                    background_color: Color::WHITE.into(),
                                    ..default()
                                },
                                UiImage::new(asset_server.load("pickaxe.png")),
                            ));
                        });
                });
        })
        // Middle column
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        align_self: AlignSelf::FlexEnd,
                        height: Val::Percent(15.0),
                        width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    // background_color: Color::rgb(0.65, 0.65, 0.65).into(),
                    ..default()
                })
                // Wood
                .with_children(|parent| {
                    parent.spawn((
                        NodeBundle {
                            style: Style {
                                width: Val::Px(50.0),
                                height: Val::Px(50.0),
                                ..default()
                            },
                            background_color: Color::WHITE.into(),
                            ..default()
                        },
                        UiImage::new(asset_server.load("wood.png")),
                    ));
                    parent.spawn((
                        TextBundle::from_section(
                            "0",
                            TextStyle {
                                font_size: 30.0,
                                color: Color::WHITE,
                                ..default()
                            },
                        ),
                        WoodText,
                    ));
                })
                // Rocks
                .with_children(|parent| {
                    parent.spawn((
                        NodeBundle {
                            style: Style {
                                width: Val::Px(50.0),
                                height: Val::Px(50.0),
                                ..default()
                            },
                            background_color: Color::WHITE.into(),
                            ..default()
                        },
                        UiImage::new(asset_server.load("rocks.png")),
                    ));
                    parent.spawn((
                        TextBundle::from_section(
                            "0",
                            TextStyle {
                                font_size: 30.0,
                                color: Color::WHITE,
                                ..default()
                            },
                        ),
                        RocksText,
                    ));
                });
        })
        // Right column
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(15.0),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    // background_color: Color::rgb(0.65, 0.65, 0.65).into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        ButtonBundle {
                            background_color: Color::BLACK.into(),
                            ..default()
                        },
                        HouseButton,
                    ));
                    // .with_children(|parent| {
                    //     parent.spawn((
                    //         NodeBundle {
                    //             style: Style {
                    //                 width: Val::Px(50.0),
                    //                 height: Val::Px(50.0),
                    //                 ..default()
                    //             },
                    //             background_color: Color::WHITE.into(),
                    //             ..default()
                    //         },
                    //         UiImage::new(asset_server.load("axe.png")),
                    //     ));
                    // });
                });
        });
}

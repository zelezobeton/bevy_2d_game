use crate::{Cursor, Inventory, InventoryObject, Recipe, Tool};
use bevy::prelude::*;

#[derive(Component)]
pub struct WoodText;

#[derive(Component)]
pub struct RocksText;

#[derive(Component)]
pub struct AxeButton;

#[derive(Component)]
pub struct PickaxeButton;

#[derive(Component, PartialEq, Eq, Hash, Debug, Clone)]
pub enum House {
    Corner1,
    Corner2,
    Corner3,
    Corner4,
    Wall1,
    Wall2,
    Wall3,
    Door,
}

#[derive(Component, PartialEq)]
pub struct OnCursor(pub House);

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
                color_house_button,
                interact_with_house_button,
            ),
        );
    }
}

pub fn update_wood_text(
    mut text_query: Query<&mut Text, With<WoodText>>,
    inv_query: Query<&Inventory>,
) {
    let count = inv_query.single().items[&InventoryObject::Wood];
    for mut text in text_query.iter_mut() {
        text.sections[0].value = count.to_string();
    }
}

pub fn update_rocks_text(
    mut text_query: Query<&mut Text, With<RocksText>>,
    inv_query: Query<&Inventory>,
) {
    let count = inv_query.single().items[&InventoryObject::Rocks];
    for mut text in text_query.iter_mut() {
        text.sections[0].value = count.to_string();
    }
}

fn color_house_button(
    mut button_query: Query<(&House, &mut BackgroundColor)>,
    inv_query: Query<&Inventory>,
) {
    for (house_part, mut background_color) in button_query.iter_mut() {
        match *house_part {
            House::Corner1 => {
                if inv_query.single().recipe_satisfied(Recipe(House::Corner1)) {
                    *background_color = Color::rgb(0.0, 0.65, 0.0).into();
                } else {
                    *background_color = Color::RED.into();
                }
            }
            House::Corner2 => {
                if inv_query.single().recipe_satisfied(Recipe(House::Corner2)) {
                    *background_color = Color::rgb(0.0, 0.65, 0.0).into();
                } else {
                    *background_color = Color::RED.into();
                }
            }
            House::Corner3 => {
                if inv_query.single().recipe_satisfied(Recipe(House::Corner3)) {
                    *background_color = Color::rgb(0.0, 0.65, 0.0).into();
                } else {
                    *background_color = Color::RED.into();
                }
            }
            House::Corner4 => {
                if inv_query.single().recipe_satisfied(Recipe(House::Corner4)) {
                    *background_color = Color::rgb(0.0, 0.65, 0.0).into();
                } else {
                    *background_color = Color::RED.into();
                }
            }
            House::Wall1 => {
                if inv_query.single().recipe_satisfied(Recipe(House::Wall1)) {
                    *background_color = Color::rgb(0.0, 0.65, 0.0).into();
                } else {
                    *background_color = Color::RED.into();
                }
            }
            House::Wall2 => {
                if inv_query.single().recipe_satisfied(Recipe(House::Wall2)) {
                    *background_color = Color::rgb(0.0, 0.65, 0.0).into();
                } else {
                    *background_color = Color::RED.into();
                }
            }
            House::Wall3 => {
                if inv_query.single().recipe_satisfied(Recipe(House::Wall3)) {
                    *background_color = Color::rgb(0.0, 0.65, 0.0).into();
                } else {
                    *background_color = Color::RED.into();
                }
            }
            House::Door => {
                if inv_query.single().recipe_satisfied(Recipe(House::Door)) {
                    *background_color = Color::rgb(0.0, 0.65, 0.0).into();
                } else {
                    *background_color = Color::RED.into();
                }
            }
        }
    }
}

fn interact_with_house_button(
    mut button_query: Query<(&Interaction, &mut BorderColor, &House), Changed<Interaction>>,
    cursor: Query<(Entity, &Transform), With<Cursor>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    inv_query: Query<&Inventory>,
) {
    for (interaction, mut border_color, house_part) in button_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => match house_part {
                House::Corner1 => {
                    *border_color = Color::WHITE.into();
                    if inv_query.single().recipe_satisfied(Recipe(House::Corner1)) {
                        let texture = asset_server.load("corner1.png");
                        commands
                            .entity(cursor.single().0)
                            .insert(SpriteBundle {
                                transform: *cursor.single().1,
                                texture,
                                ..default()
                            })
                            .insert(OnCursor(House::Corner1));
                    }
                }
                House::Corner2 => {
                    *border_color = Color::WHITE.into();
                    if inv_query.single().recipe_satisfied(Recipe(House::Corner2)) {
                        let texture = asset_server.load("corner2.png");
                        commands
                            .entity(cursor.single().0)
                            .insert(SpriteBundle {
                                transform: *cursor.single().1,
                                texture,
                                ..default()
                            })
                            .insert(OnCursor(House::Corner2));
                    }
                }
                House::Corner3 => {
                    *border_color = Color::WHITE.into();
                    if inv_query.single().recipe_satisfied(Recipe(House::Corner3)) {
                        let texture = asset_server.load("corner3.png");
                        commands
                            .entity(cursor.single().0)
                            .insert(SpriteBundle {
                                transform: *cursor.single().1,
                                texture,
                                ..default()
                            })
                            .insert(OnCursor(House::Corner3));
                    }
                }
                House::Corner4 => {
                    *border_color = Color::WHITE.into();
                    if inv_query.single().recipe_satisfied(Recipe(House::Corner4)) {
                        let texture = asset_server.load("corner4.png");
                        commands
                            .entity(cursor.single().0)
                            .insert(SpriteBundle {
                                transform: *cursor.single().1,
                                texture,
                                ..default()
                            })
                            .insert(OnCursor(House::Corner4));
                    }
                }
                House::Wall1 => {
                    *border_color = Color::WHITE.into();
                    if inv_query.single().recipe_satisfied(Recipe(House::Wall1)) {
                        let texture = asset_server.load("wall1.png");
                        commands
                            .entity(cursor.single().0)
                            .insert(SpriteBundle {
                                transform: *cursor.single().1,
                                texture,
                                ..default()
                            })
                            .insert(OnCursor(House::Wall1));
                    }
                }
                House::Wall2 => {
                    *border_color = Color::WHITE.into();
                    if inv_query.single().recipe_satisfied(Recipe(House::Wall2)) {
                        let texture = asset_server.load("wall2.png");
                        commands
                            .entity(cursor.single().0)
                            .insert(SpriteBundle {
                                transform: *cursor.single().1,
                                texture,
                                ..default()
                            })
                            .insert(OnCursor(House::Wall2));

                    }
                }
                House::Wall3 => {
                    *border_color = Color::WHITE.into();
                    if inv_query.single().recipe_satisfied(Recipe(House::Wall3)) {
                        let texture = asset_server.load("wall3.png");
                        commands
                            .entity(cursor.single().0)
                            .insert(SpriteBundle {
                                transform: *cursor.single().1,
                                texture,
                                ..default()
                            })
                            .insert(OnCursor(House::Wall3));
                    }
                }
                House::Door => {
                    *border_color = Color::WHITE.into();
                    if inv_query.single().recipe_satisfied(Recipe(House::Door)) {
                        let texture = asset_server.load("door.png");
                        commands
                            .entity(cursor.single().0)
                            .insert(SpriteBundle {
                                transform: *cursor.single().1,
                                texture,
                                ..default()
                            })
                            .insert(OnCursor(House::Door));
                    }
                }
            },
            Interaction::Hovered => {
                *border_color = Color::GRAY.into();
            }
            Interaction::None => {
                *border_color = Color::BLACK.into();
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
                    // Wall 1
                    parent
                        .spawn((
                            ButtonBundle {
                                background_color: Color::BLACK.into(),
                                border_color: Color::BLACK.into(),
                                style: Style {
                                    border: UiRect::all(Val::Px(2.0)),
                                    ..default()
                                },
                                ..default()
                            },
                            House::Wall1,
                        ))
                        .with_children(|parent| {
                            parent.spawn(ImageBundle {
                                style: Style {
                                    width: Val::Px(50.0),
                                    height: Val::Px(50.0),
                                    ..default()
                                },
                                image: UiImage::new(asset_server.load("wall1_icon.png")),
                                ..default()
                            });
                        });
                    // Wall 2
                    parent
                        .spawn((
                            ButtonBundle {
                                background_color: Color::BLACK.into(),
                                border_color: Color::BLACK.into(),
                                style: Style {
                                    border: UiRect::all(Val::Px(2.0)),
                                    ..default()
                                },
                                ..default()
                            },
                            House::Wall2,
                        ))
                        .with_children(|parent| {
                            parent.spawn(ImageBundle {
                                style: Style {
                                    width: Val::Px(50.0),
                                    height: Val::Px(50.0),
                                    ..default()
                                },
                                image: UiImage::new(asset_server.load("wall2_icon.png")),
                                ..default()
                            });
                        });
                    // Wall 2
                    parent
                        .spawn((
                            ButtonBundle {
                                background_color: Color::BLACK.into(),
                                border_color: Color::BLACK.into(),
                                style: Style {
                                    border: UiRect::all(Val::Px(2.0)),
                                    ..default()
                                },
                                ..default()
                            },
                            House::Wall3,
                        ))
                        .with_children(|parent| {
                            parent.spawn(ImageBundle {
                                style: Style {
                                    width: Val::Px(50.0),
                                    height: Val::Px(50.0),
                                    ..default()
                                },
                                image: UiImage::new(asset_server.load("wall3_icon.png")),
                                ..default()
                            });
                        });
                    // Corner 1
                    parent
                        .spawn((
                            ButtonBundle {
                                background_color: Color::BLACK.into(),
                                border_color: Color::BLACK.into(),
                                style: Style {
                                    border: UiRect::all(Val::Px(2.0)),
                                    ..default()
                                },
                                ..default()
                            },
                            House::Corner1,
                        ))
                        .with_children(|parent| {
                            parent.spawn(ImageBundle {
                                style: Style {
                                    width: Val::Px(50.0),
                                    height: Val::Px(50.0),
                                    ..default()
                                },
                                image: UiImage::new(asset_server.load("corner1_icon.png")),
                                ..default()
                            });
                        });
                    // Corner 2
                    parent
                        .spawn((
                            ButtonBundle {
                                background_color: Color::BLACK.into(),
                                border_color: Color::BLACK.into(),
                                style: Style {
                                    border: UiRect::all(Val::Px(2.0)),
                                    ..default()
                                },
                                ..default()
                            },
                            House::Corner2,
                        ))
                        .with_children(|parent| {
                            parent.spawn(ImageBundle {
                                style: Style {
                                    width: Val::Px(50.0),
                                    height: Val::Px(50.0),
                                    ..default()
                                },
                                image: UiImage::new(asset_server.load("corner2_icon.png")),
                                ..default()
                            });
                        });
                    // Corner 3
                    parent
                        .spawn((
                            ButtonBundle {
                                background_color: Color::BLACK.into(),
                                border_color: Color::BLACK.into(),
                                style: Style {
                                    border: UiRect::all(Val::Px(2.0)),
                                    ..default()
                                },
                                ..default()
                            },
                            House::Corner3,
                        ))
                        .with_children(|parent| {
                            parent.spawn(ImageBundle {
                                style: Style {
                                    width: Val::Px(50.0),
                                    height: Val::Px(50.0),
                                    ..default()
                                },
                                image: UiImage::new(asset_server.load("corner3_icon.png")),
                                ..default()
                            });
                        });
                    // Corner 4
                    parent
                        .spawn((
                            ButtonBundle {
                                background_color: Color::BLACK.into(),
                                border_color: Color::BLACK.into(),
                                style: Style {
                                    border: UiRect::all(Val::Px(2.0)),
                                    ..default()
                                },
                                ..default()
                            },
                            House::Corner4,
                        ))
                        .with_children(|parent| {
                            parent.spawn(ImageBundle {
                                style: Style {
                                    width: Val::Px(50.0),
                                    height: Val::Px(50.0),
                                    ..default()
                                },
                                image: UiImage::new(asset_server.load("corner4_icon.png")),
                                ..default()
                            });
                        });
                    // Door
                    parent
                        .spawn((
                            ButtonBundle {
                                background_color: Color::BLACK.into(),
                                border_color: Color::BLACK.into(),
                                style: Style {
                                    border: UiRect::all(Val::Px(2.0)),
                                    ..default()
                                },
                                ..default()
                            },
                            House::Door,
                        ))
                        .with_children(|parent| {
                            parent.spawn(ImageBundle {
                                style: Style {
                                    width: Val::Px(50.0),
                                    height: Val::Px(50.0),
                                    ..default()
                                },
                                image: UiImage::new(asset_server.load("door_icon.png")),
                                ..default()
                            });
                        });
                });
        });
}

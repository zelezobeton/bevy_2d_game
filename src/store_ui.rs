use crate::menu_ui::{HOVERED_BUTTON, NORMAL_BUTTON, PRESSED_BUTTON};
use crate::InventoryObject;
use crate::{AppState, Inventory};

use bevy::prelude::*;

#[derive(Component)]
struct Store;

#[derive(Component)]
struct ExitButton;

#[derive(Component)]
struct BeansButton;

#[derive(Component)]
struct PotatoesButton;

#[derive(Component)]
pub struct CoinsText;

pub struct StoreUiPlugin;
impl Plugin for StoreUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Store), spawn_store)
            .add_systems(
                Update,
                (
                    interact_with_exit_button,
                    interact_with_beans_button,
                    interact_with_potatoes_button,
                    update_coins_text
                )
                    .run_if(in_state(AppState::Store)),
            )
            .add_systems(OnExit(AppState::Store), despawn_store);
    }
}

fn spawn_store(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    costs_query: Query<&Inventory>,
) {
    commands
        .spawn(NodeBundle {
            style: Style {
                display: Display::Flex,
                height: Val::Percent(100.0),
                width: Val::Percent(100.0),
                ..default()
            },
            background_color: Color::BLACK.into(),
            ..default()
        })
        .insert(Store)
        // Left column
        .with_children(|parent| {
            parent.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(25.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            });
        })
        // Middle column
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        align_self: AlignSelf::FlexEnd,
                        height: Val::Percent(100.0),
                        width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Start,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::DARK_GRAY.into(),
                    ..default()
                })
                // Beans
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                align_self: AlignSelf::FlexEnd,
                                height: Val::Percent(15.0),
                                width: Val::Percent(100.0),
                                flex_direction: FlexDirection::Row,
                                justify_content: JustifyContent::SpaceBetween,
                                align_items: AlignItems::Center,
                                border: UiRect::all(Val::Px(1.0)),
                                ..default()
                            },
                            border_color: Color::GRAY.into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(ImageBundle {
                                style: Style {
                                    width: Val::Px(50.0),
                                    height: Val::Px(50.0),
                                    ..default()
                                },
                                image: UiImage::new(asset_server.load("beans.png")),
                                ..default()
                            });
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Beans",
                                TextStyle {
                                    font_size: 30.0,
                                    color: Color::WHITE,
                                    ..default()
                                },
                            ));
                        })
                        .with_children(|parent| {
                            parent
                                .spawn((
                                    ButtonBundle {
                                        background_color: Color::BLACK.into(),
                                        ..default()
                                    },
                                    BeansButton,
                                ))
                                .with_children(|parent| {
                                    parent.spawn(ImageBundle {
                                        style: Style {
                                            width: Val::Px(25.0),
                                            height: Val::Px(25.0),
                                            ..default()
                                        },
                                        image: UiImage::new(asset_server.load("coins.png")),
                                        ..default()
                                    });
                                })
                                .with_children(|parent| {
                                    parent.spawn(TextBundle::from_section(
                                        costs_query.single().costs[&InventoryObject::Beans]
                                            .to_string(),
                                        TextStyle {
                                            font_size: 30.0,
                                            color: Color::rgb(0.9, 0.9, 0.9),
                                            ..default()
                                        },
                                    ));
                                });
                        });
                })
                // Potato seeds
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                align_self: AlignSelf::FlexEnd,
                                height: Val::Percent(15.0),
                                width: Val::Percent(100.0),
                                flex_direction: FlexDirection::Row,
                                justify_content: JustifyContent::SpaceBetween,
                                align_items: AlignItems::Center,
                                border: UiRect::all(Val::Px(1.0)),
                                ..default()
                            },
                            border_color: Color::GRAY.into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(ImageBundle {
                                style: Style {
                                    width: Val::Px(50.0),
                                    height: Val::Px(50.0),
                                    ..default()
                                },
                                image: UiImage::new(asset_server.load("potato_seeds.png")),
                                ..default()
                            });
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Potato seeds",
                                TextStyle {
                                    font_size: 30.0,
                                    color: Color::WHITE,
                                    ..default()
                                },
                            ));
                        })
                        .with_children(|parent| {
                            parent
                                .spawn((
                                    ButtonBundle {
                                        background_color: Color::BLACK.into(),
                                        ..default()
                                    },
                                    PotatoesButton,
                                ))
                                .with_children(|parent| {
                                    parent.spawn(ImageBundle {
                                        style: Style {
                                            width: Val::Px(25.0),
                                            height: Val::Px(25.0),
                                            ..default()
                                        },
                                        image: UiImage::new(asset_server.load("coins.png")),
                                        ..default()
                                    });
                                })
                                .with_children(|parent| {
                                    parent.spawn(TextBundle::from_section(
                                        costs_query.single().costs[&InventoryObject::PotatoSeeds]
                                            .to_string(),
                                        TextStyle {
                                            font_size: 30.0,
                                            color: Color::rgb(0.9, 0.9, 0.9),
                                            ..default()
                                        },
                                    ));
                                });
                        });
                });
        })
        // Right column
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(25.0),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                height: Val::Percent(100.0),
                                width: Val::Percent(100.0),
                                flex_direction: FlexDirection::Row,
                                justify_content: JustifyContent::End,
                                align_items: AlignItems::Start,
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(ImageBundle {
                                style: Style {
                                    width: Val::Px(25.0),
                                    height: Val::Px(25.0),
                                    ..default()
                                },
                                image: UiImage::new(asset_server.load("coins.png")),
                                ..default()
                            });
                        })
                        .with_children(|parent| {
                            parent.spawn((
                                TextBundle::from_section(
                                    "0",
                                    TextStyle {
                                        font_size: 30.0,
                                        color: Color::WHITE,
                                        ..default()
                                    },
                                ),
                                CoinsText,
                            ));
                        });
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                height: Val::Percent(100.0),
                                width: Val::Percent(100.0),
                                flex_direction: FlexDirection::Column,
                                justify_content: JustifyContent::End,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            ..default()
                        })
                        // Exit button
                        .with_children(|parent| {
                            parent
                                .spawn(NodeBundle {
                                    background_color: Color::WHITE.into(),
                                    ..default()
                                })
                                .with_children(|parent| {
                                    parent
                                        .spawn((
                                            ButtonBundle {
                                                background_color: Color::BLACK.into(),
                                                ..default()
                                            },
                                            ExitButton,
                                        ))
                                        .with_children(|parent| {
                                            parent.spawn(TextBundle::from_section(
                                                "Exit",
                                                TextStyle {
                                                    font_size: 30.0,
                                                    color: Color::WHITE,
                                                    ..default()
                                                },
                                            ));
                                        });
                                });
                        });
                });
        });
}

pub fn update_coins_text(
    mut text_query: Query<&mut Text, With<CoinsText>>,
    inv_query: Query<&Inventory>,
) {
    let count = inv_query.single().coins;
    for mut text in text_query.iter_mut() {
        text.sections[0].value = count.to_string();
    }
}

fn interact_with_exit_button(
    mut next_state: ResMut<NextState<AppState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<ExitButton>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                next_state.set(AppState::InGame);
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

fn interact_with_beans_button(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<BeansButton>),
    >,
    mut inv_query: Query<&mut Inventory>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                if inv_query.single_mut().coins >= 20 {
                    inv_query
                        .single_mut()
                        .items
                        .entry(InventoryObject::Beans)
                        .and_modify(|(_, count)| *count += 1);
                    inv_query.single_mut().coins -= 20;
                }
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

fn interact_with_potatoes_button(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<PotatoesButton>),
    >,
    mut inv_query: Query<&mut Inventory>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                if inv_query.single_mut().coins >= 10 {
                    inv_query
                        .single_mut()
                        .items
                        .entry(InventoryObject::PotatoSeeds)
                        .and_modify(|(_, count)| *count += 1);
                    inv_query.single_mut().coins -= 10;
                }
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

fn despawn_store(mut commands: Commands, query: Query<Entity, With<Store>>) {
    commands.entity(query.single()).despawn_recursive();
}

use crate::{AppState, StoreCoins};
use crate::InventoryObject;
use crate::menu_ui::{HOVERED_BUTTON, NORMAL_BUTTON, PRESSED_BUTTON};

use bevy::prelude::*;

#[derive(Component)]
struct Store;

#[derive(Component)]
struct ExitButton;

#[derive(Component)]
struct BeansButton;

#[derive(Component)]
struct PotatoesButton;

pub struct StoreUiPlugin;
impl Plugin for StoreUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Store), spawn_store)
            .add_systems(
                Update,
                interact_with_exit_button.run_if(in_state(AppState::Store)),
            )
            .add_systems(OnExit(AppState::Store), despawn_store);
    }
}

fn spawn_store(mut commands: Commands, asset_server: Res<AssetServer>, store_query: Query<&StoreCoins>) {
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
                // background_color: Color::GRAY.into(),
                ..default()
            });
        })
        // Middle column
        .with_children(|parent| {
            parent.spawn(NodeBundle {
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
                parent.spawn(NodeBundle {
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
                                store_query.single().costs[&InventoryObject::Beans].to_string(),
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
                parent.spawn(NodeBundle {
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
                                store_query.single().costs[&InventoryObject::PotatoSeeds].to_string(),
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
            parent.spawn(NodeBundle {
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
                parent.spawn(NodeBundle {
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
                    parent.spawn(TextBundle::from_section(
                        store_query.single().coins.to_string(),
                        TextStyle {
                            font_size: 30.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ));
                });
                parent.spawn(NodeBundle {
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
                    parent.spawn(
                        NodeBundle {
                            background_color: Color::WHITE.into(),
                            ..default()
                        }
                    )
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

fn despawn_store(mut commands: Commands, query: Query<Entity, With<Store>>) {
    commands.entity(query.single()).despawn_recursive();
}
//! This example illustrates how to create a button that changes color and text based on its
//! interaction state.

use bevy::{prelude::*, winit::WinitSettings, input::common_conditions::input_toggle_active};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_text_popup::TextPopupPlugin;
mod buttons;

fn main() {
    let window_plugin = WindowPlugin {
        primary_window: Some(Window {
            title: String::from("Tic Tac Toe"),
            resize_constraints: WindowResizeConstraints { 
                max_width: 550.0,
                max_height: 600.0,
                ..default()
            },
            ..default()
        }),
        ..default()
        };

    App::new()
        .add_plugins((DefaultPlugins.set(window_plugin),TextPopupPlugin))
        .add_plugins(WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)))
        // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
        .insert_resource(WinitSettings::desktop_app())
        .add_systems(Startup, setup)
        .add_systems(Update, buttons::button_system)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // ui camera
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn((NodeBundle {
            style: Style {
                display: Display::Grid,
                justify_content: JustifyContent::Center,
                align_self: AlignSelf::Center,
                margin: UiRect::left(Val::Vw(20.0)),
                grid_template_columns: RepeatedGridTrack::flex(3, 1.0),
                // Set the grid to have 4 rows all with sizes minmax(0, 1fr)
                // This creates 4 exactly evenly sized rows
                grid_template_rows: RepeatedGridTrack::flex(3, 1.0),
                aspect_ratio: Some(1.0),
                align_content: AlignContent::Center,
                align_items: AlignItems::Center,
                row_gap: Val::Px(1.0),
                column_gap: Val::Px(1.0),
                ..default()
            },
            ..default()
        },buttons::game::Game::default()))
        .with_children(|parent| {
            parent
                .spawn((ButtonBundle {
                    style: Style {
                        width: Val::Px(110.0),
                        height: Val::Px(110.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    background_color: buttons::NORMAL_BUTTON.into(),
                    ..default()
                },buttons::Marker1))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        })
        .with_children(|parent| {
            parent
                .spawn((ButtonBundle {
                    style: Style {
                        width: Val::Px(110.0),
                        height: Val::Px(110.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    background_color: buttons::NORMAL_BUTTON.into(),
                    ..default()
                },buttons::Marker2))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        })
        .with_children(|parent| {
            parent
                .spawn((ButtonBundle {
                    style: Style {
                        width: Val::Px(110.0),
                        height: Val::Px(110.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    background_color: buttons::NORMAL_BUTTON.into(),
                    ..default()
                },buttons::Marker3))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        })
        .with_children(|parent| {
            parent
                .spawn((ButtonBundle {
                    style: Style {
                        width: Val::Px(110.0),
                        height: Val::Px(110.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    background_color: buttons::NORMAL_BUTTON.into(),
                    ..default()
                },buttons::Marker4))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        })
        .with_children(|parent| {
            parent
                .spawn((ButtonBundle {
                    style: Style {
                        width: Val::Px(110.0),
                        height: Val::Px(110.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    background_color: buttons::NORMAL_BUTTON.into(),
                    ..default()
                },buttons::Marker5))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        })
        .with_children(|parent| {
            parent
                .spawn((ButtonBundle {
                    style: Style {
                        width: Val::Px(110.0),
                        height: Val::Px(110.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    background_color: buttons::NORMAL_BUTTON.into(),
                    ..default()
                },buttons::Marker6))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        })
        .with_children(|parent| {
            parent
                .spawn((ButtonBundle {
                    style: Style {
                        width: Val::Px(110.0),
                        height: Val::Px(110.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    background_color: buttons::NORMAL_BUTTON.into(),
                    ..default()
                },buttons::Marker7))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        })
        .with_children(|parent| {
            parent
                .spawn((ButtonBundle {
                    style: Style {
                        width: Val::Px(110.0),
                        height: Val::Px(110.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    background_color: buttons::NORMAL_BUTTON.into(),
                    ..default()
                },buttons::Marker8))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        })
        .with_children(|parent| {
            parent
                .spawn((ButtonBundle {
                    style: Style {
                        width: Val::Px(110.0),
                        height: Val::Px(110.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    background_color: buttons::NORMAL_BUTTON.into(),
                    ..default()
                },buttons::Marker9))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        });
}
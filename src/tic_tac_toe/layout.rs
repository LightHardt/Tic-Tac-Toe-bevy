use bevy::prelude::*;

use super::components::*;
use super::game::*;
use super::styles::*;

fn grid_setup(commands: &mut Commands, asset_server: &Res<AssetServer>) {
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
        },Game::default()))
        .with_children(|parent| {
            parent
                .spawn((get_grid_button(), Marker1))
                .with_children(|parent| {
                    parent.spawn(get_grid_button_text(&asset_server));
                });
        })
        .with_children(|parent| {
            parent
                .spawn((get_grid_button(), Marker2))
                .with_children(|parent| {
                    parent.spawn(get_grid_button_text(&asset_server));
                });
        })
        .with_children(|parent| {
            parent
                .spawn((get_grid_button(), Marker3))
                .with_children(|parent| {
                    parent.spawn(get_grid_button_text(&asset_server));
                });
        })
        .with_children(|parent| {
            parent
                .spawn((get_grid_button(), Marker4))
                .with_children(|parent| {
                    parent.spawn(get_grid_button_text(&asset_server));
                });
        })
        .with_children(|parent| {
            parent
                .spawn((get_grid_button(), Marker5))
                .with_children(|parent| {
                    parent.spawn(get_grid_button_text(&asset_server));
                });
        })
        .with_children(|parent| {
            parent
                .spawn((get_grid_button(), Marker6))
                .with_children(|parent| {
                    parent.spawn(get_grid_button_text(&asset_server));
                });
        })
        .with_children(|parent| {
            parent
                .spawn((get_grid_button(), Marker7))
                .with_children(|parent| {
                    parent.spawn(get_grid_button_text(&asset_server));
                });
        })
        .with_children(|parent| {
            parent
                .spawn((get_grid_button(), Marker8))
                .with_children(|parent| {
                    parent.spawn(get_grid_button_text(&asset_server));
                });
        })
        .with_children(|parent| {
            parent
                .spawn((get_grid_button(), Marker9))
                .with_children(|parent| {
                    parent.spawn(get_grid_button_text(&asset_server));
                });
        });
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // ui camera
    commands.spawn(Camera2dBundle::default());
    
    // spawn buttons for game 3x3 grid
    grid_setup(&mut commands, &asset_server);

    // spawn button for ai move option
    commands
        .spawn(NodeBundle {
            style: Style { 
                display: Display::Flex,
                width: Val::Px(75.0),
                height: Val::Px(55.0),
                align_self: AlignSelf::Center,
                margin: UiRect::top(Val::Vh(70.0)),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn((ButtonBundle {
                    style: Style {
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        width: Val::Px(75.0),
                        height: Val::Px(55.0),
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    background_color: Color::PURPLE.into(),
                    ..default()
                }, AiMove))
                .with_children(|parent| {
                    parent
                        .spawn(TextBundle::from_section(
                            "Ai Move\nFirst",
                            TextStyle {
                                font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                                font_size: 15.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            }
                        ));
                });
        });               
            
}
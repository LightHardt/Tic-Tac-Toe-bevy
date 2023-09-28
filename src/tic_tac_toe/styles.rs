use bevy::prelude::*;

// Button Colors
pub const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);

// Grid button
pub fn get_grid_button() -> ButtonBundle{
    ButtonBundle {
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
        background_color: NORMAL_BUTTON.into(),
        ..default()
    }
}

// Grid button text
pub fn get_grid_button_text(asset_server: &Res<AssetServer>) -> TextBundle {
    TextBundle::from_section(
        "",
        TextStyle {
            font: asset_server.load("fonts/FiraMono-Medium.ttf"),
            font_size: 40.0,
            color: Color::rgb(0.9, 0.9, 0.9),
        },
    )
}
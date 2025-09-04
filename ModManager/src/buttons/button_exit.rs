use bevy::prelude::*;
use crate::{PreloadedAssets, buttons::{button_large_bundle, MainButton}};

#[derive(Component)]
pub struct ExitButton;

pub fn button_exit_init(
    mut commands: Commands,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    assets: Res<PreloadedAssets>
) {
    let dimensions = UVec2::new(134, 38);
    let layout: TextureAtlasLayout = TextureAtlasLayout::from_grid(dimensions, 2, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    commands.spawn((
        ExitButton,
        button_large_bundle(
            -554., 252., 0.,
            dimensions.as_vec2(),
            assets.get_audio("buttonLarge_doubleClick_SmartSoundFX_v2"),
            assets.get_image("spr_buttonMain"),
            texture_atlas_layout,
            "Exit",
            assets.get_font("CloisterBlack"),
            None
        )
    ));
}

pub fn button_exit_step(
    button: Single<&MainButton, With<ExitButton>>,
    mut app_exit_events: ResMut<Events<bevy::app::AppExit>>
) {
    if button.just_pressed {
        app_exit_events.send(AppExit::Success);
    }
}

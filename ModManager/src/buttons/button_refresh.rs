use bevy::prelude::*;
use crate::{PreloadedAssets, buttons::{button_small_bundle, MainButton}};

#[derive(Component)]
pub struct RefreshButton;

pub fn button_refresh_init(
    mut commands: Commands,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    assets: Res<PreloadedAssets>
) {
    let dimensions = UVec2::new(131, 23);
    let layout = TextureAtlasLayout::from_grid(dimensions, 2, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    commands.spawn((
        RefreshButton,
        button_small_bundle(
            // Since the origin is at the center, use half-coordinates for sprites with odd dimensions
            547.5, 328.5, 0.,
            dimensions.as_vec2(),
            assets.get_audio("buttonSmall_doubleClick1_SmartSoundFXPOND5"),
            assets.get_image("spr_buttonSmall"),
            texture_atlas_layout,
            "REFRESH",
            assets.get_image_font("pixel_font")
        )
    ));
}

pub fn button_refresh_step(
    button: Single<&MainButton, With<RefreshButton>>
) {
    if button.just_pressed {
        // LOGIC
    }
}

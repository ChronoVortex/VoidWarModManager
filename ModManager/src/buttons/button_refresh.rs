use bevy::prelude::*;
use crate::{buttons::{button_small_bundle, MainButton}, mods::{ModEntry, ModsLoad}, PreloadedAssets};

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
    mut commands: Commands,
    button: Single<&MainButton, With<RefreshButton>>,
    mod_entry_query: Query<Entity, With<ModEntry>>
) {
    if button.just_pressed {
        // Remove all mod entries
        for mod_entry in mod_entry_query {
            commands.entity(mod_entry.entity()).despawn();
        }

        // Reload mods
        commands.trigger(ModsLoad);
    }
}

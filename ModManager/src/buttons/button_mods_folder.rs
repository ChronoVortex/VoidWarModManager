use bevy::prelude::*;
use crate::{PreloadedAssets, buttons::{button_small_bundle, MainButton}, util::appdata_path};
use std::process::Command;

#[derive(Component)]
pub struct ModsFolderButton;

pub fn button_mods_folder_init(
    mut commands: Commands,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    assets: Res<PreloadedAssets>
) {
    let dimensions = UVec2::new(131, 23);
    let layout = TextureAtlasLayout::from_grid(dimensions, 2, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    commands.spawn((
        ModsFolderButton,
        button_small_bundle(
            // Since the origin is at the center, use half-coordinates for sprites with odd dimensions
            370.5, 328.5, 0.,
            dimensions.as_vec2(),
            assets.get_audio("buttonSmall_doubleClick1_SmartSoundFXPOND5"),
            assets.get_image("spr_buttonSmall"),
            texture_atlas_layout,
            "MODS FOLDER",
            assets.get_image_font("pixel_font")
        )
    ));
}

pub fn button_mods_folder_step(
    button: Single<&MainButton, With<ModsFolderButton>>
) {
    if button.just_pressed {
        Command::new("explorer")
            .arg(appdata_path("Void_War\\mods")) 
            .spawn()
            .unwrap();
    }
}

use bevy::prelude::*;
use crate::{PreloadedAssets, buttons::{button_small_bundle, MainButton}, log::LogManager, util::vwdata_path};

#[derive(Component)]
pub struct ExportButton;

pub fn button_export_init(
    mut commands: Commands,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    assets: Res<PreloadedAssets>
) {
    let dimensions = UVec2::new(131, 23);
    let layout = TextureAtlasLayout::from_grid(dimensions, 2, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    commands.spawn((
        ExportButton,
        button_small_bundle(
            // Since the origin is at the center, use half-coordinates for sprites with odd dimensions
            -397.5, 190.5, 0.,
            dimensions.as_vec2(),
            assets.get_audio("buttonSmall_doubleClick1_SmartSoundFXPOND5"),
            assets.get_image("spr_buttonSmall"),
            texture_atlas_layout,
            "EXPORT",
            assets.get_image_font("pixel_font")
        )
    ));
}

pub fn button_export_step(
    button: Single<&MainButton, With<ExportButton>>,
    log_man: Single<&LogManager>
) {
    if button.just_pressed {
        if let Some(path) = rfd::FileDialog::new()
            .set_file_name("vw-mod-log.txt")
            .set_directory(vwdata_path(""))
            .save_file() {
            let _ = std::fs::write(path, log_man.strings.join("\n"));
        }
    }
}

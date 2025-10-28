use std::fs;
use bevy::prelude::*;
use crate::{
    buttons::{button_large_bundle, confirm_popup_bundle, ConfirmPopup, MainButton}, log::LogManager, mods::ModLibrary, util::{appdata_path, local_path}, PreloadedAssets
};

#[derive(Component)]
pub struct BackupButton;

pub fn button_backup_init(
    mut commands: Commands,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    assets: Res<PreloadedAssets>
) {
    let dimensions = UVec2::new(134, 38);
    let layout = TextureAtlasLayout::from_grid(dimensions, 2, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    commands.spawn((
        BackupButton,
        button_large_bundle(
            -554., 321., 0.,
            dimensions.as_vec2(),
            assets.get_audio("buttonLarge_doubleClick_SmartSoundFX_v2"),
            assets.get_image("spr_buttonMain"),
            texture_atlas_layout,
            "New Backup",
            assets.get_font("CloisterBlack"),
            Some(22.)
        )
    ));
}

#[derive(Component)]
pub struct BackupButtonConfirm;

pub fn button_backup_step(
    mut button: Single<&mut MainButton, With<BackupButton>>,
    mut buttons_other: Query<&mut MainButton, Without<BackupButton>>,
    mut mod_library: Single<&mut ModLibrary>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    assets: Res<PreloadedAssets>
) {
    if button.just_pressed {
        // Disable all buttons that aren't part of the confirmation box
        button.active = false;
        for mut button_other in &mut buttons_other {
            button_other.active = false;
        }
        mod_library.buttons_active = false;

        // Spawn confirmation box
        let dimensions = UVec2::new(134, 38);
        let layout = TextureAtlasLayout::from_grid(dimensions, 2, 1, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);
        commands.spawn(confirm_popup_bundle(
            meshes.add(Rectangle::new(1280., 720.)),
            materials.add(Color::srgba(0., 0., 0., 0.96)),
            assets.get_image("spr_confirmBox"),
            "Are you sure? You should only do this on a fresh\ninstallation with only the mod manager installed!",
            assets.get_font("dubellay"),
            dimensions.as_vec2(),
            assets.get_audio("buttonLarge_doubleClick_SmartSoundFX_v2"),
            assets.get_image("spr_buttonMain"),
            texture_atlas_layout,
            assets.get_font("CloisterBlack"),
            BackupButtonConfirm
        ));
    }
}

pub fn button_backup_confirm_step(
    mut commands: Commands,
    button: Single<&MainButton, With<BackupButtonConfirm>>,
    mut buttons_other: Query<&mut MainButton, Without<BackupButtonConfirm>>,
    mut log_man: Single<&mut LogManager>,
    popup: Single<Entity, With<ConfirmPopup>>
) {
    if button.just_pressed {
        // Create new backup
        if !fs::exists(appdata_path("Void_War\\.DATA_BAK")).expect("Unable to check for backup folder") {
            fs::create_dir(appdata_path("Void_War\\.DATA_BAK")).expect("Unable to create backup folder");
        }
        fs::copy(
            local_path("data.win"), 
            appdata_path("Void_War\\.DATA_BAK\\data.win")
        ).expect("Unable to create backup");

        // Log the backup
        log_man.log("Created new data.win backup".to_string(), None);

        // Enable all buttons
        for mut button_other in &mut buttons_other {
            button_other.active = true;
        }

        // Despawn the confirmation popup
        commands.entity(popup.entity()).despawn();
    }
}

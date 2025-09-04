use bevy::prelude::*;
use crate::{
    PreloadedAssets,
    buttons::{button_large_bundle, confirm_popup_bundle, MainButton},
    util::{local_path, run_program}
};

#[derive(Component)]
pub struct ReturnButton;

pub fn button_return_init(
    mut commands: Commands,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    assets: Res<PreloadedAssets>
) {
    let dimensions = UVec2::new(134, 38);
    let layout = TextureAtlasLayout::from_grid(dimensions, 2, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    commands.spawn((
        ReturnButton,
        button_large_bundle(
            -390., 252., 0.,
            dimensions.as_vec2(),
            assets.get_audio("buttonLarge_doubleClick_SmartSoundFX_v2"),
            assets.get_image("spr_buttonMain"),
            texture_atlas_layout,
            "Return to Game",
            assets.get_font("CloisterBlack"),
            Some(18.)
        )
    ));
}

#[derive(Component)]
pub struct ReturnButtonConfirm;

pub fn button_return_step(
    mut button: Single<&mut MainButton, With<ReturnButton>>,
    mut buttons_other: Query<&mut MainButton, Without<ReturnButton>>,
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

        // Spawn confirmation box
        let dimensions = UVec2::new(134, 38);
        let layout = TextureAtlasLayout::from_grid(dimensions, 2, 1, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);
        commands.spawn(confirm_popup_bundle(
            meshes.add(Rectangle::new(1280., 720.)),
            materials.add(Color::srgba(0., 0., 0., 0.96)),
            assets.get_image("spr_confirmBox"),
            "This will close the mod manager\nand restart the game.",
            assets.get_font("dubellay"),
            dimensions.as_vec2(),
            assets.get_audio("buttonLarge_doubleClick_SmartSoundFX_v2"),
            assets.get_image("spr_buttonMain"),
            texture_atlas_layout,
            assets.get_font("CloisterBlack"),
            ReturnButtonConfirm
        ));
    }
}

pub fn button_return_confirm_step(
    button: Single<&MainButton, With<ReturnButtonConfirm>>,
    mut app_exit_events: ResMut<Events<bevy::app::AppExit>>
) {
    if button.just_pressed {
        run_program(local_path("Void War.exe").as_str());
        app_exit_events.send(AppExit::Success);
    }
}

use bevy::{prelude::*, sprite::Anchor};
use bevy_image_font::{
    ImageFontText, LetterSpacing,
    atlas_sprites::ImageFontSpriteText
};
use crate::{PreloadedAssets, AppState};

#[derive(Component)]
pub struct ModLog;

fn log_init(
    mut commands: Commands,
    assets: Res<PreloadedAssets>
) {
    commands.spawn((
        ModLog,
        ImageFontSpriteText::default()
            .anchor(Anchor::TopLeft)
            .color(Color::srgb_u8(155, 153, 139))
            .letter_spacing(LetterSpacing::Pixel(1)),
        ImageFontText::default()
            .text("Test content for log")
            .font(assets.get_image_font("pixel_font")),
        Transform::from_xyz(-618., 140., -100.)
    ));
}

pub struct LogPlugin;
impl Plugin for LogPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(AppState::Loading), log_init);
    }
}

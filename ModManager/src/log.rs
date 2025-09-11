use std::collections::VecDeque;
use bevy::{prelude::*, sprite::Anchor};
use bevy_image_font::{
    ImageFontText, LetterSpacing,
    atlas_sprites::ImageFontSpriteText
};
use crate::{PreloadedAssets, AppState};

#[derive(Component)]
pub struct LogManager {
    strings: VecDeque<(String, Color)>,
    pos_x: f32,
    pos_y: f32,
    pos_y_start: f32,
    pos_y_end: f32,
    spacing: f32

}
impl LogManager {
    fn new(pos_x: f32, pos_y: f32, spacing: f32, height: f32) -> Self {
        LogManager {
            strings: VecDeque::new(),
            pos_x: pos_x,
            pos_y: pos_y,
            pos_y_start: pos_y,
            pos_y_end: pos_y - height + (height % spacing),
            spacing: spacing
        }
    }

    pub fn log(&mut self, string: String, color: Option<Color>) {
        self.strings.push_back((
            string,
            color.unwrap_or(Color::srgb_u8(155, 153, 139))
        ));
    }
}

#[derive(Component)]
pub struct LogText;

fn log_init(
    mut commands: Commands
) {
    commands.spawn(LogManager::new(-615., 138., 13., 470.));
}

fn log_step(
    mut log_man: Single<&mut LogManager>,
    mut log_texts: Query<(Entity, &mut Transform), With<LogText>>,
    mut commands: Commands,
    assets: Res<PreloadedAssets>
) {
    if !log_man.strings.is_empty() {
        // Determine how far the texts need to be scrolled up to make space for new lines
        let new_log_height: f32 = log_man.pos_y_start - log_man.pos_y + (log_man.strings.len() as f32)*log_man.spacing;
        let max_log_height: f32 = log_man.pos_y_start - log_man.pos_y_end;
        let scroll_text: f32 = (new_log_height - max_log_height).max(0.);

        // Spawn new texts
        let mut pos_y = log_man.pos_y + scroll_text;
        while !log_man.strings.is_empty() {
            let string_data = log_man.strings.pop_front().unwrap();
            commands.spawn((
                LogText,
                ImageFontSpriteText::default()
                    .anchor(Anchor::TopLeft)
                    .color(string_data.1)
                    .letter_spacing(LetterSpacing::Pixel(1)),
                ImageFontText::default()
                    .text(string_data.0)
                    .font(assets.get_image_font("pixel_font")),
                Transform::from_xyz(log_man.pos_x, pos_y, 0.)
            ));
            pos_y -= log_man.spacing;
        }
        log_man.pos_y = pos_y;

        // Scroll up texts, despawn the ones that have scrolled beyond the log window
        if scroll_text > 0. {
            for (log_text, mut log_text_pos) in &mut log_texts {
                log_text_pos.translation.y += scroll_text;
                if log_text_pos.translation.y > log_man.pos_y_start {
                    commands.entity(log_text.entity()).despawn();
                }
            }
        }
    }
}

pub struct LogPlugin;
impl Plugin for LogPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(AppState::Loading), log_init);
        app.add_systems(Update, log_step);
    }
}

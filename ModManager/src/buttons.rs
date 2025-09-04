use bevy::{
    prelude::*,
    audio::Volume,
    color::palettes::basic::BLACK
};
use bevy_image_font::{
    ImageFont, ImageFontText, LetterSpacing,
    atlas_sprites::ImageFontSpriteText
};
use crate::AppState;
use crate::cursor::Cursor;
use crate::settings::Settings;

mod button_backup;
use crate::buttons::button_backup::{button_backup_init, button_backup_step, button_backup_confirm_step};

mod button_patch;
use crate::buttons::button_patch::{button_patch_init, button_patch_step};

mod button_exit;
use crate::buttons::button_exit::{button_exit_init, button_exit_step};

mod button_return;
use crate::buttons::button_return::{button_return_init, button_return_step, button_return_confirm_step};

mod button_mods_folder;
use crate::buttons::button_mods_folder::{button_mods_folder_init, button_mods_folder_step};

mod button_refresh;
use crate::buttons::button_refresh::{button_refresh_init, button_refresh_step};

mod button_export;
use crate::buttons::button_export::{button_export_init, button_export_step};

#[derive(Component)]
struct MainButton {
    pub size: Vec2,
    pub sound: Handle<AudioSource>,
    pub pressed: bool,
    pub just_pressed: bool,
    pub active: bool
}
impl MainButton {
    fn new(size: Vec2, sound: Handle<AudioSource>) -> Self {
        MainButton {
            size: size,
            sound: sound,
            pressed: false,
            just_pressed: false,
            active: true
        }
    }
}

fn button_large_bundle(
    pos_x: f32,
    pos_y: f32,
    pos_z: f32,
    dimensions: Vec2,
    sound: Handle<AudioSource>,
    texture: Handle<Image>,
    texture_atlas_layout: Handle<TextureAtlasLayout>,
    text: &str,
    font: Handle<Font>,
    font_size: Option<f32>
) -> impl Bundle {
    (
        MainButton::new(
            dimensions,
            sound
        ),
        Sprite::from_atlas_image(
            texture,
            TextureAtlas {
                layout: texture_atlas_layout,
                index: 0,
            },
        ),
        Text2d::new(text),
        TextFont {
            font: font,
            font_size: font_size.unwrap_or(26.),
            ..default()
        },
        TextColor(BLACK.into()),
        TextLayout::new_with_justify(JustifyText::Center),
        Transform::from_xyz(pos_x, pos_y, pos_z)
    )
}

fn button_small_bundle(
    pos_x: f32,
    pos_y: f32,
    pos_z: f32,
    dimensions: Vec2,
    sound: Handle<AudioSource>,
    texture: Handle<Image>,
    texture_atlas_layout: Handle<TextureAtlasLayout>,
    text: &str,
    font: Handle<ImageFont>
) -> impl Bundle {
    (
        MainButton::new(
            dimensions,
            sound
        ),
        Sprite::from_atlas_image(
            texture,
            TextureAtlas {
                layout: texture_atlas_layout,
                index: 0,
            },
        ),
        Transform::from_xyz(pos_x, pos_y, pos_z),
        children![(
            ImageFontSpriteText::default()
                .color(BLACK)
                .letter_spacing(LetterSpacing::Pixel(1)),
            ImageFontText::default()
                .text(text)
                .font(font),
            Transform::from_xyz(0., 0., 0.)
        )]
    )
}

#[derive(Component)]
struct CancelButton;

#[derive(Component)]
struct ConfirmPopup;

fn confirm_popup_bundle(
    bg: Handle<Mesh>,
    bg_material: Handle<ColorMaterial>,
    texture: Handle<Image>,
    text: &str,
    font: Handle<Font>,
    button_dimensions: Vec2,
    button_sound: Handle<AudioSource>,
    button_texture: Handle<Image>,
    button_texture_atlas_layout: Handle<TextureAtlasLayout>,
    button_font: Handle<Font>,
    confirm_tag: impl Component
) -> impl Bundle {
    (
        ConfirmPopup,
        Mesh2d(bg),
        MeshMaterial2d(bg_material),
        Transform::from_xyz(0., 0., 0.),
        children![(
            Sprite::from_image(texture),
            Transform::from_xyz(0., 0., 10.)
        ), (
            Text2d::new(text),
            TextFont {
                font: font,
                font_size: 16.,
                ..default()
            },
            TextColor(BLACK.into()),
            TextLayout::new_with_justify(JustifyText::Center),
            Transform::from_xyz(0., 50., 10.)
        ), (
            confirm_tag,
            button_large_bundle(
                -93., -28., 20.,
                button_dimensions,
                button_sound.clone(),
                button_texture.clone(),
                button_texture_atlas_layout.clone(),
                "Confirm",
                button_font.clone(),
                None
            )
        ), (
            CancelButton,
            button_large_bundle(
                91., -28., 20.,
                button_dimensions,
                button_sound,
                button_texture,
                button_texture_atlas_layout,
                "Cancel",
                button_font,
                None
            )
        )]
    )
}

fn button_step(
    mut commands: Commands,
    mut button_query: Query<(&mut MainButton, &mut Sprite, &Transform)>,
    cursor: Single<&Cursor>,
    mouse: Res<ButtonInput<MouseButton>>,
    settings: Res<Settings>
) {
    for (mut button, mut sprite, transform) in &mut button_query {
        let atlas = sprite.texture_atlas.get_or_insert_default();

        // Reset state before executing logic
        atlas.index = 0;
        button.just_pressed = false;
        if !(button.active && mouse.pressed(MouseButton::Left))  {
            button.pressed = false;
        }
        
        if button.active {
            // Get the bounding box of the button's sprite
            let image_size = button.size;
            let scaled = image_size*transform.scale.truncate();
            let bounding_box =
                Rect::from_center_size(transform.translation.truncate(), scaled);

            // Check if the cursor position is in the bounding box
            if bounding_box.contains(cursor.pos) {
                // Show hover frame
                atlas.index = 1;
                
                if mouse.just_pressed(MouseButton::Left) {
                    // Mark button as pressed
                    button.pressed = true;
                    button.just_pressed = true;

                    // Play press sound
                    commands.spawn((
                        AudioPlayer::new(button.sound.clone()),
                        PlaybackSettings {
                            volume: Volume::Linear(settings.volume_max_sfx),
                            ..default()
                        },
                    ));
                }
            }
        }
    }
}

fn button_cancel_step(
    mut commands: Commands,
    button: Single<&MainButton, With<CancelButton>>,
    mut buttons_other: Query<&mut MainButton, Without<CancelButton>>,
    popup: Single<Entity, With<ConfirmPopup>>
) {
    if button.just_pressed {
        // Enable all buttons
        for mut button_other in &mut buttons_other {
            button_other.active = true;
        }

        // Despawn the confirmation popup
        commands.entity(popup.entity()).despawn();
    }
}

pub struct ButtonsPlugin;
impl Plugin for ButtonsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(AppState::Loading), (
            button_backup_init,
            button_patch_init,
            button_exit_init,
            button_return_init,
            button_mods_folder_init,
            button_refresh_init,
            button_export_init
        ));
        app.add_systems(Update, (
            button_backup_step,
            button_backup_confirm_step,
            button_patch_step,
            button_exit_step,
            button_return_step,
            button_return_confirm_step,
            button_mods_folder_step,
            button_refresh_step,
            button_export_step
        ));
        app.add_systems(Update, (
            button_step,
            button_cancel_step
        ));
    }
}

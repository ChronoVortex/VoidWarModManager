use std::fs;
use bevy::{prelude::*, sprite::Anchor, text::TextBounds};
use bevy_image_font::{ImageFontText, LetterSpacing, atlas_sprites::ImageFontSpriteText};
use serde::Deserialize;
use image::image_dimensions;
use crate::{AppState, PreloadedAssets, buttons::button_bundle, util::appdata_path};

#[derive(Deserialize)]
#[serde(default)]
struct ModInfo {
    title: String,
    version_major: u32,
    version_minor: u32,
    version_patch: u32,
    authors: Vec<String>,
    description: String
}
impl Default for ModInfo {
    fn default() -> Self {
        ModInfo {
            title: "".to_string(),
            version_major: 0,
            version_minor: 0,
            version_patch: 0,
            authors: vec!["Unknown".to_string()],
            description: "No description provided.".to_string()
        }
    }
}

#[derive(Component)]
pub struct ModEntry {
    pub path: String
}
impl ModEntry {
    fn new(path: String) -> Self {
        ModEntry {
            path: path
        }
    }
}

fn load_mods(
    mut commands: Commands,
    mut next_state: ResMut<NextState<AppState>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
    assets: Res<PreloadedAssets>
) {
    let mods_dir = appdata_path("Void_War\\mods");
    if fs::exists(mods_dir.clone()).expect("Unable to check for mods folder") {
        // Walk through each folder in the mods folder
        for mod_dir_entry in fs::read_dir(mods_dir).expect("Unable to read mods folder") {
            let mod_dir = mod_dir_entry.unwrap();
            if mod_dir.file_type().unwrap().is_dir() {
                let mod_path = String::from(mod_dir.path().as_path().to_str().unwrap());

                // Ensure the folder has a project file
                if fs::exists(mod_path.clone() + "\\project.json").expect("Unable to check for project file") {
                    // Check if the mod preview image exists and is the correct size, otherwise use the missing preview icon
                    let preview_path = mod_path.clone() + "\\preview.png";
                    let preview_sprite =
                        if let Ok(preview_size) = image_dimensions(preview_path.clone()) && preview_size.0 <= 168 && preview_size.1 <= 120
                        { Sprite::from_image(asset_server.load(String::from("file://") + preview_path.as_str())) } else
                        { Sprite::from_image(assets.get_image("spr_missing")) };
                    
                    // Load metadata
                    let mut metadata = 
                        if let Ok(metadata_str) = fs::read_to_string(mod_path.clone() + "\\metadata.json")
                        && let Ok(metadata_res) = serde_json::from_str::<ModInfo>(&metadata_str) {
                            metadata_res
                        } else {
                            ModInfo::default()
                        };
                    if metadata.title.is_empty() {
                        metadata.title = String::from(mod_dir.path().as_path().file_name().unwrap().to_str().unwrap());
                    }

                    // Create the mod entry
                    // TODO: fix button_step system not working on button children, implement ordering functionality
                    let dimensions_toggle = UVec2::new(16, 16);
                    let layout_toggle = TextureAtlasLayout::from_grid(dimensions_toggle, 2, 1, None, None);
                    let texture_atlas_layout_toggle = texture_atlas_layouts.add(layout_toggle);
                    let dimensions_arrows = UVec2::new(16, 14);
                    let layout_arrows = TextureAtlasLayout::from_grid(dimensions_arrows, 2, 1, None, None);
                    let texture_atlas_layout_arrows = texture_atlas_layouts.add(layout_arrows);
                    let text_color_light = Color::srgb_u8(155, 153, 139);
                    let text_color_dark = Color::srgb_u8(64, 64, 64);
                    let text_start_x: f32 = -205.;
                    let text_start_y: f32 = 53.;
                    let text_width: f32 = 620.;
                    commands.spawn((
                        ModEntry::new(mod_path),
                        Transform::from_xyz(146., 206., -100.),
                        Visibility::default(),
                        children![button_bundle(
                            // Toggle button
                            -407., 0., 0., true,
                            dimensions_toggle.as_vec2(),
                            assets.get_audio("vs_ui_click1"),
                            assets.get_image("spr_modButton_toggle"),
                            texture_atlas_layout_toggle
                        ), button_bundle(
                            // Up arrow button
                            -407., 25., 0., false,
                            dimensions_arrows.as_vec2(),
                            assets.get_audio("vs_ui_click1"),
                            assets.get_image("spr_modButton_up"),
                            texture_atlas_layout_arrows.clone()
                        ), button_bundle(
                            // Down arrow button
                            -407., -25., 0., false,
                            dimensions_arrows.as_vec2(),
                            assets.get_audio("vs_ui_click1"),
                            assets.get_image("spr_modButton_down"),
                            texture_atlas_layout_arrows
                        ), (
                            // Preview image
                            preview_sprite,
                            Transform::from_xyz(-302., 0., 0.)
                        ), (
                            // Preview frame
                            Sprite::from_image(assets.get_image("spr_modFrame")),
                            Transform::from_xyz(-302., 0., 1.)
                        ), (
                            // Title text
                            Text2d::new(metadata.title),
                            TextFont {
                                font: assets.get_font("dubellay"),
                                font_size: 16.,
                                ..default()
                            },
                            TextColor(text_color_light.clone()),
                            TextBounds::new(text_width, 20.),
                            Anchor::TopLeft,
                            Transform::from_xyz(text_start_x, text_start_y, 0.)
                        ), (
                            // Version and authors text
                            ImageFontSpriteText::default()
                                .color(text_color_dark)
                                .letter_spacing(LetterSpacing::Pixel(1))
                                .anchor(Anchor::TopLeft),
                            ImageFontText::default()
                                .text(format!(
                                    "VERSION: {}.{}.{}   AUTHORS: {}",
                                    metadata.version_major,
                                    metadata.version_minor,
                                    metadata.version_patch,
                                    metadata.authors.join(", ")
                                ))
                                .font(assets.get_image_font("pixel_font")),
                            Transform::from_xyz(text_start_x, text_start_y - 24., 0.)
                        ), (
                            // Horizontal rule
                            Mesh2d(meshes.add(Rectangle::new(text_width, 1.))),
                            MeshMaterial2d(materials.add(text_color_light.clone())),
                            Transform::from_xyz(text_start_x + text_width/2., text_start_y - 42., 0.)
                        ), (
                            // Description
                            Text2d::new(metadata.description),
                            TextFont {
                                font: assets.get_font("dubellay"),
                                font_size: 16.,
                                ..default()
                            },
                            TextColor(text_color_light),
                            TextBounds::new(text_width, 60.),
                            Anchor::TopLeft,
                            Transform::from_xyz(text_start_x, text_start_y - 51., 0.)
                        )]
                    ));
                }
            }
        }
    } else {
        fs::create_dir(appdata_path("Void_War\\mods")).expect("Unable to create mods folder");
    }
    next_state.set(AppState::Running);
}

pub struct ModsPlugin;
impl Plugin for ModsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::LoadingMods), load_mods);
    }
}

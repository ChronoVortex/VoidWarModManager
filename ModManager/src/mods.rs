use std::fs;
use bevy::{prelude::*, sprite::Anchor, text::TextBounds};
use bevy_image_font::{ImageFontText, LetterSpacing, atlas_sprites::ImageFontSpriteText};
use serde::Deserialize;
use image::image_dimensions;
use crate::{buttons::{button_bundle, MainButton}, util::appdata_path, AppState, PreloadedAssets};

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
    pub path: String,
    pub index: i32
}
impl ModEntry {
    fn new(path: String, index: i32) -> Self {
        ModEntry {
            path: path,
            index: index
        }
    }
}

#[derive(Component)]
pub struct ModToggleButton;

#[derive(Component)]
pub struct ModUpButton;

#[derive(Component)]
pub struct ModDownButton;

#[derive(Event)]
struct ModOrderChanged;

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

        // Collect all mods in the mod folder
        let mut mod_paths: Vec<String> = Vec::new();
        for mod_dir_entry in fs::read_dir(mods_dir).expect("Unable to read mods folder") {
            let mod_dir = mod_dir_entry.unwrap();
            if mod_dir.file_type().unwrap().is_dir() {
                let mod_path = String::from(mod_dir.path().as_path().to_str().unwrap());

                // Ensure the folder has a project file
                if fs::exists(mod_path.clone() + "\\project.json").expect("Unable to check for project file") {
                    mod_paths.push(mod_path);
                }
            }
        }

        // Apply saved mod order
        if let Ok(mod_order_json) = fs::read_to_string(appdata_path("Void_War\\mods\\order.json"))
        && let Ok(mod_order) = serde_json::from_str::<Vec<String>>(&mod_order_json) {
            let mut mod_paths_new: Vec<String> = Vec::with_capacity(mod_paths.len());
            let mut mod_paths_marks = vec![false; mod_paths.len()];
            let mut mod_order_marks = vec![false; mod_order.len()];

            // Mark entires to be moved to new path list
            for mod_paths_index in 0..mod_paths.len() {
                let mut missing_from_order = true;
                for mod_order_index in 0..mod_order.len() {
                    if mod_paths[mod_paths_index] == mod_order[mod_order_index] {
                        mod_order_marks[mod_order_index] = true;
                        missing_from_order = false;
                        break;
                    }
                }
                if missing_from_order {
                    mod_paths_marks[mod_paths_index] = true;
                }
            }

            // Move entries to new path list
            for mod_paths_index in 0..mod_paths.len() {
                if mod_paths_marks[mod_paths_index] {
                    mod_paths_new.push(mod_paths[mod_paths_index].clone());
                }
            }
            for mod_order_index in 0..mod_order.len() {
                if mod_order_marks[mod_order_index] {
                    mod_paths_new.push(mod_order[mod_order_index].clone());
                }
            }

            // Assign new path list
            mod_paths = mod_paths_new;
        }

        // Save mod order
        if let Ok(mod_order_json) = serde_json::to_string(&mod_paths) {
            let _ = std::fs::write(appdata_path("Void_War\\mods\\order.json"), mod_order_json);
        }

        // Walk through each mod
        let mut vertical_offset: f32 = 206.;
        let vertical_spacing: f32 = 136.;
        let mut mod_entry_index = 0;
        for mod_path in mod_paths {
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
                metadata.title = String::from(mod_path.get(mod_path.rfind("\\").unwrap() + 1..).unwrap());
            }

            // Create the mod entry
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
                ModEntry::new(mod_path, mod_entry_index),
                Transform::from_xyz(146., vertical_offset, -100.),
                Visibility::default(),
                children![(
                    // Toggle button
                    ModToggleButton,
                    button_bundle(
                        -407., 0., 0., true,
                        dimensions_toggle.as_vec2(),
                        assets.get_audio("vs_ui_click1"),
                        assets.get_image("spr_modButton_toggle"),
                        texture_atlas_layout_toggle
                    )
                ), (
                    // Up arrow button
                    ModUpButton,
                    button_bundle(
                        -407., 25., 0., false,
                        dimensions_arrows.as_vec2(),
                        assets.get_audio("vs_ui_click1"),
                        assets.get_image("spr_modButton_up"),
                        texture_atlas_layout_arrows.clone()
                    )
                ), (
                    // Down arrow button
                    ModDownButton,
                    button_bundle(
                        -407., -25., 0., false,
                        dimensions_arrows.as_vec2(),
                        assets.get_audio("vs_ui_click1"),
                        assets.get_image("spr_modButton_down"),
                        texture_atlas_layout_arrows
                    )
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

            mod_entry_index += 1;
            vertical_offset -= vertical_spacing;
        }
    } else {
        fs::create_dir(appdata_path("Void_War\\mods")).expect("Unable to create mods folder");
    }
    next_state.set(AppState::Running);
}

pub fn button_mod_arrow_step(
    mut commands: Commands,
    mut mod_entry_query: Query<(&mut ModEntry, &mut Transform, &Children)>,
    button_up_query: Query<&MainButton, With<ModUpButton>>,
    button_down_query: Query<&MainButton, With<ModDownButton>>
) {
    // Iterate through every combination of two mod entries to check if we need to swap any
    let mut iter = mod_entry_query.iter_combinations_mut();
    while let Some([
        (mut mod_entry_1, mut transform_1, children_1),
        (mut mod_entry_2, mut transform_2, children_2)
    ]) = iter.fetch_next() {
        for child in children_1.iter() {
            // Swap mod_entry_1 with the entry above if its up arrow was pressed
            if mod_entry_2.index + 1 == mod_entry_1.index
            && let Ok(button) = button_up_query.get(child)
            && button.just_pressed {
                std::mem::swap(&mut mod_entry_1.index, &mut mod_entry_2.index);
                std::mem::swap(&mut transform_1.translation.y, &mut transform_2.translation.y);
                commands.trigger(ModOrderChanged);
                return;
            }

            // Swap mod_entry_1 with the entry below if its down arrow was pressed
            if mod_entry_2.index - 1 == mod_entry_1.index
            && let Ok(button) = button_down_query.get(child)
            && button.just_pressed {
                std::mem::swap(&mut mod_entry_1.index, &mut mod_entry_2.index);
                std::mem::swap(&mut transform_1.translation.y, &mut transform_2.translation.y);
                commands.trigger(ModOrderChanged);
                return;
            }
        }
        for child in children_2.iter() {
            // Swap mod_entry_2 with the entry above if its up arrow was pressed
            if mod_entry_1.index + 1 == mod_entry_2.index
            && let Ok(button) = button_up_query.get(child)
            && button.just_pressed {
                std::mem::swap(&mut mod_entry_1.index, &mut mod_entry_2.index);
                std::mem::swap(&mut transform_1.translation.y, &mut transform_2.translation.y);
                commands.trigger(ModOrderChanged);
                return;
            }

            // Swap mod_entry_2 with the entry below if its down arrow was pressed
            if mod_entry_1.index - 1 == mod_entry_2.index
            && let Ok(button) = button_down_query.get(child)
            && button.just_pressed {
                std::mem::swap(&mut mod_entry_1.index, &mut mod_entry_2.index);
                std::mem::swap(&mut transform_1.translation.y, &mut transform_2.translation.y);
                commands.trigger(ModOrderChanged);
                return;
            }
        }
    }
}

fn save_mod_order(
	_: Trigger<ModOrderChanged>,
	mod_entry_query: Query<&ModEntry>
) {
    // Create a vector for the mod order
    let mut mod_paths = vec![""; mod_entry_query.iter().count()];
    for mod_entry in mod_entry_query {
        mod_paths[mod_entry.index as usize] = mod_entry.path.as_str();
    }

    // Save mod order
    if let Ok(mod_order_json) = serde_json::to_string(&mod_paths) {
        let _ = std::fs::write(appdata_path("Void_War\\mods\\order.json"), mod_order_json);
    }
}

pub struct ModsPlugin;
impl Plugin for ModsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::LoadingMods), load_mods);
        app.add_systems(Update, button_mod_arrow_step);
        app.add_observer(save_mod_order);
    }
}

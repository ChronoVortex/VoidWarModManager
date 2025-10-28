use bevy::prelude::*;
use crate::{AppState, PreloadedAssets, buttons::{MainButton, button_bundle}, cursor::Cursor, mods::{ModDownButton, ModLibrary, ModToggleButton, ModUpButton}};

#[derive(Component)]
pub struct ScrollButton {
    pub scroll_height: f32,
    pub start_y: f32,
    pub y_selected: f32
}
impl ScrollButton {
    fn new(scroll_height: f32, start_y: f32) -> ScrollButton {
        ScrollButton {
            scroll_height: scroll_height,
            start_y: start_y,
            y_selected: 0.
        }
    }
}

#[derive(Component)]
struct ScrollUpButton;

#[derive(Component)]
struct ScrollDownButton;

fn button_scroll_init(
    mut commands: Commands,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    assets: Res<PreloadedAssets>
) {
    let dimensions = UVec2::new(22, 50);
    let layout = TextureAtlasLayout::from_grid(dimensions, 2, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    commands.spawn((
        ScrollButton::new(496., 224.),
        button_bundle(
            615., 224., 0., None,
            dimensions.as_vec2(),
            assets.get_audio("vs_ui_click1"),
            assets.get_image("spr_buttonScroll"),
            texture_atlas_layout
        )
    ));
}

fn button_scroll_up_init(
    mut commands: Commands,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    assets: Res<PreloadedAssets>
) {
    let dimensions = UVec2::new(22, 22);
    let layout = TextureAtlasLayout::from_grid(dimensions, 2, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    commands.spawn((
        ScrollUpButton,
        button_bundle(
            615., 269., 0., None,
            dimensions.as_vec2(),
            assets.get_audio("vs_ui_click1"),
            assets.get_image("spr_buttonScrollUp"),
            texture_atlas_layout
        )
    ));
}

fn button_scroll_down_init(
    mut commands: Commands,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    assets: Res<PreloadedAssets>
) {
    let dimensions = UVec2::new(22, 22);
    let layout = TextureAtlasLayout::from_grid(dimensions, 2, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    commands.spawn((
        ScrollDownButton,
        button_bundle(
            615., -320., 0., None,
            dimensions.as_vec2(),
            assets.get_audio("vs_ui_click1"),
            assets.get_image("spr_buttonScrollDown"),
            texture_atlas_layout
        )
    ));
}

fn buttons_manage_active(
    mut button_query: Query<(&mut MainButton, &GlobalTransform), Or<(With<ModToggleButton>, With<ModUpButton>, With<ModDownButton>)>>,
    mod_library: Single<&ModLibrary>
) {
    // Only manage mod buttons if all buttons are enabled
    if mod_library.buttons_active {
        for (mut button, transform) in &mut button_query {
            // Deactivate mod buttons which aren't visible
            button.active = mod_library.window_rect.contains(transform.translation().truncate());
        }
    }
}

fn button_scroll_step(
    mod_library_query: Single<(&ModLibrary, &mut Transform)>,
    button_query: Single<(&MainButton, &mut ScrollButton, &mut Transform), Without<ModLibrary>>,
    cursor: Single<&Cursor>
) {
    // Only do scrolling if mod list extends outside the mod window
    let (mod_library, mut mod_library_transform) = mod_library_query.into_inner();
    let mod_library_view_height = mod_library.window_height - 2.*mod_library.window_padding;
    if mod_library.mods_height > mod_library_view_height {
        let (button, mut scroll_button, mut button_transform) = button_query.into_inner();
        if button.just_pressed {
            // Save point of reference for updating scrollbar position
            scroll_button.y_selected = cursor.pos.y - button_transform.translation.y;
        } else if button.pressed {
            // Move scrollbar with cursor
            let min_scroll = scroll_button.start_y - scroll_button.scroll_height;
            let max_scroll = scroll_button.start_y;
            button_transform.translation.y = (cursor.pos.y - scroll_button.y_selected).clamp(min_scroll, max_scroll);

            // Move mod library with scrollbar
            let percentage_scroll = 1. - (button_transform.translation.y - min_scroll)/(max_scroll - min_scroll);
            mod_library_transform.translation.y = mod_library.start_y + percentage_scroll*(mod_library.mods_height - mod_library_view_height);
        }
    }
}

pub struct ModsScrollPlugin;
impl Plugin for ModsScrollPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(AppState::Loading), (
            button_scroll_init,
            button_scroll_up_init,
            button_scroll_down_init
        ));
        app.add_systems(Update, (
            buttons_manage_active,
            button_scroll_step
        ));
    }
}

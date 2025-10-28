#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::{
    asset::UnapprovedPathMode,
    audio::{PlaybackMode, Volume},
    color::palettes::basic::BLACK,
    image::ImageSampler,
    prelude::*,
    window::{CursorOptions, PrimaryWindow, WindowMode, WindowResolution, WindowTheme},
    winit::WinitWindows
};
use bevy_image_font::{loader::ImageFontLoaderSettings, ImageFont, ImageFontPlugin};
use bevy_histrion_packer::{HistrionPackerPlugin, HistrionPackerMode};
use bevy_file_asset::FileAssetPlugin;
use winit::window::Icon;
use windows_icons::get_icon_by_path;
use std::collections::HashMap;

mod util;
use crate::util::local_path;

mod settings;
use crate::settings::{Settings, get_settings};

mod camera;
use crate::camera::CameraPlugin;

mod cursor;
use crate::cursor::CursorPlugin;

mod buttons;
use crate::buttons::ButtonsPlugin;

mod log;
use crate::log::LogPlugin;

mod mods;
mod mods_scroll;
use crate::mods::ModsPlugin;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    Loading,
    Running
}

#[derive(Resource)]
pub struct PreloadedAssets {
    pub images: HashMap<String, Handle<Image>>,
    pub audio: HashMap<String, Handle<AudioSource>>,
    pub fonts: HashMap<String, Handle<Font>>,
    pub image_fonts: HashMap<String, Handle<ImageFont>>
}
impl PreloadedAssets {
    fn new() -> PreloadedAssets {
        PreloadedAssets {
            images: HashMap::new(),
            audio: HashMap::new(),
            fonts: HashMap::new(),
            image_fonts: HashMap::new()
        }
    }

    fn get_image(&self, id: &str) -> Handle<Image> {
        (*self.images.get(id).unwrap()).clone()
    }

    fn get_audio(&self, id: &str) -> Handle<AudioSource> {
        (*self.audio.get(id).unwrap()).clone()
    }

    fn get_font(&self, id: &str) -> Handle<Font> {
        (*self.fonts.get(id).unwrap()).clone()
    }

    fn get_image_font(&self, id: &str) -> Handle<ImageFont> {
        (*self.image_fonts.get(id).unwrap()).clone()
    }
}

// All setup needed on game start
fn initialize(
    mut commands: Commands,
    mut next_state: ResMut<NextState<AppState>>,
    asset_server: Res<AssetServer>,
    main_window: Single<Entity, With<PrimaryWindow>>,
    windows: NonSend<WinitWindows>,
    settings: Res<Settings>
) {
    // Get primary window
    let primary =
        windows.get_window(main_window.entity())
        .expect("Unable to get Windows data for application");
    
    // Set window boarder visibility
    primary.set_decorations(!settings.toggle_borderless);

    // Set the icon for the application
    let (icon_rgba, icon_width, icon_height) = {
        // Get the same icon that the main Void War executable uses
        let image = get_icon_by_path(local_path("Void War.exe")).unwrap();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();
    primary.set_window_icon(Some(icon));

    // Center the application
    if let Some(monitor) = primary.current_monitor() {
        let screen_size = monitor.size();
        let window_size = primary.outer_size();
        primary.set_outer_position(winit::dpi::PhysicalPosition {
            x: screen_size.width.saturating_sub(window_size.width) as f64 / 2.
                + monitor.position().x as f64,
            y: screen_size.height.saturating_sub(window_size.height) as f64 / 2.
                + monitor.position().y as f64,
        });
    }

    // Preload assets
    let mut assets = PreloadedAssets::new();
    assets.images.insert(
        "spr_buttonMain".to_string(), 
        asset_server.load("hpak://img/spr_buttonMain.png")
    );
    assets.images.insert(
        "spr_buttonSmall".to_string(), 
        asset_server.load("hpak://img/spr_buttonSmall.png")
    );
    assets.images.insert(
        "spr_confirmBox".to_string(), 
        asset_server.load("hpak://img/spr_confirmBox.png")
    );
    assets.images.insert(
        "spr_buttonScrollDown".to_string(), 
        asset_server.load("hpak://img/spr_buttonScrollDown.png")
    );
    assets.images.insert(
        "spr_buttonScrollUp".to_string(), 
        asset_server.load("hpak://img/spr_buttonScrollUp.png")
    );
    assets.images.insert(
        "spr_buttonScroll".to_string(), 
        asset_server.load("hpak://img/spr_buttonScroll.png")
    );
    assets.images.insert(
        "spr_modButton_up".to_string(), 
        asset_server.load("hpak://img/spr_modButton_up.png")
    );
    assets.images.insert(
        "spr_modButton_down".to_string(), 
        asset_server.load("hpak://img/spr_modButton_down.png")
    );
    assets.images.insert(
        "spr_modButton_toggle".to_string(), 
        asset_server.load("hpak://img/spr_modButton_toggle.png")
    );
    assets.images.insert(
        "spr_modFrame".to_string(), 
        asset_server.load("hpak://img/spr_modFrame.png")
    );
    assets.images.insert(
        "spr_missing".to_string(), 
        asset_server.load("hpak://img/spr_missing.png")
    );
    assets.audio.insert(
        "buttonLarge_doubleClick_SmartSoundFX_v2".to_string(),
        asset_server.load("hpak://snd/buttonLarge_doubleClick_SmartSoundFX_v2.wav")
    );
    assets.audio.insert(
        "buttonSmall_doubleClick1_SmartSoundFXPOND5".to_string(),
        asset_server.load("hpak://snd/buttonSmall_doubleClick1_SmartSoundFXPOND5.wav")
    );
    assets.audio.insert(
        "vs_ui_click1".to_string(),
        asset_server.load("hpak://snd/vs_ui_click1.wav")
    );
    assets.fonts.insert(
        "CloisterBlack".to_string(),
        asset_server.load("hpak://fnt/CloisterBlack.ttf")
    );
    assets.fonts.insert(
        "dubellay".to_string(),
        asset_server.load("hpak://fnt/dubellay.ttf")
    );
    let sampler = if settings.texture_filtering
        { ImageSampler::linear() } else
        { ImageSampler::nearest() };
    assets.image_fonts.insert(
        "pixel_font".to_string(),
        asset_server.load_with_settings(
            "hpak://fnt/pixel_font.image_font.ron",
            move |fnt_settings: &mut ImageFontLoaderSettings| {
                fnt_settings.image_sampler = sampler.clone();
            }
        )
    );
    commands.insert_resource(assets);

    // Show the main layout image and play music
    commands.spawn((
        Sprite::from_image(asset_server.load("hpak://img/spr_main.png")),
        AudioPlayer::new(asset_server.load("hpak://snd/Menu_Music.ogg")),
        PlaybackSettings {
            mode: PlaybackMode::Loop,
            volume: Volume::Linear(settings.curr_volume_bgm),
            ..default()
        },
        Transform::from_xyz(0., 0., -50.),
        children![(
            Text2d::new("Mod Library"),
            TextFont {
                font: asset_server.load("hpak://fnt/CloisterBlack.ttf"),
                font_size: 32.,
                ..default()
            },
            TextColor(BLACK.into()),
            TextLayout::new_with_justify(JustifyText::Center),
            Transform::from_xyz(-10., 328., 0.)
        ), (
            Text2d::new("Log"),
            TextFont {
                font: asset_server.load("hpak://fnt/CloisterBlack.ttf"),
                font_size: 32.,
                ..default()
            },
            TextColor(BLACK.into()),
            TextLayout::new_with_justify(JustifyText::Center),
            Transform::from_xyz(-560., 190., 0.)
        )]
    ));

    // Transition to running state
    // Use OnExit(AppState::Loading) schedule for any startup
    // systems that depend on assets initialized here
    next_state.set(AppState::Running);
}

fn main() {
    let settings = get_settings();

    // Set up the application window
    let app_window = Some(Window {
        title: "Void War".into(),
        resizable: true,
        cursor_options: CursorOptions {
            visible: false,
            ..default()
        },
        mode: if settings.toggle_fullscreen
            { WindowMode::BorderlessFullscreen(MonitorSelection::Current) } else
            { WindowMode::Windowed },
        resolution: WindowResolution::new(
            settings.curr_screen_size.w, 
            settings.curr_screen_size.h
        ).with_scale_factor_override(1.),
        window_theme: Some(WindowTheme::Light), // GameMaker doesn't seem to be capable of using other themes
        desired_maximum_frame_latency: core::num::NonZero::new(1u32),
        ..default()
    });

    // Start the application
    let image_plugin = if settings.texture_filtering
        { ImagePlugin::default_linear() } else
        { ImagePlugin::default_nearest() };
    App::new()
        .insert_resource(ClearColor(BLACK.into()))
        .insert_resource(settings)
        .add_systems(Startup, initialize)
        .add_plugins((
            HistrionPackerPlugin {
                source: "data.hpak".to_string(),
                mode: HistrionPackerMode::Autoload("hpak"),
            },
            FileAssetPlugin
        ))
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: app_window,
                ..default()
            })
            .set(image_plugin)
            // Needed to use FileAssetPlugin's asset source
            // Dicey for applications which support mods but since this is the modding tool itself it should be fine
            .set(AssetPlugin {
                unapproved_path_mode: UnapprovedPathMode::Allow,
                ..default()
            })
        )
        .init_state::<AppState>()
        .add_plugins(ImageFontPlugin)
        .add_plugins((CameraPlugin, CursorPlugin, LogPlugin, ButtonsPlugin, ModsPlugin))
        .run();
}

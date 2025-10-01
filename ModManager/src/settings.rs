use std::fs;
use bevy::ecs::resource::Resource;
use serde::Deserialize;
use serde_this_or_that::as_bool;
use crate::util::appdata_path;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct SettingsWrapper {
    saved_settings: Settings
}

#[derive(Deserialize, Resource)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    pub curr_screen_size: SettingsRes,
    #[serde(deserialize_with = "as_bool")]
    pub toggle_fullscreen: bool,
    #[serde(deserialize_with = "as_bool")]
    pub toggle_borderless: bool,
    #[serde(deserialize_with = "as_bool")]
    pub texture_filtering: bool,
    #[serde(rename = "volumeMax_SFX")]
    pub volume_max_sfx: f32,
    #[serde(rename = "currVolume_BGM")]
    pub curr_volume_bgm: f32
}

#[derive(Deserialize)]
pub struct SettingsRes {
    pub h: f32,
    pub w: f32
}

pub fn get_settings() -> Settings {
    let mut json_data = fs::read_to_string(appdata_path("Void_War\\profile.sav")).expect("Unable to open Void War profile");
    json_data.truncate(json_data.rfind('}').unwrap() + 1); // VW can save the profile with a NUL at the end, we need to account for this
    let settings_wrapper = serde_json::from_str::<SettingsWrapper>(&json_data).expect("Unable to read Void War profile");
    return settings_wrapper.saved_settings;
}

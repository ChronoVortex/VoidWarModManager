use bevy::prelude::*;
use crate::{PreloadedAssets, buttons::{MainButton, button_large_bundle}, mods::{ModEntry, ModToggleButton}, util::local_path};
use std::ffi::{c_char, CString};
use libloading::{Library, Symbol};

#[derive(Component)]
pub struct PatchButton;

pub fn button_patch_init(
    mut commands: Commands,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    assets: Res<PreloadedAssets>
) {
    let dimensions = UVec2::new(134, 38);
    let layout = TextureAtlasLayout::from_grid(dimensions, 2, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    commands.spawn((
        PatchButton,
        button_large_bundle(
            -390., 321., 0.,
            dimensions.as_vec2(),
            assets.get_audio("buttonLarge_doubleClick_SmartSoundFX_v2"),
            assets.get_image("spr_buttonMain"),
            texture_atlas_layout,
            "Patch",
            assets.get_font("CloisterBlack"),
            None
        )
    ));
}

pub fn button_patch_step(
    button: Single<&MainButton, With<PatchButton>>,
	mod_entry_query: Query<(&ModEntry, &Children)>,
    button_toggle_query: Query<&MainButton, With<ModToggleButton>>
) {
    if button.just_pressed {
        unsafe {
            // Prep mod installation library function
            let lib = Library::new("ModManLib.dll").expect("Could not load ModManLib.dll");
            let ex_install_project: Symbol<unsafe extern "C" fn(data_path: *const c_char, proj_path: *const c_char)> =
                lib.get(b"EX_ModmanInstallMod\0").expect("Could not load the function EX_ModmanInstallMod");
            let data_dir = CString::new(local_path("data.win")).unwrap();

            // Patch selected mods in order of index
            for (mod_entry, children) in mod_entry_query.iter().sort_by_key::<&ModEntry, _>(|mod_entry| mod_entry.index) {
                for child in children.iter() {
                    if let Ok(button_toggle) = button_toggle_query.get(child) {
                        if button_toggle.toggle_on {
                            let mod_dir = CString::new(mod_entry.path.as_str()).unwrap();
                            ex_install_project(data_dir.as_ptr(), mod_dir.as_ptr());
                        }
                        break;
                    }
                }
            }
        }
    }
}

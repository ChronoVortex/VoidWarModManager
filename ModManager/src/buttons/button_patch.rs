use std::path::Path;
use bevy::{prelude::*, color::palettes::basic::{LIME, RED}};
use crate::{PreloadedAssets, buttons::{MainButton, button_large_bundle}, log::LogManager, mods::{ModEntry, ModToggleButton}, util::local_path};
use dotbridge::{ClrValue, DotBridgeError, DotBridgeRuntime};

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
    button_toggle_query: Query<&MainButton, With<ModToggleButton>>,
    mut log_man: Single<&mut LogManager>
) {
    if button.just_pressed {
        // Prep mod installation library function
        let runtime = DotBridgeRuntime::new().unwrap(); // TODO: This should only happen once, figure out how to serve it to this function from main
        let install_project = runtime.func_from_assembly(
            "UndertaleProjectMan.dll",
            "UndertaleProjectMan.ProjectInstaller",
            "InstallProject",
        ).expect("Could not find UndertaleProjectMan.dll");

        // Patch selected mods in order of index
        for (mod_entry, children) in mod_entry_query.iter().sort_by_key::<&ModEntry, _>(|mod_entry| mod_entry.index) {
            for child in children.iter() {
                if let Ok(button_toggle) = button_toggle_query.get(child) {
                    if button_toggle.toggle_on {
                        let mut mod_path = mod_entry.path.clone();
                        mod_path.push_str("\\project.json");
                        let mut input = std::collections::HashMap::new();
                        input.insert("dataPath".to_string(), ClrValue::String(local_path("data.win").into()));
                        input.insert("projectPath".to_string(), ClrValue::String(mod_path.as_str().into()));
                        let mod_name = Path::new(&mod_entry.path).file_name().unwrap().to_str().unwrap();
                        match install_project.call_sync(ClrValue::Object(input)) {
                            Ok(_) => {
                                log_man.log(format!("Patched {mod_name}"), None);
                            }
                            Err(DotBridgeError::DotNetException { message, stack_trace }) => {
                                log_man.log(format!("Patching {mod_name} failed"), Some(RED.into()));
                                log_man.log("Export for full error".to_string(), None);
                                log_man.log_export_only(message);
                                if let Some(trace) = stack_trace {
                                    log_man.log_export_only(format!("Stack trace:\n{trace}"));
                                }
                                return;
                            }
                            Err(e) => {
                                log_man.log(format!("Patching {mod_name} failed"), Some(RED.into()));
                                log_man.log("Export for full error".to_string(), None);
                                log_man.log_export_only(e.to_string());
                                return;
                            }
                        }
                    }
                    break;
                }
            }
        }
        log_man.log("Patching successful".to_string(), Some(LIME.into()));
    }
}

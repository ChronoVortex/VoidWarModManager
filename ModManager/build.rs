use std::{env, io, path::PathBuf};
use copy_to_output::copy_to_output;
use winresource::WindowsResource;
use bevy::prelude::*;
use bevy_histrion_packer as bhp;
use bevy_image_font::{ImageFont, loader::ImageFontLoader};

fn main() -> io::Result<()> {
    // Create rust interface for C functions
    let bindings = bindgen::Builder::default()
        .header("src/util.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");
    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // Build C code
    cc::Build::new().file("src/util.c").compile("util-cc");
    
    // Mark for rebuilding if C source files changed
    println!("cargo:rerun-if-changed=src/util.c");
    println!("cargo:rerun-if-changed=src/util.h");

    // Set the icon of the executable
    if env::var_os("CARGO_CFG_WINDOWS").is_some() {
        WindowsResource::new()
            .set_icon("icon.ico")
            .compile()?;
    }

    // Process assets, we can add more assets pre-processing steps here
    App::new()
        .add_plugins(
            DefaultPlugins
                .build()
                .set(bevy::window::WindowPlugin {
                    primary_window: None,
                    exit_condition: bevy::window::ExitCondition::DontExit,
                    ..default()
                })
                .set(bevy::asset::AssetPlugin {
                    mode: AssetMode::Processed,
                    ..default()
                }),
        )
        .add_plugins(bevy::app::ScheduleRunnerPlugin::run_loop(
            std::time::Duration::from_secs_f64(1./30.),
        ))
        // Assets from other crates
        .init_asset::<ImageFont>()
        .init_asset_loader::<ImageFontLoader>()
        // Process assets
        .add_systems(
            Update,
            |asset_processor: Res<bevy::asset::processor::AssetProcessor>,
            mut exit_tx: EventWriter<AppExit>| {
                if bevy::tasks::block_on(asset_processor.get_state())
                    == bevy::asset::processor::ProcessorState::Finished
                {
                    exit_tx.write(AppExit::Success);
                }
            },
        )
        .run();

    // Pack assets
    let crate_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    bhp::writer::pack_assets_folder(
        // Processed assets directory
        crate_dir.join("imported_assets/Default"),
        // Output file
        crate_dir.join("data.hpak"),
        // Do not compress metadata
        bhp::CompressionMethod::None,
        // Use deflate compression method as default for data
        bhp::CompressionMethod::Deflate,
        // Use default extensions compression method
        bhp::writer::default_extensions_compression_method(),
        // Don't ignore missing meta
        false,
        // Don't apply any alignment
        // To align to 4096 bytes you could use:
        // Some(4096),
        None,
    )
    .unwrap();

    // Copy necessary files to build folder
    let target_dir = env::var("PROFILE").unwrap();
    copy_to_output("data.hpak", &target_dir).expect("Could not copy game data to output folder");
    copy_to_output("Void War.exe", &target_dir).expect("Could not copy VW executable to output folder");
    copy_to_output("data.win", &target_dir).expect("Could not copy data.win to output folder");

    Ok(())
}

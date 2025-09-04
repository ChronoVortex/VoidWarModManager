use bevy::{
    prelude::*,
    render::camera::{RenderTarget, ScalingMode},
    window::{PrimaryWindow, WindowRef, WindowResized}
};

#[derive(Component)]
pub struct MainCamera {
    pub ar: f32
}

fn create_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d::default(),
        MainCamera {ar: 16./9.},
        Projection::from(OrthographicProjection {
            scaling_mode: ScalingMode::Fixed {
                width: 1280.,
                height: 720.,
            },
            ..OrthographicProjection::default_2d()
        }),
        Transform::from_xyz(0., 0., 0.),
        Msaa::Off
    ));
}

// Derived from https://github.com/RuelYasa/bevy_auto_scaling
fn update_camera(
    mut e: EventReader<WindowResized>,
    mut cam: Query<(&MainCamera, &mut Camera)>,
    windows: Query<&Window>,
    primary: Query<&PrimaryWindow>
) {
    for event in e.read() {
        for (main_cam, mut camera) in cam.iter_mut() {
            let RenderTarget::Window(rref) = camera.target else {
                continue;
            };
            if let WindowRef::Primary = rref {
                if !primary.contains(event.window) {
                    continue;
                }
            } else if let WindowRef::Entity(e) = rref {
                if e != event.window {
                    continue;
                }
            }
            let window = windows.get(event.window).unwrap();
            let (window_height, window_width) = (
                window.physical_height() as f32,
                window.physical_width() as f32,
            );
            let viewport = camera.viewport.get_or_insert_default();
            viewport.physical_size = if window_width/window_height < main_cam.ar
                { UVec2::new(window_width as u32, (window_width/main_cam.ar) as u32) } else
                { UVec2::new((window_height*main_cam.ar) as u32, window_height as u32) };
            viewport.physical_position = UVec2::new(
                (window_width/2.0) as u32 - viewport.physical_size.x/2,
                (window_height/2.0) as u32 - viewport.physical_size.y/2,
            );
        }
    }
}

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_camera);
        app.add_systems(Update, update_camera);
    }
}

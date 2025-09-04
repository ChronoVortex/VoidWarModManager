use bevy::{prelude::*, sprite::Anchor};

#[derive(Component)]
pub struct Cursor {
    pub pos: Vec2
}

fn cursor_init(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands.spawn((
        Cursor { pos: Vec2::new(0., 0.) },
        Sprite {
            image: asset_server.load("hpak://img/spr_cursor7.png"),
            anchor: Anchor::TopLeft,
            ..default()
        },
        Transform::from_xyz(0., 0., 100.)
    ));
}

fn cursor_step(
    mouse: Res<ButtonInput<MouseButton>>,
    window: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    mut cursor_state: Query<(&mut Transform, &mut Cursor)>
) {
    // Offset sprite by 2 pixels while holding a mouse button
    let offset: f32 =
        if mouse.any_pressed([MouseButton::Left, MouseButton::Right])
        { 2. } else
        { 0. };
    
    if let Ok((camera, position)) = cameras.single() {
        for (mut transform, mut cursor) in &mut cursor_state {
            // Update actual cursor position
            cursor.pos = window
                .single()
                .map(|window| window.cursor_position())
                .unwrap_or_default()
                .map(|cursor| camera.viewport_to_world(position, cursor))
                .map(|ray| ray.unwrap().origin.truncate())
                .unwrap_or(cursor.pos);
            
            // Update cursor sprite position
            transform.translation.x = cursor.pos.x - offset;
            transform.translation.y = cursor.pos.y + offset;
        }
    }
}

pub struct CursorPlugin;
impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, cursor_init);
        app.add_systems(Update, cursor_step);
    }
}

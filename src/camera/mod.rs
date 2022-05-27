use bevy::{prelude::*, render::camera::Camera2d};

use crate::{player::{Player, SPAWN_POINT}, cursor::Cursor};

// =========================================================
// ==================== CAMERA PLUGIN ======================
// =========================================================

pub struct MagusCameraPlugin;

impl Plugin for MagusCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_camera)
            .add_system(camera_movement);
    }
}

// =========================================================
// ====================== CONSTANTS ========================
// =========================================================

pub const ZOOM: f32 = 3.;

// =========================================================
// ======================= SYSTEMS =========================
// =========================================================

// ----- spawn camera --------
pub fn spawn_camera(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection.scale /= ZOOM;
    camera.transform = Transform::from_xyz(SPAWN_POINT.x, SPAWN_POINT.y, camera.transform.translation.z);

    commands.spawn_bundle(camera);
}

// ----- follow player -------
fn camera_movement(
    player_query: Query<&Transform, (With<Player>, Without<Camera2d>)>,
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
    cursor_query: Query<&GlobalTransform, With<Cursor>>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        let mut camera_transform = camera_query.single_mut();
        let cursor_transform = cursor_query.single();
    
        let focus_point = Vec2::lerp(player_transform.translation.truncate(), cursor_transform.translation.truncate(), 0.1);
    
        camera_transform.translation.x = focus_point.x;
        camera_transform.translation.y = focus_point.y;
    }
}

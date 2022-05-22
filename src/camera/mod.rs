use bevy::{prelude::*, render::camera::Camera2d};

use crate::player::Player;

pub struct MagusCameraPlugin;

impl Plugin for MagusCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_camera)
            .add_system(follow_player);
    }
}

// =========================================================
// ======================= SYSTEMS =========================
// =========================================================

// ----- spawn camera --------
pub fn spawn_camera(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection.scale /= 3.;
    camera.transform = Transform::from_xyz(400., 200., camera.transform.translation.z);

    commands.spawn_bundle(camera);
}

// ----- follow player -------
fn follow_player(
    player_query: Query<&Transform, (With<Player>, Without<Camera2d>)>,
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
    time: Res<Time>,
) {
    let player_transform = player_query.single();
    let mut camera_transform = camera_query.single_mut();

    let pos_dif = Vec2::new(
        player_transform.translation.x,
        player_transform.translation.y,
    ) - Vec2::new(
        camera_transform.translation.x,
        camera_transform.translation.y,
    );
    camera_transform.translation.x += pos_dif.x * time.delta_seconds() * 2.;
    camera_transform.translation.y += pos_dif.y * time.delta_seconds() * 2.;
}

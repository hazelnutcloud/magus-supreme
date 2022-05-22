use bevy::{prelude::*, render::camera::Camera2d};

use crate::player::Player;

pub struct MagusCameraPlugin;

impl Plugin for MagusCameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(spawn_camera)
            .add_system(follow_player);
    }
}

pub fn spawn_camera(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection.scale /= 3.;

    commands.spawn_bundle(camera);
}

fn follow_player(
    player_query: Query<&Transform, (With<Player>, Without<Camera2d>)>,
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
    time: Res<Time>
) {
    let player_transform = player_query.single();
    let mut camera_transform = camera_query.single_mut();

    let pos_dif = player_transform.translation - camera_transform.translation;
    camera_transform.translation += pos_dif * time.delta_seconds() * 2.;
}
use bevy::prelude::*;

pub fn spawn_camera(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection.scale /= 3.;

    commands.spawn_bundle(camera);
}
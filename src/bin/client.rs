use bevy::prelude::*;
use magus_supreme::player::PlayerPlugin;
use magus_supreme::camera::spawn_camera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(benimator::AnimationPlugin::default())
        .add_plugin(PlayerPlugin)
        .add_system(spawn_camera)
        .run();
}
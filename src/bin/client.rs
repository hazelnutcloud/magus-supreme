use bevy::input::InputPlugin;
use bevy::prelude::*;
use leafwing_input_manager::plugin::InputManagerPlugin;
use magus_supreme::player::{PlayerPlugin, PlayerAction};
use magus_supreme::camera::spawn_camera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(benimator::AnimationPlugin::default())
        .add_plugin(InputPlugin)
        .add_plugin(InputManagerPlugin::<PlayerAction>::default())
        .add_plugin(PlayerPlugin)
        .add_system(spawn_camera)
        .run();
}
use bevy::input::InputPlugin;
use bevy::prelude::*;
use heron::prelude::*;
use leafwing_input_manager::plugin::InputManagerPlugin;
use magus_supreme::player::{PlayerPlugin, PlayerAction};
use magus_supreme::camera::MagusCameraPlugin;
use magus_supreme::tilemap::Tilemap;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(benimator::AnimationPlugin::default())
        .add_plugin(Tilemap)
        .add_plugin(InputPlugin)
        .add_plugin(InputManagerPlugin::<PlayerAction>::default())
        .add_plugin(PhysicsPlugin::default())
        .insert_resource(PhysicsTime::new(75.))
        .add_plugin(PlayerPlugin)
        .add_plugin(MagusCameraPlugin)
        .run();
}
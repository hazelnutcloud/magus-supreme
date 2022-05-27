use bevy::input::InputPlugin;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_renet::RenetClientPlugin;
use leafwing_input_manager::plugin::InputManagerPlugin;
use magus_supreme::camera::MagusCameraPlugin;
use magus_supreme::cursor::MagusCursorPlugin;
use magus_supreme::player::{PlayerAction, MagusPlayerPlugin};
use magus_supreme::tilemap::MagusTilemapPlugin;
use magus_supreme::client::MagusClientPlugin;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Magus Supreme".into(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(benimator::AnimationPlugin::default())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.))
        .add_plugin(RapierDebugRenderPlugin::default())
        .insert_resource(RapierConfiguration {
            gravity: Vec2::new(0., 0.),
            ..Default::default()
        })
        .add_plugin(InputPlugin)
        .add_plugin(InputManagerPlugin::<PlayerAction>::default())
        .add_plugin(RenetClientPlugin)
        .add_plugin(MagusTilemapPlugin::client())
        .add_plugin(MagusPlayerPlugin::client())
        .add_plugin(MagusClientPlugin)
        .add_plugin(MagusCameraPlugin)
        .add_plugin(MagusCursorPlugin)
        .run();
}

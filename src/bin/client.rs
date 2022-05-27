use bevy::input::InputPlugin;
use bevy::input::system::exit_on_esc_system;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_renet::RenetClientPlugin;
use leafwing_input_manager::plugin::InputManagerPlugin;
use magus_supreme::camera::MagusCameraPlugin;
use magus_supreme::cursor::CursorPlugin;
use magus_supreme::player::{PlayerAction, PlayerPlugin};
use magus_supreme::tilemap::MagusTilemapPlugin;
use magus_supreme::client::MagusClientPlugin;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            cursor_locked: true,
            cursor_visible: false,
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
        .add_plugin(MagusTilemapPlugin)
        .add_plugin(PlayerPlugin::client())
        .add_plugin(MagusClientPlugin)
        .add_plugin(MagusCameraPlugin)
        .add_plugin(CursorPlugin)
        .add_system(exit_on_esc_system)
        .run();
}

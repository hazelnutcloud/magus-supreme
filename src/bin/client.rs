use bevy::input::InputPlugin;
use bevy::input::system::exit_on_esc_system;
use bevy::prelude::*;
use heron::prelude::*;
use leafwing_input_manager::plugin::InputManagerPlugin;
use magus_supreme::camera::MagusCameraPlugin;
use magus_supreme::cursor::CursorPlugin;
use magus_supreme::player::{PlayerAction, PlayerPlugin};
use magus_supreme::tilemap::Tilemap;

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
        .add_plugin(Tilemap)
        .add_plugin(InputPlugin)
        .add_plugin(InputManagerPlugin::<PlayerAction>::default())
        .add_plugin(PhysicsPlugin::default())
        .add_plugin(PlayerPlugin)
        .add_plugin(MagusCameraPlugin)
        .add_plugin(CursorPlugin)
        .add_system(exit_on_esc_system)
        .run();
}

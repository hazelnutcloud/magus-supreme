use bevy::prelude::*;
use bevy::asset::AssetPlugin;
use bevy_rapier2d::prelude::*;
use bevy_renet::RenetServerPlugin;
use magus_supreme::{player::PlayerPlugin, server::MagusServerPlugin, tilemap::MagusTilemapPlugin};

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugin(AssetPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.))
        .add_plugin(RenetServerPlugin)
        .add_plugin(PlayerPlugin::server())
        .add_plugin(MagusTilemapPlugin::server())
        .add_plugin(MagusServerPlugin)
        .run();
}
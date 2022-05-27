use bevy::prelude::*;
use bevy::asset::AssetPlugin;
use bevy_rapier2d::prelude::*;
use bevy_renet::RenetServerPlugin;
use magus_supreme::{player::MagusPlayerPlugin, server::MagusServerPlugin, tilemap::MagusTilemapPlugin};

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugin(AssetPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.))
        .insert_resource(RapierConfiguration {
            gravity: Vec2::new(0., 0.),
            ..Default::default()
        })
        .add_plugin(RenetServerPlugin)
        .add_plugin(MagusPlayerPlugin::server())
        .add_plugin(MagusTilemapPlugin::server())
        .add_plugin(MagusServerPlugin)
        .run();
}
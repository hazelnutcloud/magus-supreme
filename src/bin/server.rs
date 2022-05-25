use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use magus_supreme::player::PlayerPlugin;
//TODO implement server code
fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.))
        .add_plugin(PlayerPlugin::server());
}
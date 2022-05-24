use bevy::prelude::*;
use heron::prelude::*;
use magus_supreme::player::PlayerPlugin;
//TODO implement server code
fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugin(PhysicsPlugin::default())
        .add_plugin(PlayerPlugin::server());
}
use bevy::{prelude::*, utils::HashMap};
use bevy_renet::renet::RenetServer;

use crate::player::Player;

pub fn sync(
    query: Query<(&Transform, &Player)>,
    mut server: ResMut<RenetServer>
) {
    let mut players: HashMap<u64, [f32; 3]> = HashMap::new();

    for (transform, player) in query.iter() {
        players.insert(player.id, transform.translation.into());
    }

    let sync_message = bincode::serialize(&players).unwrap();
    server.broadcast_message(1, sync_message);
}
use crate::server::Room;

use bevy::{prelude::*, utils::HashMap};
use bevy_renet::renet::RenetClient;

pub fn sync(mut client: ResMut<RenetClient>, room: Res<Room>, mut transform_query: Query<&mut Transform>) {
    while let Some(message) = client.receive_message(1) {
        let players: HashMap<u64, [f32; 3]> = bincode::deserialize(&message).unwrap();

        for (id, translation) in players.iter() {
            if let Some(player) = room.players.get(id) {
                if let Ok(mut transform) = transform_query.get_mut(*player) {
                    transform.translation.x = translation[0];
                    transform.translation.y = translation[1];
                }
            }
        }
    }
}

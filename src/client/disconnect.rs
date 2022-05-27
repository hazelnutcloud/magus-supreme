use crate::server::{Room, ServerMessages};

use bevy::prelude::*;
use bevy_renet::renet::RenetClient;

pub fn handle_disconnect(mut commands: Commands, mut client: ResMut<RenetClient>, mut room: ResMut<Room>) {
    while let Some(message) = client.receive_message(0) {
        let server_message = bincode::deserialize(&message).unwrap();

        if let ServerMessages::PlayerDisconnected { id } = server_message {
            println!("Player {} disconnected", id);
            if let Some(player) = room.players.remove(&id) {
                commands.entity(player).despawn();
            }
        }
    }
}

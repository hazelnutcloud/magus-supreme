use super::{ServerMessages, Room};
use bevy::prelude::*;
use bevy_renet::renet::{RenetServer, ServerEvent};

pub fn handle_disconnected(
    mut events: EventReader<ServerEvent>,
    mut commands: Commands,
    mut room: ResMut<Room>,
    mut server: ResMut<RenetServer>
) {
    for event in events.iter() {
        if let ServerEvent::ClientDisconnected(id) = event {
            println!("player {} disconnected", id);

            if let Some(player) = room.players.remove(id) {
                commands.entity(player).despawn();
            }

            let message = bincode::serialize(&ServerMessages::PlayerDisconnected {id: *id}).unwrap();
            server.broadcast_message(0, message);
        }
    }
}
use crate::player::PlayerInput;

use super::Room;
use bevy::prelude::*;
use bevy_renet::renet::RenetServer;

pub fn handle_input(
    room: Res<Room>,
    mut server: ResMut<RenetServer>,
    mut input_query: Query<&mut PlayerInput>
) {
    for client_id in server.clients_id() {
        while let Some(message) = server.receive_message(client_id, 0) {
            let player_input: PlayerInput = bincode::deserialize(&message).unwrap();
            
            if let Some(player) = room.players.get(&client_id) {
                if let Ok(mut input) = input_query.get_mut(*player) {
                    *input = player_input;
                }
            }
        }
    }
}
use super::{Room, ServerMessages};
use crate::player::{PlayerBundle, PlayerInput, MovementSpeed, Player, SPAWN_POINT};
use bevy::prelude::*;
use bevy_renet::renet::{RenetServer, ServerEvent};

pub fn handle_connected(
    mut events: EventReader<ServerEvent>,
    mut commands: Commands,
    mut room: ResMut<Room>,
    mut server: ResMut<RenetServer>
) {
    for event in events.iter() {
        if let ServerEvent::ClientConnected(id, _) = event {
            println!("player {} connected", id);

            let player = commands
                .spawn_bundle(PlayerBundle {
                    player: Player { id: *id },
                    movement_speed: MovementSpeed(75.),
                    input: PlayerInput::default()
                })
                .insert_bundle(PlayerBundle::default_physics())
                .insert_bundle(TransformBundle::from_transform(Transform::from_translation(SPAWN_POINT)))
                .id();

            for &player_id in room.players.keys() {
                let message = bincode::serialize(&ServerMessages::PlayerConnected { id: player_id }).unwrap();
                server.send_message(*id, 0, message);
            }

            room.players.insert(*id, player);

            let message = bincode::serialize(&ServerMessages::PlayerConnected { id: *id }).unwrap();
            server.broadcast_message(0, message);
        }
    }
}
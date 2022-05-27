use crate::player::{PlayerAtlas, SPAWN_POINT};
use benimator::Play;
use bevy::prelude::*;
use bevy_renet::renet::RenetClient;
use leafwing_input_manager::prelude::ActionState;
use leafwing_input_manager::prelude::InputManagerBundle;

use crate::player::{MovementSpeed, Player, PlayerAnimations, PlayerBundle, PlayerInput};
use crate::server::{Room, ServerMessages};

pub fn handle_connect(
    mut commands: Commands,
    mut client: ResMut<RenetClient>,
    mut room: ResMut<Room>,
    animations: Res<PlayerAnimations>,
    spritesheet: Res<PlayerAtlas>,
) {
    while let Some(message) = client.receive_message(0) {
        let server_message = bincode::deserialize(&message).unwrap();
        if let ServerMessages::PlayerConnected { id } = server_message {
            println!("Player {} connected", id);

            let player = commands
                .spawn_bundle(PlayerBundle {
                    player: Player { id },
                    movement_speed: MovementSpeed(75.),
                    input: PlayerInput::default(),
                    physics: PlayerBundle::default_physics(),
                })
                .insert_bundle(SpriteSheetBundle {
                    texture_atlas: spritesheet.0.clone(),
                    transform: Transform::from_translation(SPAWN_POINT),
                    ..Default::default()
                })
                .insert(animations.idle.clone())
                .insert(Play)
                .id();

            if id == client.client_id() {
                commands.entity(player).insert_bundle(InputManagerBundle {
                    action_state: ActionState::default(),
                    input_map: PlayerBundle::default_input_map(),
                });
            }

            room.players.insert(id, player);
        }
    }
}

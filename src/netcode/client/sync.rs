use crate::{server::Room, player::{AnimationState, PlayerState}};

use bevy::{prelude::*, utils::HashMap};
use bevy_renet::renet::RenetClient;

pub fn sync(mut client: ResMut<RenetClient>, room: Res<Room>, mut transform_query: Query<(&mut Transform, &mut AnimationState)>) {
    while let Some(message) = client.receive_message(1) {
        let players: HashMap<u64, [f32; 2]> = bincode::deserialize(&message).unwrap();

        for (id, translation) in players.iter() {
            if let Some(player) = room.players.get(id) {
                if let Ok((mut transform, mut animation_state)) = transform_query.get_mut(*player) {
                    let velocity = Vec2::from_slice(translation) - transform.translation.truncate();

                    transform.translation.x = translation[0];
                    transform.translation.y = translation[1];
                    
                    if *id == client.client_id() { continue } // TODO: add server reconciliation

                    let is_moving_horizontally = velocity.x != 0.;

                    if is_moving_horizontally {
                        let is_facing_left = velocity.x < 0.;
                        if is_facing_left != animation_state.last_facing_is_left {
                            animation_state.last_facing_is_left = is_facing_left;
                        }
                    }

                    if velocity != Vec2::ZERO {
                        animation_state.state = PlayerState::Moving;
                    } else {
                        animation_state.state = PlayerState::Idle;
                    }
                }
            }
        }
    }
}

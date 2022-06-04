use crate::{
    netcode::snapshot_interpolation::{
        vault::{Snapshot, StateValue},
        SnapshotInterpolation,
    },
    player::{AnimationState, PlayerState},
    server::Room,
};

use bevy::prelude::*;
use bevy_renet::renet::RenetClient;

pub fn sync(mut client: ResMut<RenetClient>, mut si: ResMut<SnapshotInterpolation>) {
    while let Some(message) = client.receive_message(1) {
        let players: Snapshot = bincode::deserialize(&message).unwrap();

        si.add_snapshot(players);
    }
}

pub fn server_reconciliation(
    client: Res<RenetClient>,
    mut si: ResMut<SnapshotInterpolation>,
    room: Res<Room>,
    mut transform_query: Query<(&mut Transform, &mut AnimationState)>,
) {
    if let Some(snapshot) = si.calc_interpolation("players", vec!["x".into(), "y".into()]) {
        for snapolated_entity in snapshot.entities.iter() {

            if snapolated_entity.id == client.client_id() { continue }

            if let Some(entity) = room.players.get(&snapolated_entity.id) {
                if let Ok((mut transform, mut animation_state)) = transform_query.get_mut(*entity) {
                    if let (Some(StateValue::Number(x)), Some(StateValue::Number(y))) = (
                        snapolated_entity.state.get("x"),
                        snapolated_entity.state.get("y"),
                    ) {
                        let velocity = Vec2::new(*x, *y) - transform.translation.truncate();

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

                        transform.translation.x = *x;
                        transform.translation.y = *y;
                    }
                }
            }
        }
    }
}

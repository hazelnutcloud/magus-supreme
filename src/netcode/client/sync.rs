use crate::{
    netcode::snapshot_interpolation::{
        vault::{Snapshot, StateValue, Vault},
        SnapshotInterpolation,
    },
    player::{PlayerAction, PlayerInput},
    server::Room,
};

use bevy::prelude::*;
use bevy_rapier2d::prelude::{ExternalImpulse, Velocity};
use bevy_renet::renet::RenetClient;
use leafwing_input_manager::prelude::ActionState;

pub fn sync(mut client: ResMut<RenetClient>, mut si: ResMut<SnapshotInterpolation>) {
    while let Some(message) = client.receive_message(1) {
        let players: Snapshot = bincode::deserialize(&message).unwrap();

        si.add_snapshot(players);
    }
}

pub fn snapshot_interpolation(
    client: Res<RenetClient>,
    mut si: ResMut<SnapshotInterpolation>,
    room: Res<Room>,
    mut transform_query: Query<(&mut Transform, &mut Velocity)>,
) {
    if let Some(snapshot) = si.calc_interpolation("players", vec!["x".into(), "y".into()]) {
        for snapolated_entity in snapshot.entities.iter() {
            if snapolated_entity.id == client.client_id() {
                continue;
            }

            if let Some(entity) = room.players.get(&snapolated_entity.id) {
                if let Ok((mut transform, mut velocity)) = transform_query.get_mut(*entity) {
                    if let (Some(StateValue::Number(x)), Some(StateValue::Number(y))) = (
                        snapolated_entity.state.get("x"),
                        snapolated_entity.state.get("y"),
                    ) {
                        let displacement = Vec2::new(*x, *y) - transform.translation.truncate();
                        velocity.linvel.x = displacement.x * 100.;
                        velocity.linvel.y = displacement.y * 100.;


                        // let is_moving_horizontally = velocity.x != 0.;

                        // if is_moving_horizontally {
                        //     let is_facing_left = velocity.x < 0.;
                        //     if is_facing_left != animation_state.last_facing_is_left {
                        //         animation_state.last_facing_is_left = is_facing_left;
                        //     }
                        // }

                        // if velocity != Vec2::ZERO {
                        //     animation_state.state = PlayerState::Moving;
                        // } else {
                        //     animation_state.state = PlayerState::Idle;
                        // }

                        transform.translation.x = *x;
                        transform.translation.y = *y;
                    }
                }
            }
        }
    }
}

pub fn server_reconciliation(
    mut player_query: Query<
        (&Vault, &PlayerInput, &mut ExternalImpulse),
        With<ActionState<PlayerAction>>,
    >,
    mut si: ResMut<SnapshotInterpolation>,
    client: Res<RenetClient>,
) {
    if let Ok((vault, player_input, mut external_impulse)) = player_query.get_single_mut() {
        let mut server_position: Option<Vec2> = None;
        let mut client_position: Option<Vec2> = None;

        if let Some(server_snapshot) = si.vault.get_latest() {
            if let Some(server_players) = server_snapshot.entities.get("players") {
                if let Some(server_player) = server_players
                    .iter()
                    .find(|player| player.id == client.client_id())
                {
                    if let (Some(StateValue::Number(x)), Some(StateValue::Number(y))) =
                        (server_player.state.get("x"), server_player.state.get("y"))
                    {
                        server_position = Some(Vec2::new(*x, *y));
                    }
                }
            }

            if let Some(client_snapshot) = vault.get_closest(server_snapshot.time) {
                if let Some(client_players) = client_snapshot.entities.get("players") {
                    if let Some(client_player) = client_players.first() {
                        if let (Some(StateValue::Number(x)), Some(StateValue::Number(y))) =
                            (client_player.state.get("x"), client_player.state.get("y"))
                        {
                            client_position = Some(Vec2::new(*x, *y));
                        }
                    }
                }
            }
        }

        if let (Some(server_position), Some(client_position)) = (server_position, client_position) {
            let offset = server_position - client_position;

            let correction = match player_input.is_moving() {
                true => 30.,
                false => 90.,
            };

            if offset.length() != 0. {
                external_impulse.impulse = offset / correction;
            } else {
                external_impulse.impulse = Vec2::ZERO;
            }
        }
    }
}

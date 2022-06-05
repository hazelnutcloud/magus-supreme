use bevy::{prelude::*, utils::HashMap};
use bevy_renet::renet::RenetClient;
use leafwing_input_manager::prelude::ActionState;

use crate::{player::{PlayerAction, PlayerInput}, netcode::snapshot_interpolation::{vault::{Vault, SnapolationEntity, StateValue}, SnapshotInterpolation}};

pub fn send_input(
    query: Query<&PlayerInput, With<ActionState<PlayerAction>>>,
    mut client: ResMut<RenetClient>,
) {
    if let Ok(player_input) = query.get_single() {
        let input_message = bincode::serialize(player_input).unwrap();
    
        client.send_message(0, input_message);
    }
}

pub fn client_prediction(player_query: Query<&Transform, With<ActionState<PlayerAction>>>, mut vault_query: Query<&mut Vault>, client: Res<RenetClient>) {
    if let Ok(mut vault) = vault_query.get_single_mut() {
        let mut entities = HashMap::new();
    
        let mut players = vec![SnapolationEntity {
            id: client.client_id(),
            state: HashMap::new()
        }];
    
        let transform = player_query.single();
        players[0].state.insert("x".into(), StateValue::Number(transform.translation.x));
        players[0].state.insert("y".into(), StateValue::Number(transform.translation.y));
    
        entities.insert("players".into(), players);
        let snapshot = SnapshotInterpolation::create_snapshot(entities);
    
        vault.add(snapshot);
    }
}
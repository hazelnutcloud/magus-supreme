use bevy::{prelude::*, utils::HashMap};
use bevy_renet::renet::RenetServer;

use crate::{player::Player, netcode::snapshot_interpolation::{vault::{SnapolationEntity, StateValue, SnapolationEntities}, SnapshotInterpolation}};

pub fn sync (
    query: Query<(&Transform, &Player)>,
    mut server: ResMut<RenetServer>,
    mut si: ResMut<SnapshotInterpolation>
) {
    let mut players: Vec<SnapolationEntity> = Vec::new();

    for (transform, player) in query.iter() {
        let mut snapolation_entity = SnapolationEntity { id: player.id, state: HashMap::new() };
        snapolation_entity.state.insert("x".into(), StateValue::Number(transform.translation.x));
        snapolation_entity.state.insert("y".into(), StateValue::Number(transform.translation.y));
        players.push(snapolation_entity);
    }

    if players.is_empty() {
        return
    }
    
    let mut snapolation_entities: SnapolationEntities = HashMap::new();
    snapolation_entities.insert("players".into(), players);

    let snapshot = SnapshotInterpolation::create_snapshot(snapolation_entities);

    let sync_message = bincode::serialize(&snapshot).unwrap();
    println!("message size: {} bytes", sync_message.len());

    server.broadcast_message(1, sync_message);

    si.add_snapshot(snapshot);
}
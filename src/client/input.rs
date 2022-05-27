use bevy::prelude::*;
use bevy_renet::renet::RenetClient;
use leafwing_input_manager::prelude::ActionState;

use crate::player::{PlayerAction, PlayerInput};

pub fn send_input(
    query: Query<&PlayerInput, With<ActionState<PlayerAction>>>,
    mut client: ResMut<RenetClient>,
) {
    if let Ok(player_input) = query.get_single() {
        let input_message = bincode::serialize(player_input).unwrap();
    
        client.send_message(0, input_message);
    }
}

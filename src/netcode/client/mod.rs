mod connect;
mod disconnect;
mod input;
mod sync;

use connect::handle_connect;
use disconnect::handle_disconnect;
use input::send_input;
use sync::sync;

use crate::server::{Room, PRIVATE_KEY, PROTOCOL_ID};
use std::net::UdpSocket;
use std::time::SystemTime;

use bevy::prelude::*;
use bevy_renet::{
    renet::{ConnectToken, RenetClient, RenetConnectionConfig},
    run_if_client_conected,
};

pub struct MagusClientPlugin;

impl Plugin for MagusClientPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(new_renet_client())
            .insert_resource(Room::default())
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(run_if_client_conected)
                    .with_system(send_input)
                    .with_system(handle_connect)
                    .with_system(handle_disconnect)
                    .with_system(sync),
            );
    }
}

fn new_renet_client() -> RenetClient {
    let server_addr = "127.0.0.1:5000".parse().unwrap();
    let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
    let connection_config = RenetConnectionConfig::default();
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    let client_id = current_time.as_millis() as u64;

    let token = ConnectToken::generate(
        current_time,
        PROTOCOL_ID,
        300,
        client_id,
        15,
        vec![server_addr],
        None,
        PRIVATE_KEY,
    )
    .unwrap();

    RenetClient::new(current_time, socket, client_id, token, connection_config).unwrap()
}

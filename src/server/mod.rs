mod sync;
mod connect;
mod disconnect;
mod input;

use sync::sync;
use connect::handle_connected;
use disconnect::handle_disconnected;
use input::handle_input;

use std::{time::SystemTime, net::UdpSocket};

use bevy::{prelude::*, utils::HashMap, core::FixedTimestep};
use bevy_renet::renet::{NETCODE_KEY_BYTES, RenetServer, RenetConnectionConfig, ServerConfig};
use serde::{Serialize, Deserialize};

pub const PROTOCOL_ID: u64 = 1;
pub const PRIVATE_KEY: &[u8; NETCODE_KEY_BYTES] = b"12346789012345678901234567890123";

pub struct MagusServerPlugin;

impl Plugin for MagusServerPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(new_renet_server())
            .insert_resource(Room::default())
            .add_system(handle_connected)
            .add_system(handle_disconnected)
            .add_system(handle_input)
            .add_system(sync.with_run_criteria(FixedTimestep::step(1. / 30.)));
    }
}

#[derive(Default, Debug)]
pub struct Room {
    pub players: HashMap<u64, Entity>
}

#[derive(Debug, Serialize, Deserialize, Component)]
pub enum ServerMessages {
    PlayerConnected { id: u64 },
    PlayerDisconnected { id: u64 }
}

pub fn new_renet_server() -> RenetServer {
    let server_addr = "127.0.0.1:5000".parse().unwrap();
    let socket = UdpSocket::bind(server_addr).unwrap();
    let connection_config = RenetConnectionConfig::default();
    let server_config = ServerConfig::new(64, PROTOCOL_ID, server_addr, *PRIVATE_KEY);
    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();

    RenetServer::new(current_time, server_config, connection_config, socket).unwrap()
}
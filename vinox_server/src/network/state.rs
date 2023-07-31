use renet::{
    transport::{NetcodeServerTransport, ServerAuthentication, ServerConfig},
    ConnectionConfig, RenetServer, ServerEvent,
};
use std::{
    net::UdpSocket,
    time::{Duration, SystemTime},
};
use vinox_common::prelude::PROTOCOL_ID;

pub struct NetworkState {
    pub server: RenetServer,
    pub transport: NetcodeServerTransport,
}

impl NetworkState {
    pub fn new() -> Self {
        let socket = UdpSocket::bind("0.0.0.0:56552").unwrap();
        let public_addr = "127.0.0.1:56552".parse().unwrap();
        println!("Hosting server on: {public_addr}");
        let server_config = ServerConfig {
            max_clients: 64,
            protocol_id: PROTOCOL_ID,
            public_addr,
            authentication: ServerAuthentication::Unsecure,
        };

        let current_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        let transport = NetcodeServerTransport::new(current_time, server_config, socket).unwrap();

        let server: RenetServer = RenetServer::new(ConnectionConfig::default());
        Self { server, transport }
    }

    pub fn update(&mut self, duration: Duration) {
        // Uncapped or vsync frame rate
        self.server.update(duration);
        self.transport.update(duration, &mut self.server).unwrap();

        while let Some(event) = self.server.get_event() {
            match event {
                ServerEvent::ClientConnected { client_id } => {
                    // let user_data = self.transport.user_data(client_id).unwrap();
                    println!("Client {} Connected!", client_id);
                }
                ServerEvent::ClientDisconnected {
                    client_id,
                    reason: _,
                } => {
                    println!("Client {} Disconnected!", client_id);
                }
            }
        }
    }

    pub fn exit(&mut self) {
        self.server.disconnect_all();
    }
}

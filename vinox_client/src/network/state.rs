use renet::{
    transport::{ClientAuthentication, NetcodeClientTransport},
    ConnectionConfig, RenetClient,
};
use std::{
    net::UdpSocket,
    time::{Duration, SystemTime},
};
use vinox_common::prelude::PROTOCOL_ID;

pub struct NetworkState {
    pub client: RenetClient,
    pub transport: NetcodeClientTransport,
}

impl NetworkState {
    pub fn new(server_addr: String) -> Option<Self> {
        if let Ok(server_addr) = server_addr.parse() {
            let connection_config = ConnectionConfig::default();
            let client = RenetClient::new(connection_config);

            let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
            let current_time = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap();
            let client_id = current_time.as_millis() as u64;
            let authentication = ClientAuthentication::Unsecure {
                server_addr,
                client_id,
                user_data: None,
                protocol_id: PROTOCOL_ID,
            };

            let transport =
                NetcodeClientTransport::new(current_time, authentication, socket).unwrap();

            return Some(Self { client, transport });
        }
        None
    }

    pub fn update(&mut self, duration: Duration) {
        // Uncapped or vsync frame rate
        self.client.update(duration);
        self.transport.update(duration, &mut self.client).unwrap();
    }

    pub fn exit(&mut self) {
        self.client.disconnect();
    }
}

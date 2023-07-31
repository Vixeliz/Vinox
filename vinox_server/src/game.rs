use std::time::Duration;

use crate::network::state::NetworkState;

pub struct VinoxServer {
    pub network: NetworkState,
}

impl VinoxServer {
    pub fn new() -> Self {
        Self {
            network: NetworkState::new(),
        }
    }

    pub fn update(&mut self, duration: Duration) {
        // Uncapped or vsync frame rate
        self.network.update(duration);
    }

    pub fn tick(&mut self) {
        // Fixed tick update function should be 30ticks per second
    }

    pub fn exit(&mut self) {
        self.network.exit();
    }
}

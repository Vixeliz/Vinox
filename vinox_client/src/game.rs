use crate::{input::InputState, network::state::NetworkState};
use std::time::Duration;

pub struct VinoxClient {
    network: NetworkState,
}

impl VinoxClient {
    pub fn new() -> Self {
        Self {
            network: NetworkState::new("127.0.0.1:56552".to_string()).unwrap(),
        }
    }

    pub fn update(&mut self, duration: Duration) {
        // Uncapped or vsync frame rate
        self.network.update(duration);
    }

    // Maybe return a vec of items that implement a trait? Ie something similiar to ggez drawable
    pub fn render(&mut self) {
        // Do non renderer specific rendering things here ie build chunk meshes, entity meshes/models, etc
    }

    pub fn ui(&mut self) {
        // Ui is a seperate function since render will only be used for things in a 3d environment
    }

    pub fn tick(&mut self) {
        // Fixed tick update function should be 30ticks per second
    }

    pub fn input(&mut self, input: &InputState) {
        // Provide input state that is needed to vinox
    }

    pub fn exit(&mut self) {
        self.network.exit();
    }
}

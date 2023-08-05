use crate::{
    input::InputState,
    network::state::NetworkState,
    render::state::{ConvertModel, RenderState},
};
use std::time::Duration;

pub struct VinoxClient<M: ConvertModel> {
    network: NetworkState,
    render: RenderState<M>,
}

impl<M: ConvertModel> VinoxClient<M> {
    pub fn new() -> Self {
        Self {
            network: NetworkState::new("127.0.0.1:56552".to_string()).unwrap(),
            render: RenderState::default(),
        }
    }

    pub fn update(&mut self, duration: Duration) {
        // Uncapped or vsync frame rate
        self.network.update(duration).ok();
        self.render.camera.position = [-5.0, 0.0, 2.5].into();
    }

    // Maybe return a vec of items that implement a trait? Ie something similiar to ggez drawable
    pub fn render(&mut self) -> &mut RenderState<M> {
        // Do non renderer specific rendering things here ie build chunk meshes, entity meshes/models, etc
        &mut self.render
    }

    pub fn ui(&mut self, gui: &mut egui::Context) {
        // Ui is a seperate function since render will only be used for things in a 3d environment
        egui::SidePanel::new(egui::panel::Side::Left, "Title").show(gui, |ui| {
            ui.label("label");
            if ui.button("button").clicked() {
                println!("button clicked");
            }
        });
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

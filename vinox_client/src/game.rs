use glam::Quat;

use crate::{
    input::InputState,
    network::state::NetworkState,
    render::{
        model::Model,
        state::{ConvertModel, RenderState},
    },
};
use std::{marker::PhantomData, path::Path, time::Duration};

pub struct VinoxClient<S, M: ConvertModel<S>> {
    network: NetworkState,
    render: RenderState<S, M>,
    _phantom: PhantomData<S>,
}

impl<S, M: ConvertModel<S>> VinoxClient<S, M> {
    pub fn new(state: &mut S) -> Self {
        let mut render = RenderState::<S, M>::default();
        render.asset_registry.models.push(M::to_mesh(
            Model::from_gltf(Path::new("vinox_client/assets/player.glb")).unwrap(),
            state,
        ));
        Self {
            network: NetworkState::new("127.0.0.1:56552".to_string()).unwrap(),
            render,
            _phantom: PhantomData::default(),
        }
    }

    pub fn update(&mut self, duration: Duration) {
        // Uncapped or vsync frame rate
        self.network.update(duration).ok();
        self.render.camera.position = [0.0, 0.0, -5.0].into();
        self.render.camera.rotation = Quat::from_rotation_x(90.0_f32.to_radians());
        // self.render.camera.rotation = Quat::from_euler(EulerRot)
    }

    // Maybe return a vec of items that implement a trait? Ie something similiar to ggez drawable
    pub fn render(&mut self) -> &mut RenderState<S, M> {
        // Do non renderer specific rendering things here ie build chunk meshes, entity meshes/models, etc
        self.render
            .draws
            .push(crate::render::state::Draw { model_id: 0 });
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

use glam::Quat;

use crate::{
    input::InputState,
    network::state::NetworkState,
    render::{
        model::Model,
        state::{ConvertModel, RenderState},
    },
    state::GameState,
};
use std::{path::Path, time::Duration};

pub struct VinoxClient<S, M: ConvertModel<S>> {
    network: NetworkState,
    render: RenderState<S, M>,
    game: GameState<S, M>,
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
            game: GameState::new(),
        }
    }

    pub fn update(&mut self, duration: Duration) -> Result<(), String> {
        // Uncapped or vsync frame rate
        self.network.update(duration).ok();
        self.render.camera.position = [0.0, 0.0, -5.0].into();
        self.render.camera.rotation = Quat::from_rotation_x(90.0_f32.to_radians());
        self.game.update(duration)?;
        Ok(())
        // self.render.camera.rotation = Quat::from_euler(EulerRot)
    }

    // Maybe return a vec of items that implement a trait? Ie something similiar to ggez drawable
    pub fn render(&mut self) -> Result<&mut RenderState<S, M>, String> {
        // Do non renderer specific rendering things here ie build chunk meshes, entity meshes/models, etc
        self.game.render(&mut self.render)?;
        self.render
            .draws
            .push(crate::render::state::Draw { model_id: 0 });
        Ok(&mut self.render)
    }

    pub fn ui(&mut self, gui: &mut egui::Context) -> Result<(), String> {
        // Ui is a seperate function since render will only be used for things in a 3d environment
        self.game.ui(gui)?;
        Ok(())
    }

    pub fn tick(&mut self) -> Result<(), String> {
        self.game.tick()?;
        Ok(())
        // Fixed tick update function should be 30ticks per second
    }

    pub fn input(&mut self, input: &InputState) -> Result<(), String> {
        self.game.input(input)?;
        // Provide input state that is needed to vinox
        Ok(())
    }

    pub fn exit(&mut self) -> Result<(), String> {
        self.game.exit()?;
        self.network.exit();
        Ok(())
    }
}

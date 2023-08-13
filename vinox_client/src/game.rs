use glam::Quat;

use crate::{
    input::InputState,
    network::state::NetworkState,
    render::{
        model::Model,
        state::{ConvertModel, RenderState},
    },
    scene::{menu::MenuScene, SceneEvents, SceneStack, SceneSwitch},
};
use std::{
    path::Path,
    sync::{Arc, Mutex},
    time::Duration,
};

pub struct Context<S, M: ConvertModel<S>> {
    pub network: NetworkState,
    pub render: RenderState<S, M>,
    pub last_duration: Duration,
    pub input_state: InputState,
}
impl<S, M: ConvertModel<S>> Context<S, M> {
    pub fn new(state: &mut S) -> Self {
        let mut render = RenderState::<S, M>::default();
        render.asset_registry.models.push(M::to_mesh(
            Model::from_gltf(Path::new("vinox_client/assets/player.glb")).unwrap(),
            state,
        ));
        Self {
            network: NetworkState::new("127.0.0.1:56552".to_string()).unwrap(),
            render,
            last_duration: Duration::default(),
            input_state: InputState {},
        }
    }
}

pub struct SharedState {}

pub struct VinoxClient<S, M: ConvertModel<S>> {
    context: Context<S, M>,
    game: SceneStack<SharedState, SceneEvents, Context<S, M>>,
}

impl<S: 'static, M: ConvertModel<S> + 'static> VinoxClient<S, M> {
    pub fn new(state: &mut S) -> Self {
        let mut render = RenderState::<S, M>::default();
        render.asset_registry.models.push(M::to_mesh(
            Model::from_gltf(Path::new("vinox_client/assets/player.glb")).unwrap(),
            state,
        ));

        let mut context = Context::new(state);
        let mut game = SceneStack::new(&mut context, SharedState {});
        game.switch(SceneSwitch::push(MenuScene::new()));
        Self { game, context }
    }

    pub fn update(&mut self, duration: Duration) -> Result<(), String> {
        // Uncapped or vsync frame rate
        self.context.last_duration = duration;
        self.context.network.update(duration).ok();
        self.context.render.camera.position = [0.0, 0.0, -5.0].into();
        self.context.render.camera.rotation = Quat::from_rotation_x(90.0_f32.to_radians());
        self.game.update(&mut self.context);
        Ok(())
        // self.render.camera.rotation = Quat::from_euler(EulerRot)
    }

    // Maybe return a vec of items that implement a trait? Ie something similiar to ggez drawable
    pub fn render(&mut self) -> Result<&mut RenderState<S, M>, String> {
        // Do non renderer specific rendering things here ie build chunk meshes, entity meshes/models, etc
        self.game.render(&mut self.context);
        Ok(&mut self.context.render)
    }

    pub fn tick(&mut self) -> Result<(), String> {
        self.game.tick(&mut self.context);
        Ok(())
        // Fixed tick update function should be 30ticks per second
    }

    pub fn ui(&mut self, gui: &mut egui::Context) -> Result<(), String> {
        self.game.ui(gui, &mut self.context);
        Ok(())
    }

    pub fn input(&mut self, input: &InputState) -> Result<(), String> {
        self.context.input_state = input.clone();
        // Provide input state that is needed to vinox
        Ok(())
    }

    pub fn exit(&mut self) -> Result<(), String> {
        // self.game.()?;
        self.context.network.exit();
        Ok(())
    }
}

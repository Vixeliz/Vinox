use std::marker::PhantomData;

use crate::{
    game::{Context, SharedState},
    render::state::ConvertModel,
};

use super::{menu::MenuScene, Scene, SceneEvents, SceneSwitch};

pub struct GameScene<S, M: ConvertModel<S>> {
    _phantom: PhantomData<(S, M)>,
    switch: bool,
}

impl<S, M: ConvertModel<S>> GameScene<S, M> {
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData::default(),
            switch: false,
        }
    }
}

impl<S: 'static, M: ConvertModel<S> + 'static> Scene<SharedState, SceneEvents, Context<S, M>>
    for GameScene<S, M>
{
    fn update(
        &mut self,
        gameworld: &mut SharedState,
        ctx: &mut Context<S, M>,
    ) -> super::SceneSwitch<SharedState, SceneEvents, Context<S, M>> {
        if self.switch {
            SceneSwitch::replace(MenuScene::new())
        } else {
            SceneSwitch::None
        }
    }

    fn render(
        &mut self,
        gameworld: &mut SharedState,
        ctx: &mut Context<S, M>,
    ) -> Result<(), String> {
        ctx.render
            .draws
            .push(crate::render::state::Draw { model_id: 0 });

        Ok(())
    }

    fn tick(&mut self, gameworld: &mut SharedState, ctx: &mut Context<S, M>) -> Result<(), String> {
        Ok(())
    }

    fn input(
        &mut self,
        gameworld: &mut SharedState,
        event: SceneEvents,
        ctx: &mut Context<S, M>,
        started: bool,
    ) {
    }

    fn ui(&mut self, gameworld: &mut SharedState, ui: &mut egui::Context, ctx: &mut Context<S, M>) {
        egui::SidePanel::left("Game").show(ui, |ui| {
            ui.heading("Vinox");
            if ui.button("Menu").clicked() {
                self.switch = true;
            }
        });
    }

    fn name(&self) -> &str {
        "Game"
    }
}

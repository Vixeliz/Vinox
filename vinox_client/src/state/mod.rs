use crate::input::InputState;
use crate::render::state::{ConvertModel, RenderState};
use std::time::Duration;

pub mod menu;

pub struct GameState<S, M: ConvertModel<S>> {
    pub stack: Vec<Box<dyn State<S, M>>>,
}

impl<S, M: ConvertModel<S>> GameState<S, M> {
    pub fn new() -> Self {
        Self {
            stack: vec![Box::new(TestState::new())],
        }
    }

    pub fn push_state(&mut self, state: Box<dyn State<S, M>>) {
        self.stack.push(state);
    }

    pub fn pop_state(&mut self) {
        self.stack.pop();
    }

    // Maybe return a vec of items that implement a trait? Ie something similiar to ggez drawable
    pub fn render(&mut self, render_state: &mut RenderState<S, M>) -> Result<(), String> {
        self.current_state()?.render(render_state)?;
        Ok(())
    }

    pub fn ui(&mut self, gui: &mut egui::Context) -> Result<(), String> {
        self.current_state()?.ui(gui)?;
        Ok(())
    }

    pub fn tick(&mut self) -> Result<(), String> {
        self.current_state()?.tick()?;
        Ok(())
    }

    pub fn input(&mut self, input: &InputState) -> Result<(), String> {
        self.current_state()?.input(input)?;
        Ok(())
    }

    pub fn exit(&mut self) -> Result<(), String> {
        self.current_state()?.exit()?;
        Ok(())
    }

    pub fn update(&mut self, duration: Duration) -> Result<(), String> {
        self.current_state()?.update(duration)?;
        Ok(())
    }

    pub fn current_state(&mut self) -> Result<&mut Box<dyn State<S, M>>, String> {
        self.stack
            .last_mut()
            .ok_or("Failed to get current state".to_string())
    }
}

pub struct TestState;
impl TestState {
    pub fn new() -> Self {
        Self
    }
}

impl<S, M: ConvertModel<S>> State<S, M> for TestState {
    fn update(&mut self, duration: Duration) -> Result<(), String> {
        println!("{duration:?}");
        Ok(())
    }

    fn ui(&mut self, gui: &mut egui::Context) -> Result<(), String> {
        egui::SidePanel::new(egui::panel::Side::Left, "Title").show(gui, |ui| {
            ui.label("label");
            if ui.button("button").clicked() {
                println!("button clicked");
            }
        });
        Ok(())
    }
}

pub trait State<S, M: ConvertModel<S>> {
    fn update(&mut self, duration: Duration) -> Result<(), String> {
        Ok(())
    }

    // Maybe return a vec of items that implement a trait? Ie something similiar to ggez drawable
    fn render(&mut self, render_state: &mut RenderState<S, M>) -> Result<(), String> {
        Ok(())
    }

    fn ui(&mut self, gui: &mut egui::Context) -> Result<(), String> {
        Ok(())
    }

    fn tick(&mut self) -> Result<(), String> {
        Ok(())
    }

    fn input(&mut self, input: &InputState) -> Result<(), String> {
        Ok(())
    }

    fn exit(&mut self) -> Result<(), String> {
        Ok(())
    }
}

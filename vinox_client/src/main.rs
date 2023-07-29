#[macro_use]
extern crate cfg_if;

cfg_if! {
    if #[cfg(not(target_arch = "wasm32"))] {

        mod app;
        mod systems;

        use crate::app::App;
        use ggez::{
            event,
             GameResult,
        };


        fn main() -> GameResult {
            simple_logger::SimpleLogger::new()
                .with_module_level("wgpu", log::LevelFilter::Warn)
                .with_level(log::LevelFilter::Info)
                .init()
                .expect("A logger was already initialized");

            let cb = ggez::ContextBuilder::new("super_simple", "ggez");
            let (mut ctx, event_loop) = cb.build()?;
            let state = App::new(&mut ctx)?;
            event::run(ctx, event_loop, state)

        }
    }
}

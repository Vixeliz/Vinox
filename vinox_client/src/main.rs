use std::{env, path};

use ggez::*;
use ggez_state::GgezState;

mod game;
mod ggez_state;
mod input;

fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let cb = ContextBuilder::new("vinox", "vixeliz")
        .window_setup(conf::WindowSetup::default().title("Vinox"))
        .window_mode(
            conf::WindowMode::default()
                .dimensions(640.0, 480.0)
                .resizable(true),
        )
        .add_resource_path(resource_dir);

    let (mut ctx, events_loop) = cb.build()?;

    let game = GgezState::new(&mut ctx)?;
    event::run(ctx, events_loop, game)
}

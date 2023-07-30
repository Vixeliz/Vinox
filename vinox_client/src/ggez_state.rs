use ggez::*;

use crate::{game::VinoxClient, input::InputState};

pub struct GgezState {
    game: VinoxClient,
    input: InputState,
}

impl GgezState {
    pub fn new(_ctx: &mut Context) -> GameResult<GgezState> {
        Ok(GgezState {
            game: VinoxClient::new(),
            input: InputState::default(),
        })
    }
}

impl event::EventHandler for GgezState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.game.update(ctx.time.delta().as_secs_f32());
        self.game.input(&self.input);
        while ctx.time.check_update_time(30) {
            self.game.tick();
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        self.game.render();
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::BLACK);

        canvas.finish(ctx)?;

        Ok(())
    }
}

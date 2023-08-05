use crate::{
    glam::Vec2,
    render::state::{ConvertModel, RenderState},
};
use ggegui::Gui;
use ggez::{
    graphics::{DrawParam, DrawParam3d, Drawable3d},
    *,
};

use crate::{game::VinoxClient, input::InputState};

pub struct GgezMesh {}

impl ConvertModel for GgezMesh {
    fn to_mesh(model: crate::render::model::Model) -> Self {
        Self {}
    }
}

pub struct GgezState {
    game: VinoxClient<GgezMesh>,
    input: InputState,
    gui: Gui,
}

impl GgezState {
    pub fn new(ctx: &mut Context) -> GameResult<GgezState> {
        let mut gui = Gui::new(ctx);
        gui.input.set_scale_factor(1.5, ctx.gfx.drawable_size());
        Ok(GgezState {
            game: VinoxClient::new(),
            input: InputState::default(),
            gui,
        })
    }
}

impl event::EventHandler for GgezState {
    fn quit_event(&mut self, ctx: &mut Context) -> Result<bool, GameError> {
        self.game.exit();
        self.game.update(ctx.time.delta());
        Ok(false)
    }

    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.game.update(ctx.time.delta());
        self.game.input(&self.input);
        while ctx.time.check_update_time(30) {
            self.game.tick();
        }
        self.game.ui(&mut self.gui.ctx().context);
        self.gui.update(ctx);
        self.gui.input.set_scale_factor(1.5, ctx.gfx.size());

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let render_state = self.game.render();
        let scene_image = graphics::Image::new_canvas_image(
            ctx,
            ggez::graphics::ImageFormat::Rgba8Unorm,
            640,
            480,
            1,
        );

        let (width, height) = ctx.gfx.drawable_size();
        render_state.camera.resize(width as u32, height as u32);

        let mut canvas3d =
            graphics::Canvas3d::from_image(ctx, scene_image.clone(), graphics::Color::BLACK);

        canvas3d.set_projection(render_state.camera.to_matrix());

        canvas3d.draw(render_state, DrawParam3d::default());

        canvas3d.finish(ctx)?;

        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::BLACK);
        let params = DrawParam::default()
            .dest(Vec2::new(0.0, 0.0))
            .scale(Vec2::new(width / 640.0, height / 480.0));

        canvas.draw(&scene_image, params);

        canvas.draw(&self.gui, DrawParam::default().dest(glam::Vec2::ZERO));

        canvas.finish(ctx)?;

        render_state.clear();

        Ok(())
    }
}

impl Drawable3d for RenderState<GgezMesh> {
    fn draw(&self, canvas: &mut graphics::Canvas3d, param: impl Into<DrawParam3d>) {}
}

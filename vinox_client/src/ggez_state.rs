use crate::glam::{Vec2, Vec3};
use ggez::{
    graphics::{DrawParam, DrawParam3d, Mesh3dBuilder},
    *,
};

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
        let scene_image = graphics::Image::new_canvas_image(
            ctx,
            ggez::graphics::ImageFormat::Rgba8Unorm,
            640,
            480,
            1,
        );

        let mut camera = ggez::graphics::Camera3d::default();
        camera.transform.position = Vec3::new(-5.0, 0.0, -2.5).into();
        camera.transform.yaw = 0.0;
        camera.transform.pitch = 0.0;
        camera.projection.zfar = 1000.0;

        let mut canvas3d =
            graphics::Canvas3d::from_image(ctx, scene_image.clone(), graphics::Color::BLACK);

        canvas3d.set_projection(camera.to_matrix());

        canvas3d.draw(
            &Mesh3dBuilder::new().cube(Vec3::splat(2.0)).build(ctx),
            DrawParam3d::default().color(graphics::Color::WHITE),
        );

        canvas3d.finish(ctx)?;

        self.game.ui();
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::BLACK);
        let params = DrawParam::default()
            .dest(Vec2::new(0.0, 0.0))
            .scale(Vec2::new(
                ctx.gfx.drawable_size().0 / 640.0,
                ctx.gfx.drawable_size().1 / 480.0,
            ));

        canvas.draw(&scene_image, params);

        canvas.finish(ctx)?;

        Ok(())
    }
}
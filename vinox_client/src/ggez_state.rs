use crate::{
    glam::Vec2,
    render::{
        model::Animation,
        state::{ConvertModel, RenderState},
    },
};
use ggegui::Gui;
use ggez::{
    graphics::{
        DrawParam, DrawParam3d, Drawable3d, ImageFormat, Mesh3d, Mesh3dBuilder, Sampler, Shader,
        Vertex3d,
    },
    *,
};
use glam::Vec3;

use crate::{game::VinoxClient, input::InputState};

pub struct GgezModel {
    model: graphics::Model,
    animations: Vec<Animation>,
}

impl ConvertModel<Context> for GgezModel {
    fn to_mesh(model: crate::render::model::Model, state: &mut Context) -> Self {
        Self {
            model: graphics::Model {
                meshes: model
                    .meshes
                    .into_iter()
                    .map(|x| {
                        Mesh3dBuilder::new()
                            .from_data(
                                x.vertices
                                    .into_iter()
                                    .map(|x| {
                                        Vertex3d::new(
                                            x.pos,
                                            x.tex_coord,
                                            Some(graphics::Color::from(x.color)),
                                            x.normals,
                                        )
                                    })
                                    .collect(),
                                x.indices,
                                x.texture.map(|x| {
                                    graphics::Image::from_pixels(
                                        state,
                                        x.to_vec().as_slice(),
                                        ImageFormat::Rgba8UnormSrgb,
                                        x.width(),
                                        x.height(),
                                    )
                                }),
                            )
                            .build(state)
                    })
                    .collect(),
                aabb: None,
            },
            animations: vec![],
        }
    }
}

pub struct GgezState {
    game: VinoxClient<Context, GgezModel>,
    input: InputState,
    gui: Gui,
    shader: Shader,
    psx_shader: Shader,
    crt_shader: Shader,
}

impl GgezState {
    pub fn new(ctx: &mut Context) -> GameResult<GgezState> {
        let mut gui = Gui::new(ctx);
        gui.input.set_scale_factor(1.5, ctx.gfx.drawable_size());
        Ok(GgezState {
            game: VinoxClient::new(ctx),
            input: InputState::default(),
            gui,
            shader: graphics::ShaderBuilder::from_path("/shaders/shader.wgsl")
                .build(&ctx.gfx)
                .unwrap(),
            psx_shader: graphics::ShaderBuilder::from_path("/shaders/psx.wgsl")
                .build(&ctx.gfx)
                .unwrap(),
            crt_shader: graphics::ShaderBuilder::new()
                .fragment_path("/shaders/crt.wgsl")
                .build(&ctx.gfx)
                .unwrap(),
        })
    }
}

impl event::EventHandler for GgezState {
    fn quit_event(&mut self, ctx: &mut Context) -> Result<bool, GameError> {
        self.game
            .exit()
            .map_err(|x| GameError::CustomError(x.to_string()))?;
        self.game
            .update(ctx.time.delta())
            .map_err(|x| GameError::CustomError(x.to_string()))?;
        Ok(false)
    }

    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.game
            .update(ctx.time.delta())
            .map_err(|x| GameError::CustomError(x.to_string()))?;
        self.game
            .input(&self.input)
            .map_err(|x| GameError::CustomError(x.to_string()))?;
        while ctx.time.check_update_time(30) {
            self.game
                .tick()
                .map_err(|x| GameError::CustomError(x.to_string()))?;
        }
        self.game
            .ui(&mut self.gui.ctx().context)
            .map_err(|x| GameError::CustomError(x.to_string()))?;
        self.gui.update(ctx);
        self.gui.input.set_scale_factor(1.5, ctx.gfx.size());

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let render_state = self
            .game
            .render()
            .map_err(|x| GameError::CustomError(x.to_string()))?;

        let (width, height) = ctx.gfx.drawable_size();
        let mut scene_image =
            graphics::ScreenImage::new(ctx, ggez::graphics::ImageFormat::Rgba8Unorm, 1.0, 1.0, 1);

        render_state.camera.resize(width as u32, height as u32);

        let mut canvas3d =
            graphics::Canvas3d::from_screen_image(ctx, &mut scene_image, graphics::Color::BLACK);
        canvas3d.set_sampler(Sampler::nearest_clamp());
        canvas3d.set_shader(&self.shader);

        canvas3d.set_projection(render_state.camera.to_matrix());

        canvas3d.draw(render_state, DrawParam3d::default());

        canvas3d.finish(ctx)?;

        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::BLACK);
        let params = DrawParam::default().dest(Vec2::new(0.0, 0.0));
        canvas.set_shader(&self.crt_shader);

        canvas.draw(&scene_image.image(ctx), params);
        canvas.set_default_shader();

        canvas.draw(&self.gui, DrawParam::default().dest(glam::Vec2::ZERO));

        canvas.finish(ctx)?;

        render_state.clear();

        Ok(())
    }
}

impl Drawable3d for RenderState<Context, GgezModel> {
    fn draw(&self, canvas: &mut graphics::Canvas3d, param: impl Into<DrawParam3d>) {
        let param: DrawParam3d = param.into();
        for draw in self.draws.iter() {
            if let Some(model) = self.asset_registry.models.get(draw.model_id as usize) {
                canvas.draw(&model.model, param);
            }
        }
    }
}

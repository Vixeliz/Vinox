use std::{collections::HashMap, ops::Deref};

use ggez::{
    graphics::{self, DrawParam},
    mint, Context, GameResult,
};
use log::info;

use hecs::Entity;

use naia_hecs_client::{Client as NaiaClient, ClientConfig, WorldWrapper as World};

use vinox_common::{protocol, Auth, Position};

use super::systems::{events::process_events, startup::app_init};

pub type Client = NaiaClient<Entity>;

pub struct App {
    pub client: Client,
    pub world: World,
    pub message_count: u32,
    pub entity_to_id_map: HashMap<Entity, u32>,
    pub next_id: u32,
}

impl App {
    pub fn default() -> Self {
        info!("Naia Hecs Client Demo started");

        app_init(
            ClientConfig::default(),
            protocol(),
            "http://127.0.0.1:14191",
            Auth::new("charlie", "12345"),
        )
    }

    pub fn new(_: &mut Context) -> GameResult<Self> {
        Ok(Self::default())
    }

    pub fn update(&mut self) {
        process_events(self);
    }

    pub fn tick(&mut self) {
        //info!("tick");
    }
}

impl ggez::event::EventHandler for App {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.update();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));

        let white_image = graphics::Image::from_color(ctx, 16, 16, Some(graphics::Color::WHITE));
        for (_, position) in self.world.query_mut::<&mut Position>() {
            canvas.draw(
                &white_image,
                DrawParam::new().dest(mint::Point2::<f32> {
                    x: position.x.deref().0.to_num::<f32>(),
                    y: position.y.deref().0.to_num::<f32>(),
                }),
            )
        }

        canvas.finish(ctx)?;

        Ok(())
    }
}

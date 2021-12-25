pub mod logic;

use crate::{event_handler::EventHandler, fps::FPSHandler, game_mode::GameMode};

use rsfml::graphics::RenderWindow;

pub struct GameLoop<'s> {
    game_mode: GameMode<'s>,
    event_handler: EventHandler,
    render_window: RenderWindow,
    fps_handler: Option<FPSHandler<'s>>,
}

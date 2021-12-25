pub mod logic;

use rsfml::{graphics::Text, system::Clock};

pub struct FPSHandler<'s> {
    fps_clock: Clock,
    text: Text<'s>,
}

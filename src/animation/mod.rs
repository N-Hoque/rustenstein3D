pub mod logic;

use rsfml::system::Clock;

#[derive(Clone, Copy)]
pub enum PlayState {
    Play,
    Stop,
}

struct Data {
    start_tid: u32,
    offset: u32,
    texture_ids: Vec<i32>,
    lag: f32,
}

pub struct Animation {
    state: PlayState,
    data: Data,
    active_texture: u32,
    clock: Clock,
}

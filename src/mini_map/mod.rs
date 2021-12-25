pub mod logic;

use rsfml::{graphics::View, system::Vector2f, SfBox};

use crate::map::Map;

pub struct MiniMap {
    map: Map,
    active: bool,
    mini_map_view: SfBox<View>,
    player_pos: Vector2f,
    rotation: f32,
}

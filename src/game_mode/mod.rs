pub mod logic;

use rsfml::{graphics::RectangleShape, system::Vector2u};

use crate::{
    hud::HUD, mini_map::MiniMap, raycasting_engine::REngine, texture_loader::TextureLoader,
    weapon::Weapon,
};

pub struct GameMode<'s> {
    window_size: Vector2u,
    mini_map: MiniMap,
    r_engine: REngine,
    texture_loader: &'s TextureLoader,
    hud: HUD<'s>,
    weapon: Weapon<'s>,
    sky: RectangleShape<'s>,
    ground: RectangleShape<'s>,
}

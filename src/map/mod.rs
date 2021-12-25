pub mod logic;

use rsfml::system::Vector2i;

#[derive(Clone)]
pub struct Map {
    map: Vec<i32>,
    map_size: Vector2i,
}

pub enum Orientation {
    Top,
    Bottom,
    Left,
    Right,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

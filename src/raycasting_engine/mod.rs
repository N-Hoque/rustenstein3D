pub mod logic;

use rsfml::{
    graphics::VertexArray,
    system::{Vector2f, Vector2i},
};

use crate::map::Map;

pub struct REngine {
    player_position: Vector2f,
    vector_direction: Vector2f,
    cam_plane: Vector2f,
    map: Map,
    window_size: Vector2f,
    vertex_array: Vec<VertexArray>,
    textures_id: Vec<i32>,
    ground: Vec<VertexArray>,
    sky: Vec<VertexArray>,
    no_ground: bool,
    draw_state: DrawState,
}

#[derive(Default)]
struct DrawState {
    map_pos: Vector2i,
    step: Vector2i,
    side_dist: Vector2f,
    draw_start: i32,
    draw_end: i32,
    perp_wall_dist: f32,
    wall_x: f32,
}

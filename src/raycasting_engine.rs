use rsfml::{
    graphics::{
        Color, PrimitiveType, RenderStates, RenderTarget, RenderWindow, Vertex, VertexBuffer,
        VertexBufferUsage,
    },
    system::{Vector2f, Vector2i},
    window::Key,
};

use crate::{
    core::{EventUpdate, TextureRender},
    event_handler::EventHandler,
    map::Map,
    texture_loader::TextureLoader,
};

pub struct REngine<'m> {
    window_size: Vector2f,
    textures_id: Vec<i32>,
    vertex_arrays: VertexData,
    player_data: PlayerData,
    map: Map<'m>,
    disable_planes: bool,
    draw_state: DrawState,
}

struct PlayerData {
    position: Vector2f,
    direction: Vector2f,
    camera_plane: Vector2f,
}

struct VertexData {
    vertices: Box<[VertexBuffer]>,
    ground: Box<[VertexBuffer]>,
    sky: Box<[VertexBuffer]>,
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

#[derive(Clone, Copy, PartialEq, Eq)]
enum Side {
    Left,
    Right,
}

impl<'m> REngine<'m> {
    pub(crate) fn new(map: Map<'m>, window_size: Vector2f) -> Self {
        Self {
            map,
            disable_planes: false,
            draw_state: DrawState::default(),
            window_size: Vector2f::new(window_size.x, window_size.y - 80.),
            textures_id: Vec::new(),
            player_data: PlayerData {
                position: Vector2f::new(22., 12.),
                direction: Vector2f::new(-1., 0.),
                camera_plane: Vector2f::new(0., 0.66),
            },
            vertex_arrays: VertexData {
                vertices: Self::create_line_array(window_size, 2),
                ground: Self::create_line_array(window_size, 0),
                sky: Self::create_line_array(window_size, 0),
            },
        }
    }

    pub(crate) fn disable_planes(&mut self) {
        self.disable_planes = true;
    }
}

impl REngine<'_> {
    pub const fn get_player_position(&self) -> Vector2f {
        self.player_data.position
    }

    fn update_planes(&mut self, ray_dir: Vector2f, side: Side, pixel: i32) {
        if self.draw_state.draw_end < 0 {
            self.draw_state.draw_end = self.window_size.y as i32;
        }

        self.reset_plane_buffers(pixel);

        let (sky, ground) = self.compute_plane_buffers(pixel, ray_dir, side);

        self.update_plane_buffers(pixel, &sky, &ground);
    }

    fn compute_plane_buffers(
        &self,
        pixel: i32,
        ray_dir: Vector2f,
        side: Side,
    ) -> (Vec<Vertex>, Vec<Vertex>) {
        let pixel = pixel as f32;
        let plane_normal = self.compute_plane_normal(ray_dir, side);

        let texture_data = self.compute_texture_data(plane_normal);

        let sky_vertices = texture_data
            .iter()
            .map(|(idx, tex)| {
                let pos = Self::compute_plane_vertex(pixel, *idx, Some(self.window_size.y));
                Vertex::new(pos, Color::WHITE, *tex)
            })
            .collect();

        let ground_vertices = texture_data
            .iter()
            .map(|(idx, tex)| {
                let pos = Self::compute_plane_vertex(pixel, *idx, None);
                Vertex::new(pos, Color::WHITE, *tex)
            })
            .collect();

        (sky_vertices, ground_vertices)
    }

    fn compute_texture_data(&self, plane_normal: Vector2f) -> Box<[(f32, Vector2f)]> {
        ((self.draw_state.draw_end + 1)..(self.window_size.y as i32))
            .map(|idx| {
                let vertex_index = idx as f32;
                (
                    vertex_index,
                    self.compute_texture_coordinate(vertex_index, plane_normal),
                )
            })
            .collect()
    }

    fn compute_plane_vertex(pixel: f32, vertex_index: f32, window_height: Option<f32>) -> Vector2f {
        Vector2f {
            x: pixel,
            y: window_height.map_or(vertex_index, |h| h - vertex_index),
        }
    }

    fn get_buffer_size(&self) -> u32 {
        ((self.draw_state.draw_end + 1)..(self.window_size.y as i32)).count() as u32
    }

    fn update_plane_buffers(
        &mut self,
        pixel: i32,
        sky_vertices: &[Vertex],
        ground_vertices: &[Vertex],
    ) {
        if let Some(sky_buffer) = self.vertex_arrays.sky.get_mut(pixel as usize) {
            sky_buffer.update(sky_vertices, 0);
        }
        if let Some(ground_buffer) = self.vertex_arrays.ground.get_mut(pixel as usize) {
            ground_buffer.update(ground_vertices, 0);
        }
    }

    fn reset_plane_buffers(&mut self, pixel: i32) {
        let buffer_size = self.get_buffer_size();

        if let Some(sky_buffer) = self.vertex_arrays.sky.get_mut(pixel as usize) {
            *sky_buffer = Self::create_new_vertex_buffer(buffer_size);
        }
        if let Some(ground_buffer) = self.vertex_arrays.ground.get_mut(pixel as usize) {
            *ground_buffer = Self::create_new_vertex_buffer(buffer_size);
        }
    }

    fn create_new_vertex_buffer(buffer_size: u32) -> VertexBuffer {
        VertexBuffer::new(
            PrimitiveType::POINTS,
            buffer_size,
            VertexBufferUsage::STREAM,
        )
    }

    fn compute_texture_coordinate(&self, vertex_index: f32, plane_normal: Vector2f) -> Vector2f {
        let window_height = self.window_size.y;
        let wall_distance = self.draw_state.perp_wall_dist;
        let player_position = self.player_data.position;
        let current_dist = window_height / (2. * vertex_index - window_height);
        let weight = current_dist / wall_distance;
        let texture_coordinates = (plane_normal * weight) + (player_position * (1.0 - weight));

        Vector2f::new(
            ((texture_coordinates.x * 128.) as i32 % 128) as f32,
            ((texture_coordinates.y * 128.) as i32 % 128) as f32,
        )
    }

    fn compute_plane_normal(&self, ray_dir: Vector2f, side: Side) -> Vector2f {
        let map_pos = Vector2f::new(
            self.draw_state.map_pos.x as f32,
            self.draw_state.map_pos.y as f32,
        );

        match side {
            Side::Left => (
                map_pos.x + if ray_dir.x < 0. { 1. } else { 0. },
                map_pos.y + self.draw_state.wall_x,
            ),
            Side::Right => (
                map_pos.x + self.draw_state.wall_x,
                map_pos.y + if ray_dir.y < 0. { 1. } else { 0. },
            ),
        }
        .into()
    }

    fn update_wall_width(&mut self, ray_pos: Vector2f, ray_dir: Vector2f, side: Side) {
        self.draw_state.wall_x = if side == Side::Left {
            ((self.draw_state.map_pos.x as f32 - ray_pos.x
                + (1. - self.draw_state.step.x as f32) / 2.)
                / ray_dir.x)
                .mul_add(ray_dir.y, ray_pos.y)
        } else {
            ((self.draw_state.map_pos.y as f32 - ray_pos.y
                + (1. - self.draw_state.step.y as f32) / 2.)
                / ray_dir.y)
                .mul_add(ray_dir.x, ray_pos.x)
        };
        self.draw_state.wall_x -= self.draw_state.wall_x.floor();
    }

    fn update_wall_height(&mut self, ray_pos: Vector2f, ray_dir: Vector2f, side: Side) {
        self.draw_state.perp_wall_dist = if side == Side::Left {
            (self.draw_state.map_pos.x as f32 - ray_pos.x
                + (1 - self.draw_state.step.x) as f32 / 2.)
                / ray_dir.x
        } else {
            (self.draw_state.map_pos.y as f32 - ray_pos.y
                + (1 - self.draw_state.step.y) as f32 / 2.)
                / ray_dir.y
        }
        .abs();

        let line_height: i32 = if self.draw_state.perp_wall_dist as i32 == 0 {
            self.window_size.y as i32
        } else {
            ((self.window_size.y / self.draw_state.perp_wall_dist) as i32).abs()
        };

        self.draw_state.draw_start = ((self.window_size.y as i32 / 2) - (line_height / 2)).max(0);

        self.draw_state.draw_end =
            (line_height / 2 + self.window_size.y as i32 / 2).min(self.window_size.y as i32 - 1);
    }

    fn update_wall_texture(
        &mut self,
        ray_pos: Vector2f,
        ray_dir: Vector2f,
        side: Side,
        pixel: i32,
    ) {
        self.update_wall_width(ray_pos, ray_dir, side);

        self.update_texture_ids(side);

        let texture_x = self.compute_texture_width(ray_dir, side) as f32;

        self.update_main_vertex_buffer(pixel as f32, texture_x);
    }

    fn compute_texture_width(&mut self, ray_dir: Vector2f, side: Side) -> i32 {
        let mut texture_x = (self.draw_state.wall_x * 128.) as i32;
        if side == Side::Left && ray_dir.x > 0. {
            texture_x = 128 - texture_x - 1;
        }
        if side == Side::Right && ray_dir.y < 0. {
            texture_x = 128 - texture_x - 1;
        }
        texture_x
    }

    fn update_texture_ids(&mut self, side: Side) {
        if let Some(mut texture_id) = self.map.get_block(self.draw_state.map_pos) {
            if side == Side::Right {
                texture_id += 5;
            }
            self.textures_id.push(texture_id);
        }
    }

    fn update_main_vertex_buffer(&mut self, pixel: f32, tex_x_coordinate: f32) {
        if let Some(current_vertex_array) = self.vertex_arrays.vertices.get_mut(pixel as usize) {
            current_vertex_array.update(
                &[
                    Vertex::new(
                        Vector2f::new(pixel, self.draw_state.draw_start as f32),
                        Color::WHITE,
                        Vector2f::new(tex_x_coordinate, 0.),
                    ),
                    Vertex::new(
                        Vector2f::new(pixel, self.draw_state.draw_end as f32),
                        Color::WHITE,
                        Vector2f::new(tex_x_coordinate, 128.),
                    ),
                ],
                0,
            );
        }
    }

    fn update_step(&mut self, ray_pos: Vector2f, ray_dir: Vector2f, delta_dist: Vector2f) {
        (self.draw_state.step.x, self.draw_state.side_dist.x) = if ray_dir.x < 0. {
            (
                -1,
                (ray_pos.x - self.draw_state.map_pos.x as f32) * delta_dist.x,
            )
        } else {
            (
                1,
                (self.draw_state.map_pos.x as f32 + 1. - ray_pos.x) * delta_dist.x,
            )
        };

        (self.draw_state.step.y, self.draw_state.side_dist.y) = if ray_dir.y < 0. {
            (
                -1,
                (ray_pos.y - self.draw_state.map_pos.y as f32) * delta_dist.y,
            )
        } else {
            (
                1,
                (self.draw_state.map_pos.y as f32 + 1. - ray_pos.y) * delta_dist.y,
            )
        };
    }

    fn update_side(&mut self, side: &mut Side, delta_dist: Vector2f) {
        while matches!(self.map.get_block(self.draw_state.map_pos), Some(0)) {
            if self.draw_state.side_dist.x < self.draw_state.side_dist.y {
                self.draw_state.side_dist.x += delta_dist.x;
                self.draw_state.map_pos.x += self.draw_state.step.x;
                *side = Side::Left;
            } else {
                self.draw_state.side_dist.y += delta_dist.y;
                self.draw_state.map_pos.y += self.draw_state.step.y;
                *side = Side::Right;
            }
        }
    }

    fn update_direction(&mut self, event_handler: &EventHandler) {
        let direction = if let Some((x, _)) = event_handler.has_mouse_moved_event() {
            x as f32 - (self.window_size.x / 2.)
        } else {
            0.
        } / -250.;

        let mouse_cos = direction.cos();
        let mouse_sin = direction.sin();
        let old_dir_x = self.player_data.direction.x;
        let old_cam_plane_x = self.player_data.camera_plane.x;

        self.player_data.direction.x =
            self.player_data.direction.x * mouse_cos - self.player_data.direction.y * mouse_sin;

        self.player_data.direction.y =
            old_dir_x.mul_add(mouse_sin, self.player_data.direction.y * mouse_cos);

        self.player_data.camera_plane.x = self.player_data.camera_plane.x * mouse_cos
            - self.player_data.camera_plane.y * mouse_sin;

        self.player_data.camera_plane.y =
            old_cam_plane_x.mul_add(mouse_sin, self.player_data.camera_plane.y * mouse_cos);
    }

    fn update_by_key(&mut self, key: Key) {
        let multiplier = match key {
            Key::W => 0.1,
            Key::S => -0.1,
            _ => return,
        };

        if EventHandler::is_key_pressed(key) {
            let pos = self.player_data.position + (self.player_data.direction * multiplier);
            if self
                .map
                .get_block(Vector2i::new(pos.x as i32, pos.y as i32))
                == Some(0)
            {
                self.player_data.position = pos;
            }
        }
    }

    fn update_position(&mut self) {
        self.update_by_key(Key::W);
        self.update_by_key(Key::S);
    }

    fn create_line_array(window_size: Vector2f, vertex_count: u32) -> Box<[VertexBuffer]> {
        (0..window_size.x as usize)
            .map(|_| {
                VertexBuffer::new(
                    PrimitiveType::LINES,
                    vertex_count,
                    VertexBufferUsage::STREAM,
                )
            })
            .collect()
    }

    fn update_map_position(&mut self, ray_pos: Vector2f) {
        self.draw_state.map_pos = Vector2i::new(ray_pos.x as i32, ray_pos.y as i32);
    }

    fn compute_events(&mut self, pixel: i32) -> (Vector2f, Vector2f, Vector2f) {
        let camera_x = 2. * pixel as f32 / self.window_size.x - 1.;
        let ray_pos = self.player_data.position;
        let ray_dir = self.player_data.direction + (self.player_data.camera_plane * camera_x);
        let delta_dist = Vector2f::new(
            (1. + (ray_dir.y * ray_dir.y) / (ray_dir.x * ray_dir.x)).sqrt(),
            (1. + (ray_dir.x * ray_dir.x) / (ray_dir.y * ray_dir.y)).sqrt(),
        );
        (ray_pos, ray_dir, delta_dist)
    }

    fn update_events(
        &mut self,
        ray_pos: Vector2f,
        ray_dir: Vector2f,
        delta_dist: Vector2f,
        pixel: i32,
    ) {
        self.update_map_position(ray_pos);
        self.update_step(ray_pos, ray_dir, delta_dist);
        let mut side = Side::Left;
        self.update_side(&mut side, delta_dist);
        self.update_wall_height(ray_pos, ray_dir, side);
        self.update_wall_texture(ray_pos, ray_dir, side, pixel);
        if !self.disable_planes {
            self.update_planes(ray_dir, side, pixel);
        }
    }
}

impl TextureRender for REngine<'_> {
    fn draw(&self, render_window: &mut RenderWindow, texture_loader: &TextureLoader) {
        let mut render_states = RenderStates::default();

        for (idx, line) in self.vertex_arrays.vertices.iter().enumerate() {
            render_states.set_texture(Some(texture_loader.get_texture(self.textures_id[idx])));
            render_window.draw_with_renderstates(line, &render_states);
        }

        render_states.set_texture(Some(texture_loader.get_texture(0)));
        for gr in self.vertex_arrays.ground.iter() {
            render_window.draw_with_renderstates(gr, &render_states);
        }

        render_states.set_texture(Some(texture_loader.get_texture(11)));
        for sky in self.vertex_arrays.sky.iter() {
            render_window.draw_with_renderstates(sky, &render_states);
        }
    }
}

impl EventUpdate for REngine<'_> {
    fn update(&mut self, event_handler: &EventHandler) {
        self.textures_id.clear();
        for pixel in 0..self.window_size.x as i32 {
            let (ray_pos, ray_dir, delta_dist) = self.compute_events(pixel);

            self.update_events(ray_pos, ray_dir, delta_dist, pixel);
        }
        self.update_direction(event_handler);
        self.update_position();
    }
}

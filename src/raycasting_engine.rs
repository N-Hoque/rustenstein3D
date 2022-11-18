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
    no_ground: bool,
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
            no_ground: false,
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

    pub(crate) fn disable_ground(&mut self) {
        self.no_ground = true;
    }
}

impl REngine<'_> {
    pub const fn get_player_position(&self) -> Vector2f {
        self.player_data.position
    }

    fn update_planes(&mut self, ray_dir: Vector2f, side: Side, width_pixel: i32) {
        if self.draw_state.draw_end < 0 {
            self.draw_state.draw_end = self.window_size.y as i32;
        }

        let buffer_size = self.get_buffer_size();

        self.reset_plane_buffers(width_pixel, buffer_size);

        let plane_normal = self.compute_plane_normal(ray_dir, side);

        let (sky_vertices, ground_vertices) =
            self.compute_plane_vertices(buffer_size as usize, plane_normal, width_pixel as f32);

        self.update_plane_buffers(width_pixel, &sky_vertices, &ground_vertices);
    }

    fn compute_plane_vertices(
        &self,
        buffer_size: usize,
        plane_normal: Vector2f,
        width_pixel: f32,
    ) -> (Vec<Vertex>, Vec<Vertex>) {
        let mut sky_vertices = Vec::with_capacity(buffer_size);
        let mut ground_vertices = Vec::with_capacity(buffer_size);
        for vertex_index in (self.draw_state.draw_end + 1)..(self.window_size.y as i32) {
            let vertex_index = vertex_index as f32;
            let tex_coord = Self::compute_texture_coordinate(
                vertex_index,
                self.window_size.y,
                self.draw_state.perp_wall_dist,
                plane_normal,
                self.player_data.position,
            );

            let pos =
                Self::compute_plane_vertex(width_pixel, vertex_index, Some(self.window_size.y));
            Self::update_plane_vertices(&mut sky_vertices, pos, tex_coord);

            let pos = Self::compute_plane_vertex(width_pixel, vertex_index, None);
            Self::update_plane_vertices(&mut ground_vertices, pos, tex_coord);
        }
        (sky_vertices, ground_vertices)
    }

    fn compute_plane_vertex(
        width_pixel: f32,
        vertex_index: f32,
        window_height: Option<f32>,
    ) -> Vector2f {
        Vector2f {
            x: width_pixel,
            y: window_height.map_or(vertex_index, |h| h - vertex_index),
        }
    }

    fn update_plane_vertices(
        vertices: &mut Vec<Vertex>,
        position: Vector2f,
        texture_coordinate: Vector2f,
    ) {
        let vertex = Vertex::new(position, Color::WHITE, texture_coordinate);
        vertices.push(vertex);
    }

    fn get_buffer_size(&self) -> u32 {
        ((self.draw_state.draw_end + 1)..(self.window_size.y as i32)).count() as u32
    }

    fn update_plane_buffers(
        &mut self,
        width_pixel: i32,
        sky_vertices: &[Vertex],
        ground_vertices: &[Vertex],
    ) {
        if let Some(sky_buffer) = self.vertex_arrays.sky.get_mut(width_pixel as usize) {
            Self::update_buffer(sky_buffer, sky_vertices);
        }
        if let Some(ground_buffer) = self.vertex_arrays.ground.get_mut(width_pixel as usize) {
            Self::update_buffer(ground_buffer, ground_vertices);
        }
    }

    fn update_buffer(buffer: &mut VertexBuffer, vertices: &[Vertex]) {
        buffer.update(vertices, 0);
    }

    fn reset_plane_buffers(&mut self, width_pixel: i32, buffer_size: u32) {
        if let Some(sky_buffer) = self.vertex_arrays.sky.get_mut(width_pixel as usize) {
            Self::reset_buffer(sky_buffer, buffer_size);
        }
        if let Some(ground_buffer) = self.vertex_arrays.ground.get_mut(width_pixel as usize) {
            Self::reset_buffer(ground_buffer, buffer_size);
        }
    }

    fn reset_buffer(buffer: &mut VertexBuffer, buffer_size: u32) {
        *buffer = VertexBuffer::new(
            PrimitiveType::POINTS,
            buffer_size,
            VertexBufferUsage::STREAM,
        );
    }

    fn compute_texture_coordinate(
        vertex_index: f32,
        window_height: f32,
        wall_distance: f32,
        plane_normal: Vector2f,
        player_position: Vector2f,
    ) -> Vector2f {
        let current_dist = window_height / (2. * vertex_index - window_height);
        let weight = current_dist / wall_distance;
        let plane = (plane_normal * weight) + (player_position * (1.0 - weight));

        Vector2f::new(
            ((plane.x * 128.) as i32 % 128) as f32,
            ((plane.y * 128.) as i32 % 128) as f32,
        )
    }

    fn compute_plane_normal(&mut self, ray_dir: Vector2f, side: Side) -> Vector2f {
        let map_pos = Vector2f::new(
            self.draw_state.map_pos.x as f32,
            self.draw_state.map_pos.y as f32,
        );

        match side {
            Side::Left if ray_dir.x >= 0.0 => (map_pos.x, map_pos.y + self.draw_state.wall_x),
            Side::Left => (map_pos.x + 1., map_pos.y + self.draw_state.wall_x),
            Side::Right if ray_dir.y >= 0.0 => (map_pos.x + self.draw_state.wall_x, map_pos.y),
            Side::Right => (map_pos.x + self.draw_state.wall_x, map_pos.y + 1.),
        }
        .into()
    }

    fn update_wall_width(&mut self, ray_pos: Vector2f, ray_dir: Vector2f, side: Side) {
        self.draw_state.wall_x = if side == Side::Right {
            ((self.draw_state.map_pos.y as f32 - ray_pos.y
                + (1. - self.draw_state.step.y as f32) / 2.)
                / ray_dir.y)
                .mul_add(ray_dir.x, ray_pos.x)
        } else {
            ((self.draw_state.map_pos.x as f32 - ray_pos.x
                + (1. - self.draw_state.step.x as f32) / 2.)
                / ray_dir.x)
                .mul_add(ray_dir.y, ray_pos.y)
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
        width_pixel: i32,
    ) {
        self.update_wall_width(ray_pos, ray_dir, side);

        self.update_texture_ids(side);

        let texture_x = self.compute_texture_width(ray_dir, side);

        self.update_main_vertex_buffer(width_pixel, texture_x);
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

    fn update_main_vertex_buffer(&mut self, width_pixel: i32, tex_x_coordinate: i32) {
        if let Some(current_vertex_array) =
            self.vertex_arrays.vertices.get_mut(width_pixel as usize)
        {
            current_vertex_array.update(
                &[
                    Vertex::new(
                        Vector2f::new(width_pixel as f32, self.draw_state.draw_start as f32),
                        Color::WHITE,
                        Vector2f::new(tex_x_coordinate as f32, 0.),
                    ),
                    Vertex::new(
                        Vector2f::new(width_pixel as f32, self.draw_state.draw_end as f32),
                        Color::WHITE,
                        Vector2f::new(tex_x_coordinate as f32, 128.),
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

    fn update_wall_side(&mut self, side: &mut Side, delta_dist: Vector2f) {
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
        for width_pixel in 0..self.window_size.x as i32 {
            // compute
            let camera_x = 2. * width_pixel as f32 / self.window_size.x - 1.;

            let ray_pos = self.player_data.position;
            let ray_dir = self.player_data.direction + (self.player_data.camera_plane * camera_x);

            let delta_dist = Vector2f::new(
                (1. + (ray_dir.y * ray_dir.y) / (ray_dir.x * ray_dir.x)).sqrt(),
                (1. + (ray_dir.x * ray_dir.x) / (ray_dir.y * ray_dir.y)).sqrt(),
            );

            // update
            self.update_map_position(ray_pos);

            self.update_step(ray_pos, ray_dir, delta_dist);

            let mut side = Side::Left;
            self.update_wall_side(&mut side, delta_dist);

            self.update_wall_height(ray_pos, ray_dir, side);

            self.update_wall_texture(ray_pos, ray_dir, side, width_pixel);

            if !self.no_ground {
                self.update_planes(ray_dir, side, width_pixel);
            }
        }
        self.update_direction(event_handler);
        self.update_position();
    }
}

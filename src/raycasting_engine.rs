use rsfml::{
    graphics::{
        Color, PrimitiveType, RenderStates, RenderTarget, RenderWindow, Vertex, VertexArray,
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
    vertex_data: VertexData,
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
    vertices: Box<[VertexArray]>,
    ground: Box<[VertexArray]>,
    sky: Box<[VertexArray]>,
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

impl<'m> REngine<'m> {
    pub(crate) fn new(map: Map<'m>, window_size: Vector2f, no_ground: bool) -> Self {
        Self {
            map,
            no_ground,
            draw_state: DrawState::default(),
            window_size: Vector2f::new(window_size.x, window_size.y - 80.),
            textures_id: Vec::new(),
            player_data: PlayerData {
                position: Vector2f::new(22., 12.),
                direction: Vector2f::new(-1., 0.),
                camera_plane: Vector2f::new(0., 0.66),
            },
            vertex_data: VertexData {
                vertices: Self::create_line_array(window_size),
                ground: Self::create_line_array(window_size),
                sky: Self::create_line_array(window_size),
            },
        }
    }
}

impl REngine<'_> {
    fn calculate_ground(&mut self, ray_dir: Vector2f, side: i32, width_pixel: i32) {
        if self.draw_state.draw_end < 0 {
            self.draw_state.draw_end = self.window_size.y as i32;
        }

        self.vertex_data
            .ground
            .get_mut(width_pixel as usize)
            .unwrap_or_else(|| panic!("Getting vertices at index: {}", width_pixel))
            .clear();

        self.vertex_data
            .sky
            .get_mut(width_pixel as usize)
            .unwrap_or_else(|| panic!("Getting vertices at index: {}", width_pixel))
            .clear();

        let mut pos = Vector2f {
            x: width_pixel as f32,
            y: 0.,
        };

        let floor = self.calculate_floor(ray_dir, side);

        for y in (self.draw_state.draw_end + 1)..(self.window_size.y as i32) {
            let current_dist = self.window_size.y / (2. * y as f32 - self.window_size.y as f32);
            let weight = current_dist / self.draw_state.perp_wall_dist;
            let current_floor = weight * floor + (1.0 - weight) * self.player_data.position;

            let tex_coord = Vector2f::new(
                ((current_floor.x * 128.) as i32 % 128) as f32,
                ((current_floor.y * 128.) as i32 % 128) as f32,
            );

            pos.y = y as f32;

            let vertex = Vertex::new(pos, Color::WHITE, tex_coord);

            self.vertex_data
                .ground
                .get_mut(width_pixel as usize)
                .unwrap_or_else(|| panic!("Getting vertices at index: {}", width_pixel))
                .append(&vertex);

            pos.y = self.window_size.y - y as f32;

            let vertex = Vertex::new(pos, Color::WHITE, tex_coord);

            self.vertex_data
                .sky
                .get_mut(width_pixel as usize)
                .unwrap_or_else(|| panic!("Getting vertices at index: {}", width_pixel))
                .append(&vertex);
        }
    }

    fn calculate_floor(&mut self, ray_dir: Vector2f, side: i32) -> Vector2f {
        let map_pos = Vector2f::new(
            self.draw_state.map_pos.x as f32,
            self.draw_state.map_pos.y as f32,
        );

        match side {
            0 if ray_dir.x >= 0.0 => (map_pos.x, map_pos.y + self.draw_state.wall_x),
            0 => (map_pos.x + 1., map_pos.y + self.draw_state.wall_x),
            1 if ray_dir.y >= 0.0 => (map_pos.x + self.draw_state.wall_x, map_pos.y),
            _ => (map_pos.x + self.draw_state.wall_x, map_pos.y + 1.),
        }
        .into()
    }

    fn calculate_wall_height(&mut self, ray_pos: Vector2f, ray_dir: Vector2f, side: i32) {
        self.draw_state.perp_wall_dist = if side == 0 {
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

    fn calculate_wall_texture(
        &mut self,
        ray_pos: Vector2f,
        ray_dir: Vector2f,
        side: i32,
        width_pixel: i32,
    ) {
        self.draw_state.wall_x = if side == 1 {
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

        let mut texture_x = (self.draw_state.wall_x * 128.) as i32;
        if side == 0 && ray_dir.x > 0. {
            texture_x = 128 - texture_x - 1;
        }
        if side == 1 && ray_dir.y < 0. {
            texture_x = 128 - texture_x - 1;
        }

        let mut texture_id = self
            .map
            .get_block(self.draw_state.map_pos)
            .unwrap_or_else(|| {
                panic!(
                    "Getting block at map position {:?}",
                    self.draw_state.map_pos
                )
            });

        if side == 1 {
            texture_id += 5;
        }

        self.textures_id.push(texture_id);

        let current_vertex_array = self
            .vertex_data
            .vertices
            .get_mut(width_pixel as usize)
            .unwrap_or_else(|| panic!("Getting vertices at index: {}", width_pixel));

        current_vertex_array.clear();
        current_vertex_array.append(&Vertex::new(
            Vector2f::new(width_pixel as f32, self.draw_state.draw_start as f32),
            Color::WHITE,
            Vector2f::new(texture_x as f32, 0.),
        ));
        current_vertex_array.append(&Vertex::new(
            Vector2f::new(width_pixel as f32, self.draw_state.draw_end as f32),
            Color::WHITE,
            Vector2f::new(texture_x as f32, 128.),
        ));
    }

    fn calculate_step(&mut self, ray_pos: Vector2f, ray_dir: Vector2f, delta_dist: Vector2f) {
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

    fn hit_wall(&mut self, side: &mut i32, delta_dist: Vector2f) {
        while matches!(self.map.get_block(self.draw_state.map_pos), Some(0)) {
            if self.draw_state.side_dist.x < self.draw_state.side_dist.y {
                self.draw_state.side_dist.x += delta_dist.x;
                self.draw_state.map_pos.x += self.draw_state.step.x;
                *side = 0;
            } else {
                self.draw_state.side_dist.y += delta_dist.y;
                self.draw_state.map_pos.y += self.draw_state.step.y;
                *side = 1;
            }
        }
    }

    fn update_direction(&mut self, event_handler: &EventHandler) {
        let mouse_move = if let Some((x, _)) = event_handler.has_mouse_moved_event() {
            x as f32 - (self.window_size.x / 2.)
        } else {
            0.
        } / -250.;

        let mouse_cos = mouse_move.cos();
        let mouse_sin = mouse_move.sin();
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

    fn update_by_key(&mut self, _: &EventHandler, key: Key) {
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

    fn update_position(&mut self, event_handler: &EventHandler) {
        self.update_by_key(event_handler, Key::W);
        self.update_by_key(event_handler, Key::S);
    }

    fn create_line_array(window_size: Vector2f) -> Box<[VertexArray]> {
        (0..window_size.x as usize)
            .map(|_| VertexArray::new(PrimitiveType::LINES, 0))
            .collect()
    }

    pub const fn get_player_pos(&self) -> Vector2f {
        self.player_data.position
    }
}

impl TextureRender for REngine<'_> {
    fn draw(&self, render_window: &mut RenderWindow, texture_loader: &TextureLoader) {
        let mut render_states = RenderStates::default();

        for (idx, line) in self.vertex_data.vertices.iter().enumerate() {
            render_states.set_texture(Some(texture_loader.get_texture(self.textures_id[idx])));
            render_window.draw_with_renderstates(line, &render_states);
        }

        render_states.set_texture(Some(texture_loader.get_texture(0)));
        for gr in self.vertex_data.ground.iter() {
            render_window.draw_with_renderstates(gr, &render_states);
        }

        render_states.set_texture(Some(texture_loader.get_texture(11)));
        for sky in self.vertex_data.sky.iter() {
            render_window.draw_with_renderstates(sky, &render_states);
        }
    }
}

impl EventUpdate for REngine<'_> {
    fn update(&mut self, event_handler: &EventHandler) {
        self.textures_id.clear();
        for width_pixel in 0..self.window_size.x as i32 {
            // initialize
            let camera_x = 2. * width_pixel as f32 / self.window_size.x - 1.;

            let ray_pos = self.player_data.position;
            self.draw_state.map_pos = Vector2i::new(ray_pos.x as i32, ray_pos.y as i32);

            let ray_dir = self.player_data.direction + (self.player_data.camera_plane * camera_x);
            let delta_dist = Vector2f::new(
                (1. + (ray_dir.y * ray_dir.y) / (ray_dir.x * ray_dir.x)).sqrt(),
                (1. + (ray_dir.x * ray_dir.x) / (ray_dir.y * ray_dir.y)).sqrt(),
            );

            // calculate
            self.calculate_step(ray_pos, ray_dir, delta_dist);

            let mut side = 0;
            self.hit_wall(&mut side, delta_dist);

            self.calculate_wall_height(ray_pos, ray_dir, side);

            self.calculate_wall_texture(ray_pos, ray_dir, side, width_pixel);

            if !self.no_ground {
                self.calculate_ground(ray_dir, side, width_pixel);
            }
        }
        self.update_direction(event_handler);
        self.update_position(event_handler);
    }
}

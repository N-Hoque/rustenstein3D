use rsfml::{
    graphics::{
        Color, PrimitiveType, RenderStates, RenderTarget, RenderWindow, Vertex, VertexArray,
    },
    system::{Vector2f, Vector2i},
    window::Key,
};

use crate::{event_handler::EventHandler, map::Map, texture_loader::TextureLoader};

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
    step: Vector2i,
    side_dist: Vector2f,
    draw_start: i32,
    draw_end: i32,
    perp_wall_dist: f32,
    wall_x: f32,
}

impl REngine {
    pub fn new(map: Map, window_size: &Vector2f, no_ground: bool) -> REngine {
        REngine {
            draw_state: DrawState::default(),
            map,
            no_ground,
            player_position: Vector2f { x: 22., y: 12. },
            vector_direction: Vector2f { x: -1., y: 0. },
            cam_plane: Vector2f { x: 0., y: 0.66 },
            window_size: Vector2f {
                x: window_size.x,
                y: window_size.y - 80.,
            },
            vertex_array: REngine::create_line_array(window_size),
            textures_id: Vec::new(),
            ground: REngine::create_line_array(window_size),
            sky: REngine::create_line_array(window_size),
        }
    }

    pub fn update(&mut self, event_handler: &EventHandler) {
        self.textures_id.clear();
        let ray_pos = Vector2f {
            x: self.player_position.x,
            y: self.player_position.y,
        };
        let mut ray_dir = Vector2f { x: 0., y: 0. };
        let mut map_pos = Vector2i { x: 0, y: 0 };
        let mut delta_dist = Vector2f { x: 0., y: 0. };
        let mut camera_x: f32;
        let mut side: i32;
        let mut x: i32 = 0;
        while x < self.window_size.x as i32 {
            // initialize
            camera_x = 2. * x as f32 / self.window_size.x - 1.;
            ray_dir.x = self.vector_direction.x + self.cam_plane.x * camera_x;
            ray_dir.y = self.vector_direction.y + self.cam_plane.y * camera_x;
            map_pos.x = ray_pos.x as i32;
            map_pos.y = ray_pos.y as i32;
            delta_dist.x = (1. + (ray_dir.y * ray_dir.y) / (ray_dir.x * ray_dir.x)).sqrt();
            delta_dist.y = (1. + (ray_dir.x * ray_dir.x) / (ray_dir.y * ray_dir.y)).sqrt();
            side = 0;

            // calculate
            self.calculate_step(&ray_dir, &ray_pos, &map_pos, &delta_dist);

            self.hit_wall(&mut map_pos, &mut delta_dist, &mut side);

            self.calculate_wall_height(side, &map_pos, &ray_pos, &ray_dir);

            self.calculate_wall_texture(side, &ray_dir, x, &map_pos, &ray_pos);

            if !self.no_ground {
                self.calculate_ground(side, &map_pos, &ray_dir, x);
            }

            x += 1;
        }
        self.update_events(event_handler);
    }

    fn calculate_ground(&mut self, side: i32, map_pos: &Vector2i, ray_dir: &Vector2f, x: i32) {
        let mut floor = Vector2f { x: 0., y: 0. };
        let dist_player: f32 = 0.;
        let mut current_dist: f32;
        let mut weight: f32;
        let mut current_floor = Vector2f { x: 0., y: 0. };
        let mut tex_coord = Vector2f { x: 0., y: 0. };
        let mut pos = Vector2f { x: x as f32, y: 0. };
        if side == 0 && ray_dir.x > 0. {
            floor.x = map_pos.x as f32;
            floor.y = map_pos.y as f32 + self.draw_state.wall_x;
        } else if side == 0 && ray_dir.x < 0. {
            floor.x = map_pos.x as f32 + 1.;
            floor.y = map_pos.y as f32 + self.draw_state.wall_x;
        } else if side == 1 && ray_dir.y > 0. {
            floor.x = map_pos.x as f32 + self.draw_state.wall_x;
            floor.y = map_pos.y as f32;
        } else {
            floor.x = map_pos.x as f32 + self.draw_state.wall_x;
            floor.y = map_pos.y as f32 + 1.;
        }

        if self.draw_state.draw_end < 0 {
            self.draw_state.draw_end = self.window_size.y as i32;
        }
        let mut y: i32 = self.draw_state.draw_end + 1;
        self.ground.get_mut(x as usize).unwrap().clear();
        self.sky.get_mut(x as usize).unwrap().clear();
        let mut vertex = Vertex::default();
        while y < self.window_size.y as i32 {
            current_dist = self.window_size.y / (2. * y as f32 - self.window_size.y as f32);
            weight = (current_dist - dist_player) / (self.draw_state.perp_wall_dist - dist_player);
            current_floor.x = weight * floor.x + (1. - weight) * self.player_position.x;
            current_floor.y = weight * floor.y + (1. - weight) * self.player_position.y;

            tex_coord.x = ((current_floor.x * 128.) as i32 % 128) as f32;
            tex_coord.y = ((current_floor.y * 128.) as i32 % 128) as f32;

            pos.y = y as f32;
            vertex.position.x = pos.x;
            vertex.position.y = pos.y;
            vertex.tex_coords.x = tex_coord.x;
            vertex.tex_coords.y = tex_coord.y;
            self.ground.get_mut(x as usize).unwrap().append(&vertex);
            pos.y = self.window_size.y - y as f32;
            vertex.position.x = pos.x;
            vertex.position.y = pos.y;
            vertex.tex_coords.x = tex_coord.x;
            vertex.tex_coords.y = tex_coord.y;
            self.sky.get_mut(x as usize).unwrap().append(&vertex);

            y += 1;
        }
    }

    fn calculate_wall_height(
        &mut self,
        side: i32,
        map_pos: &Vector2i,
        ray_pos: &Vector2f,
        ray_dir: &Vector2f,
    ) {
        self.draw_state.perp_wall_dist = if side == 0 {
            (map_pos.x as f32 - ray_pos.x + (1 - self.draw_state.step.x) as f32 / 2.) / ray_dir.x
        } else {
            (map_pos.y as f32 - ray_pos.y + (1 - self.draw_state.step.y) as f32 / 2.) / ray_dir.y
        }
        .abs();
        let line_height: i32 = if self.draw_state.perp_wall_dist as i32 == 0 {
            self.window_size.y as i32
        } else {
            ((self.window_size.y / self.draw_state.perp_wall_dist) as i32).abs()
        };
        self.draw_state.draw_start = (self.window_size.y as i32 / 2) - (line_height / 2);
        if self.draw_state.draw_start < 0 {
            self.draw_state.draw_start = 0;
        }
        self.draw_state.draw_end = line_height / 2 + self.window_size.y as i32 / 2;
        if self.draw_state.draw_end > self.window_size.y as i32 {
            self.draw_state.draw_end = self.window_size.y as i32 - 1;
        }
    }

    fn calculate_wall_texture(
        &mut self,
        side: i32,
        ray_dir: &Vector2f,
        x: i32,
        map_pos: &Vector2i,
        ray_pos: &Vector2f,
    ) {
        let mut texture_id = self
            .map
            .get_block(map_pos)
            .unwrap_or_else(|| panic!("Failed to get block at map position {:?}", map_pos));

        self.draw_state.wall_x = if side == 1 {
            ray_pos.x
                + ((map_pos.y as f32 - ray_pos.y + (1. - self.draw_state.step.y as f32) / 2.)
                    / ray_dir.y)
                    * ray_dir.x
        } else {
            ray_pos.y
                + ((map_pos.x as f32 - ray_pos.x + (1. - self.draw_state.step.x as f32) / 2.)
                    / ray_dir.x)
                    * ray_dir.y
        };
        self.draw_state.wall_x -= self.draw_state.wall_x.floor();

        let mut texture_x = (self.draw_state.wall_x * 128.) as i32;
        if side == 0 && ray_dir.x > 0. {
            texture_x = 128 - texture_x - 1;
        }
        if side == 1 && ray_dir.y < 0. {
            texture_x = 128 - texture_x - 1;
        }

        if side == 1 {
            texture_id += 5;
        }

        self.textures_id.push(texture_id);
        self.vertex_array.get_mut(x as usize).unwrap().clear();
        self.vertex_array
            .get_mut(x as usize)
            .unwrap()
            .append(&Vertex::new(
                Vector2f::new(x as f32, self.draw_state.draw_end as f32),
                Color::WHITE,
                Vector2f::new(texture_x as f32, 128.),
            ));
        self.vertex_array
            .get_mut(x as usize)
            .unwrap()
            .append(&Vertex::new(
                Vector2f::new(x as f32, self.draw_state.draw_start as f32),
                Color::WHITE,
                Vector2f::new(texture_x as f32, 0.),
            ));
    }

    fn calculate_step(
        &mut self,
        ray_dir: &Vector2f,
        ray_pos: &Vector2f,
        map_pos: &Vector2i,
        delta_dist: &Vector2f,
    ) {
        if ray_dir.x < 0. {
            self.draw_state.step.x = -1;
            self.draw_state.side_dist.x = (ray_pos.x - map_pos.x as f32) * delta_dist.x;
        } else {
            self.draw_state.step.x = 1;
            self.draw_state.side_dist.x = (map_pos.x as f32 + 1. - ray_pos.x) * delta_dist.x;
        }
        if ray_dir.y < 0. {
            self.draw_state.step.y = -1;
            self.draw_state.side_dist.y = (ray_pos.y - map_pos.y as f32) * delta_dist.y;
        } else {
            self.draw_state.step.y = 1;
            self.draw_state.side_dist.y = (map_pos.y as f32 + 1. - ray_pos.y) * delta_dist.y;
        }
    }

    fn hit_wall(&mut self, map_pos: &mut Vector2i, delta_dist: &mut Vector2f, side: &mut i32) {
        let mut hit: bool = false;
        while !hit {
            if self.draw_state.side_dist.x < self.draw_state.side_dist.y {
                self.draw_state.side_dist.x += delta_dist.x;
                map_pos.x += self.draw_state.step.x;
                *side = 0;
            } else {
                self.draw_state.side_dist.y += delta_dist.y;
                map_pos.y += self.draw_state.step.y;
                *side = 1;
            }
            hit = if let Some(0) = self.map.get_block(map_pos) {
                false
            } else {
                true
            };
        }
    }

    fn update_events(&mut self, event_handler: &EventHandler) {
        self.update_position(event_handler);
        self.update_direction(event_handler);
    }

    fn update_direction(&mut self, event_handler: &EventHandler) {
        let mouse_move = if let Some((x, _)) = event_handler.has_mouse_moved_event() {
            x as f32 - (self.window_size.x / 2.)
        } else {
            0.
        } / -250.;

        let old_dir_x = self.vector_direction.x;
        self.vector_direction.x = self.vector_direction.x * (mouse_move).cos()
            - self.vector_direction.y * (mouse_move).sin();
        self.vector_direction.y =
            old_dir_x * (mouse_move).sin() + self.vector_direction.y * (mouse_move).cos();
        let old_cam_plane_x = self.cam_plane.x;
        self.cam_plane.x =
            self.cam_plane.x * (mouse_move).cos() - self.cam_plane.y * (mouse_move).sin();
        self.cam_plane.y =
            old_cam_plane_x * (mouse_move).sin() + self.cam_plane.y * (mouse_move).cos();
    }

    fn update_by_key(&mut self, pos: &mut Vector2i, event_handler: &EventHandler, key: Key) {
        let multiplier = match key {
            Key::W => 1.0,
            Key::S => -1.0,
            _ => return,
        };

        if event_handler.is_key_pressed(key) {
            pos.x = (self.player_position.x + (self.vector_direction.x * 0.1 * multiplier)) as i32;
            pos.y = self.player_position.y as i32;
            if let Some(0) = self.map.get_block(&pos) {
                self.player_position.x += 0.1 * multiplier * self.vector_direction.x;
            }

            pos.y = (self.player_position.y + (self.vector_direction.y * 0.1 * multiplier)) as i32;
            pos.x = self.player_position.x as i32;
            if let Some(0) = self.map.get_block(&pos) {
                self.player_position.y += 0.1 * multiplier * self.vector_direction.y;
            }
        }
    }

    fn update_position(&mut self, event_handler: &EventHandler) {
        let mut pos = Vector2i::default();
        self.update_by_key(&mut pos, event_handler, Key::W);
        self.update_by_key(&mut pos, event_handler, Key::S);
    }

    fn create_line_array(window_size: &Vector2f) -> Vec<VertexArray> {
        (0..window_size.x as i32)
            .into_iter()
            .map(|_x| VertexArray::new(PrimitiveType::LINES, 2))
            .collect()
    }

    pub fn get_player_pos(&self) -> Vector2f {
        self.player_position
    }

    pub fn draw(&self, render_window: &mut RenderWindow, texture_loader: &TextureLoader) {
        let mut render_states = RenderStates::default();

        for (idx, line) in self.vertex_array.iter().enumerate() {
            render_states.set_texture(Some(
                texture_loader.get_texture(self.textures_id[idx as usize]),
            ));
            render_window.draw_with_renderstates(line, &render_states);
        }

        render_states.set_texture(Some(texture_loader.get_texture(0)));
        for gr in self.ground.iter() {
            render_window.draw_with_renderstates(gr, &render_states);
        }

        render_states.set_texture(Some(texture_loader.get_texture(11)));
        for sky in self.sky.iter() {
            render_window.draw_with_renderstates(sky, &render_states);
        }
    }
}

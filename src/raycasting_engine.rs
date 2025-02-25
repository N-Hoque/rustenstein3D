//! Module for performing the raycasting calculations and rendering

use rsfml::{
    graphics::{
        Color, PrimitiveType, RenderStates, RenderTarget, RenderWindow, Vertex, VertexArray,
    },
    system::{Vector2f, Vector2i},
    window::Key,
};

use crate::{event_handler::EventHandler, map::Map, texture_loader::TextureLoader};

pub struct RaycastEngine {
    player_position: Vector2f,
    vector_direction: Vector2f,
    cam_plane: Vector2f,
    map: Map,
    window_size: Vector2f,
    vertex_array: Vec<Box<VertexArray>>,
    textures_id: Vec<i32>,
    ground: Vec<Box<VertexArray>>,
    sky: Vec<Box<VertexArray>>,
    no_ground: bool,
}

impl RaycastEngine {
    pub fn new(map: Map, window_size: &Vector2f, no_ground: bool) -> RaycastEngine {
        RaycastEngine {
            player_position: Vector2f { x: 22., y: 12. },
            vector_direction: Vector2f { x: -1., y: 0. },
            cam_plane: Vector2f { x: 0., y: 0.66 },
            map,
            window_size: Vector2f {
                x: window_size.x,
                y: window_size.y - 80.,
            },
            vertex_array: RaycastEngine::create_line_array(window_size),
            textures_id: Vec::new(),
            ground: RaycastEngine::create_ground_array(window_size),
            sky: RaycastEngine::create_ground_array(window_size),
            no_ground,
        }
    }

    pub fn update(&mut self, event_handler: &EventHandler) {
        self.textures_id.clear();
        let ray_pos = self.player_position.clone();
        let mut ray_dir = Vector2f { x: 0., y: 0. };
        let mut map_pos = Vector2i { x: 0, y: 0 };
        let mut side_dist = Vector2f { x: 0., y: 0. };
        let mut delta_dist = Vector2f { x: 0., y: 0. };
        let mut step = Vector2i { x: 0, y: 0 };
        let mut draw_start: i32 = 0;
        let mut draw_end: i32 = 0;
        let mut camera_x: f32;
        let mut side: i32;
        let mut perpendicular_wall_dist: f32 = 0.;
        let mut wall_x: f32 = 0.;
        for x in 0..(self.window_size.x as i32) {
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
            self.calculate_step(
                &ray_dir,
                &mut step,
                &ray_pos,
                &map_pos,
                &delta_dist,
                &mut side_dist,
            );

            self.hit_wall(
                &mut map_pos,
                &mut side_dist,
                &mut step,
                &mut delta_dist,
                &mut side,
            );

            self.calculate_wall_height(
                side,
                &mut draw_start,
                &mut draw_end,
                &map_pos,
                &ray_pos,
                &ray_dir,
                &step,
                &mut perpendicular_wall_dist,
            );

            self.calculate_wall_texture(
                side,
                &ray_dir,
                x,
                &map_pos,
                &step,
                &ray_pos,
                draw_end,
                draw_start,
                &mut wall_x,
            );

            if !self.no_ground {
                self.calculate_ground(
                    side,
                    &map_pos,
                    wall_x,
                    &ray_dir,
                    perpendicular_wall_dist,
                    &mut draw_end,
                    x,
                );
            }
        }
        self.update_events(event_handler);
    }

    fn calculate_ground(
        &mut self,
        side: i32,
        map_pos: &Vector2i,
        wall_x: f32,
        ray_dir: &Vector2f,
        perpendicular_wall_dist: f32,
        draw_end: &mut i32,
        x: i32,
    ) {
        if *draw_end < 0 {
            *draw_end = self.window_size.y as i32;
        }

        self.ground.get_mut(x as usize).unwrap().clear();
        self.sky.get_mut(x as usize).unwrap().clear();

        let mut vertex = Vertex::default();
        let mut current_dist: f32;
        let mut weight: f32;
        let mut current_floor = Vector2f { x: 0., y: 0. };
        let mut tex_coord = Vector2f { x: 0., y: 0. };
        let mut pos = Vector2f { x: x as f32, y: 0. };
        let dist_player: f32 = 0.;

        let (map_pos_x, map_pos_y) = (map_pos.x as f32, map_pos.y as f32);
        let floor = if side == 0 && ray_dir.x > 0. {
            Vector2f {
                x: map_pos_x,
                y: map_pos_y + wall_x,
            }
        } else if side == 0 && ray_dir.x < 0. {
            Vector2f {
                x: map_pos_x + 1.,
                y: map_pos_y + wall_x,
            }
        } else if side == 1 && ray_dir.y > 0. {
            Vector2f {
                x: map_pos_x + wall_x,
                y: map_pos_y,
            }
        } else {
            Vector2f {
                x: map_pos_x + wall_x,
                y: map_pos_y + 1.,
            }
        };

        for y in (*draw_end + 1)..(self.window_size.y as i32) {
            current_dist = self.window_size.y / (2. * y as f32 - self.window_size.y as f32);
            weight = (current_dist - dist_player) / (perpendicular_wall_dist - dist_player);
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
        }
    }

    fn calculate_wall_height(
        &mut self,
        side: i32,
        draw_start: &mut i32,
        draw_end: &mut i32,
        map_pos: &Vector2i,
        ray_pos: &Vector2f,
        ray_dir: &Vector2f,
        step: &Vector2i,
        perpendicular_wall_dist: &mut f32,
    ) {
        *perpendicular_wall_dist = if side == 0 {
            (map_pos.x as f32 - ray_pos.x + (1 - step.x) as f32 / 2.) / ray_dir.x
        } else {
            (map_pos.y as f32 - ray_pos.y + (1 - step.y) as f32 / 2.) / ray_dir.y
        }
        .abs();

        let line_height: i32 = if *perpendicular_wall_dist as i32 == 0 {
            self.window_size.y as i32
        } else {
            ((self.window_size.y / *perpendicular_wall_dist) as i32).abs()
        };
        *draw_start = (self.window_size.y as i32 / 2) - (line_height / 2);
        if *draw_start < 0 {
            *draw_start = 0;
        }
        *draw_end = line_height / 2 + self.window_size.y as i32 / 2;
        if *draw_end > self.window_size.y as i32 {
            *draw_end = self.window_size.y as i32 - 1;
        }
    }

    fn calculate_wall_texture(
        &mut self,
        side: i32,
        ray_dir: &Vector2f,
        x: i32,
        map_pos: &Vector2i,
        step: &Vector2i,
        ray_pos: &Vector2f,
        draw_end: i32,
        draw_start: i32,
        wall_x: &mut f32,
    ) {
        let mut texture_id = self
            .map
            .get_block(map_pos)
            .expect(&format!("ERROR: Cannot get block ID {:?}", map_pos));

        *wall_x = if side == 1 {
            ray_pos.x
                + ((map_pos.y as f32 - ray_pos.y + (1. - step.y as f32) / 2.) / ray_dir.y)
                    * ray_dir.x
        } else {
            ray_pos.y
                + ((map_pos.x as f32 - ray_pos.x + (1. - step.x as f32) / 2.) / ray_dir.x)
                    * ray_dir.y
        };
        *wall_x -= wall_x.floor();

        let mut texture_x = (*wall_x * 128.) as i32;
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
                Vector2f::new(x as f32, draw_end as f32),
                Color::WHITE,
                Vector2f::new(texture_x as f32, 128.),
            ));
        self.vertex_array
            .get_mut(x as usize)
            .unwrap()
            .append(&Vertex::new(
                Vector2f::new(x as f32, draw_start as f32),
                Color::WHITE,
                Vector2f::new(texture_x as f32, 0.),
            ));
    }

    fn calculate_step(
        &self,
        ray_dir: &Vector2f,
        step: &mut Vector2i,
        ray_pos: &Vector2f,
        map_pos: &Vector2i,
        delta_dist: &Vector2f,
        side_dist: &mut Vector2f,
    ) {
        if ray_dir.x < 0. {
            step.x = -1;
            side_dist.x = (ray_pos.x - map_pos.x as f32) * delta_dist.x;
        } else {
            step.x = 1;
            side_dist.x = (map_pos.x as f32 + 1. - ray_pos.x) * delta_dist.x;
        }
        if ray_dir.y < 0. {
            step.y = -1;
            side_dist.y = (ray_pos.y - map_pos.y as f32) * delta_dist.y;
        } else {
            step.y = 1;
            side_dist.y = (map_pos.y as f32 + 1. - ray_pos.y) * delta_dist.y;
        }
    }

    fn hit_wall(
        &self,
        map_pos: &mut Vector2i,
        side_dist: &mut Vector2f,
        step: &mut Vector2i,
        delta_dist: &mut Vector2f,
        side: &mut i32,
    ) {
        let mut hit: bool = false;
        while !hit {
            if side_dist.x < side_dist.y {
                side_dist.x += delta_dist.x;
                map_pos.x += step.x;
                *side = 0;
            } else {
                side_dist.y += delta_dist.y;
                map_pos.y += step.y;
                *side = 1;
            }
            hit = match self.map.get_block(map_pos) {
                Some(block) if block == 0 => false,
                _ => true,
            };
        }
    }

    fn update_events(&mut self, event_handler: &EventHandler) {
        let mut pos = Vector2i { x: 0, y: 0 };
        if event_handler.is_key_pressed(Key::W) {
            pos.x = (self.player_position.x + (self.vector_direction.x * 0.1)) as i32;
            pos.y = self.player_position.y as i32;
            if self
                .map
                .get_block(&pos)
                .expect(&format!("ERROR: Cannot get block ID {:?}", &pos))
                == 0
            {
                self.player_position.x += self.vector_direction.x * 0.1;
            }
            pos.y = (self.player_position.y + (self.vector_direction.y * 0.1)) as i32;
            pos.x = self.player_position.x as i32;
            if self
                .map
                .get_block(&pos)
                .expect(&format!("ERROR: Cannot get block ID {:?}", &pos))
                == 0
            {
                self.player_position.y += self.vector_direction.y * 0.1;
            }
        }
        if event_handler.is_key_pressed(Key::S) {
            pos.x = (self.player_position.x - (self.vector_direction.x * 0.1)) as i32;
            pos.y = self.player_position.y as i32;
            if self
                .map
                .get_block(&pos)
                .expect(&format!("ERROR: Cannot get block ID {:?}", &pos))
                == 0
            {
                self.player_position.x -= self.vector_direction.x * 0.1;
            }
            pos.y = (self.player_position.y - (self.vector_direction.y * 0.1)) as i32;
            pos.x = self.player_position.x as i32;
            if self
                .map
                .get_block(&pos)
                .expect(&format!("ERROR: Cannot get block ID {:?}", &pos))
                == 0
            {
                self.player_position.y -= self.vector_direction.y * 0.1;
            }
        }

        let mouse_move = match event_handler.has_mouse_moved_event() {
            Some((x, _)) => x as f32 - (self.window_size.x / 2.) as f32,
            None => 0.,
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

    fn create_line_array(window_size: &Vector2f) -> Vec<Box<VertexArray>> {
        let mut lines: Vec<Box<VertexArray>> = Vec::new();
        for _ in 0..(window_size.x as i32) {
            let mut line: Box<VertexArray> = Box::new(VertexArray::default());
            line.set_primitive_type(PrimitiveType::Lines);
            lines.push(line);
        }
        lines
    }

    fn create_ground_array(window_size: &Vector2f) -> Vec<Box<VertexArray>> {
        let mut lines: Vec<Box<VertexArray>> = Vec::new();
        for _ in 0..(window_size.x as i32) {
            let line: Box<VertexArray> = Box::new(VertexArray::default());
            lines.push(line);
        }
        lines
    }

    pub fn get_player_pos(&self) -> Vector2f {
        self.player_position.clone()
    }

    pub fn draw<'r>(&self, render_window: &'r mut RenderWindow, texture_loader: &'r TextureLoader) {
        let mut render_states = RenderStates::default();
        for (line_idx, line) in self.vertex_array.iter().enumerate() {
            render_states.texture =
                Some(texture_loader.get_texture(self.textures_id[line_idx as usize]));
            render_window.draw_with_renderstates(&*(*line), render_states);
        }

        render_states.texture = Some(texture_loader.get_texture(0));
        for gr in self.ground.iter() {
            render_window.draw_with_renderstates(&*(*gr), render_states);
        }

        render_states.texture = Some(texture_loader.get_texture(11));
        for sky in self.sky.iter() {
            render_window.draw_with_renderstates(&*(*sky), render_states);
        }
    }
}

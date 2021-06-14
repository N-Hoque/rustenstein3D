//! Module for displaying the mini-map

use rsfml::{
    graphics::{
        Color, FloatRect, RectangleShape, RenderTarget, RenderWindow, Shape, Transformable, View,
    },
    system::{SfBox, Vector2f, Vector2i, Vector2u},
};

use crate::{map::*, texture_loader::TextureLoader};

pub struct MiniMap {
    map: Map,
    active: bool,
    mini_map_view: SfBox<View>,
    player_pos: Vector2f,
    rotation: f32,
}

impl MiniMap {
    pub fn new(map: Map, window_size: &Vector2u) -> MiniMap {
        let mut tmp_view = View::new(Vector2f::default(), Vector2f::default());
        let borrowed_view = &mut (*tmp_view);
        borrowed_view.set_size(Vector2f::new(window_size.x as f32, window_size.y as f32));
        borrowed_view.set_viewport(&FloatRect::new(0.70, 0.05, 0.25, 0.25));
        borrowed_view.set_rotation(-90.);
        MiniMap {
            map,
            active: true,
            mini_map_view: tmp_view,
            player_pos: Vector2f { x: 0., y: 0. },
            rotation: 0.,
        }
    }

    pub fn toggle_active(&mut self) -> bool {
        self.active = !self.active;
        self.active
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn update(&mut self, player_position: Vector2f, new_rotation: f32) -> () {
        self.player_pos = player_position;
        let borrowed_mini_map_view = &mut (*self.mini_map_view);
        borrowed_mini_map_view.rotate(new_rotation);
        borrowed_mini_map_view.set_center(Vector2f::new(
            self.player_pos.x * 80.,
            self.player_pos.y * 80.,
        ));
        self.rotation += new_rotation;
    }

    pub fn draw(&mut self, render_window: &mut RenderWindow, texture_loader: &TextureLoader) -> () {
        let mut block: i32;
        let map_size = self.map.get_map_size();
        let mut pos: Vector2i = Vector2i::new(0, 0);
        let mut rect = RectangleShape::with_size(Vector2f::new(80., 80.));
        rect.set_fill_color(Color::rgba(255, 255, 255, 175));
        render_window.set_view(&self.mini_map_view);
        while pos.x < map_size.x {
            while pos.y < map_size.y {
                block = self
                    .map
                    .get_block(&pos)
                    .expect("ERROR: Cannot get block in minimap.");
                if block == 0 {
                    rect.set_texture(texture_loader.get_texture(block), false);
                    rect.set_position(Vector2f::new(pos.x as f32 * 80., pos.y as f32 * 80.));
                } else {
                    rect.set_texture(texture_loader.get_texture(block), false);
                    rect.set_position(Vector2f::new(pos.x as f32 * 80., pos.y as f32 * 80.));
                }
                render_window.draw(&mut rect);
                pos.y += 1;
            }
            pos.x += 1;
            pos.y = 0;
        }
        rect.set_fill_color(Color::rgba(255, 0, 0, 125));
        rect.set_origin(Vector2f::new(40., 40.));
        rect.set_position(Vector2f::new(
            self.player_pos.x as f32 * 80.,
            self.player_pos.y as f32 * 80.,
        ));
        render_window.draw(&mut rect);

        //TODO: Figure out how to restore view.
        //let def_view = render_window.default_view();
        //render_window.set_view(&def_view);
    }
}

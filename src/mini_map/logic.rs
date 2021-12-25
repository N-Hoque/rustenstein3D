use std::borrow::BorrowMut;

use rsfml::{
    graphics::{
        Color, FloatRect, RectangleShape, RenderTarget, RenderWindow, Shape, Transformable, View,
    },
    system::{Vector2f, Vector2i, Vector2u},
};

use crate::{map::Map, texture_loader::TextureLoader};

use super::MiniMap;

impl MiniMap {
    pub fn new(map: Map, window_size: Vector2u) -> MiniMap {
        let mut mini_map_view = View::new(
            Vector2f::default(),
            Vector2f::new(window_size.x as f32, window_size.y as f32),
        );
        mini_map_view.set_viewport(&FloatRect::new(0.70, 0.05, 0.25, 0.25));
        mini_map_view.set_rotation(-90.0);

        MiniMap {
            map,
            active: true,
            mini_map_view,
            player_pos: Vector2f::default(),
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

    pub fn update(&mut self, player_position: Vector2f, new_rotation: f32) {
        self.player_pos = player_position;
        let map_view = (*self.mini_map_view).borrow_mut();
        map_view.rotate(new_rotation);
        map_view.set_center(self.player_pos * 80.);
        self.rotation += new_rotation;
    }

    pub fn draw(&self, render_window: &mut RenderWindow, texture_loader: &TextureLoader) {
        let def_view_centre = render_window.default_view().center();
        let def_view_size = render_window.default_view().size();
        let def_view = View::new(def_view_centre, def_view_size);
        let mut rect = RectangleShape::with_size(Vector2f::new(80., 80.));
        rect.set_fill_color(Color::rgba(255, 255, 255, 175));
        render_window.set_view(&self.mini_map_view);
        let map_size = self.map.get_map_size();
        let mut pos = Vector2i::default();
        while pos.x < map_size.x {
            while pos.y < map_size.y {
                let block = self
                    .map
                    .get_block(pos)
                    .unwrap_or_else(|| panic!("Getting block in minimap at position: {:?}", pos));
                rect.set_texture(texture_loader.get_texture(block), false);
                rect.set_position(Vector2f::new(pos.x as f32, pos.y as f32) * 80.);
                render_window.draw(&rect);
                pos.y += 1;
            }
            pos.x += 1;
            pos.y = 0;
        }
        rect.set_fill_color(Color::rgba(255, 0, 0, 125));
        rect.set_origin(Vector2f::new(40., 40.));
        rect.set_position(self.player_pos * 80.);
        render_window.draw(&rect);
        render_window.set_view(&def_view);
    }
}

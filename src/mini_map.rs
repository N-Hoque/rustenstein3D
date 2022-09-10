use std::borrow::BorrowMut;

use rsfml::{
    graphics::{
        Color, FloatRect, RectangleShape, RenderTarget, RenderWindow, Shape, Transformable, View,
    },
    system::{Vector2f, Vector2i, Vector2u},
    SfBox,
};

use crate::{core::TextureRender, map::Map, texture_loader::TextureLoader};

pub struct MiniMap<'m> {
    map: Map<'m>,
    active: bool,
    mini_map_view: SfBox<View>,
    player_pos: Vector2f,
    rotation: f32,
}

impl<'m> MiniMap<'m> {
    pub(crate) fn new(map: Map<'m>, window_size: Vector2u) -> Self {
        let mut mini_map_view = View::new(
            Vector2f::default(),
            Vector2f::new(window_size.x as f32, window_size.y as f32),
        );
        mini_map_view.set_viewport(&FloatRect::new(0.70, 0.05, 0.25, 0.25));
        mini_map_view.set_rotation(-90.0);

        Self {
            map,
            active: true,
            mini_map_view,
            player_pos: Vector2f::default(),
            rotation: 0.,
        }
    }

    pub(crate) fn toggle_active(&mut self) -> bool {
        self.active = !self.active;
        self.active
    }

    pub(crate) const fn is_active(&self) -> bool {
        self.active
    }

    pub(crate) fn update(&mut self, player_position: Vector2f, new_rotation: f32) {
        self.player_pos = player_position;
        let map_view = (*self.mini_map_view).borrow_mut();
        map_view.rotate(new_rotation);
        map_view.set_center(self.player_pos * 80.);
        self.rotation += new_rotation;
    }
}

impl MiniMap<'_> {
    fn create_block<'s>(
        &self,
        render_window: &mut RenderWindow,
        map_size: Vector2i,
        texture_loader: &'s TextureLoader,
    ) -> RectangleShape<'s> {
        let mut rect = create_default_block();
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
        rect
    }
}

impl TextureRender for MiniMap<'_> {
    fn draw(&self, render_window: &mut RenderWindow, texture_loader: &TextureLoader) {
        render_window.set_view(&self.mini_map_view);
        let drawn_block = self.create_block(render_window, self.map.get_map_size(), texture_loader);
        render_window.draw(&drawn_block);
        render_window.set_view(&get_default_view(render_window));
    }
}

fn create_default_block<'s>() -> RectangleShape<'s> {
    let mut rect = RectangleShape::with_size(Vector2f::new(80., 80.));
    rect.set_fill_color(Color::rgba(255, 255, 255, 175));
    rect
}

fn get_default_view(render_window: &RenderWindow) -> SfBox<View> {
    let def_view_centre = render_window.default_view().center();
    let def_view_size = render_window.default_view().size();
    View::new(def_view_centre, def_view_size)
}

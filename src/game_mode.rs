//! Module for configuration of selected game mode

use rsfml::{
    graphics::{Color, RectangleShape, RenderTarget, RenderWindow, Shape, Transformable},
    system::{Vector2f, Vector2i, Vector2u},
    window::Key,
};

use crate::{
    event_handler::*, hud::HUD, map::Map, mini_map::*, raycasting_engine::REngine,
    texture_loader::TextureLoader, weapon::Weapon,
};

pub struct GameMode<'s> {
    r_engine: REngine,
    texture_loader: &'s TextureLoader,
    window_size: Vector2u,
    map: Map,
    mini_map: MiniMap,
    player_position: Vector2f,
    hud: HUD<'s>,
    weapon: Weapon<'s>,
    sky: RectangleShape<'s>,
    ground: RectangleShape<'s>,
}

impl<'s> GameMode<'s> {
    pub fn new(
        window_size: Vector2u,
        texture_loader: &'s TextureLoader,
        no_ground: bool,
    ) -> GameMode<'s> {
        let map = GameMode::get_map();
        let mut sky = RectangleShape::with_size(Vector2f::new(
            window_size.x as f32,
            window_size.y as f32 / 2. - 40.,
        ));
        sky.set_fill_color(Color::rgb(63, 48, 21));
        let mut ground = RectangleShape::with_size(Vector2f::new(
            window_size.x as f32,
            window_size.y as f32 / 2. - 40.,
        ));
        ground.set_fill_color(Color::rgb(109, 108, 112));
        ground.set_position(Vector2f::new(0., window_size.y as f32 / 2. - 40.));
        GameMode {
            window_size,
            map: map.clone(),
            mini_map: MiniMap::new(map.clone(), &window_size),
            player_position: Vector2f { x: 4., y: 1. },
            r_engine: REngine::new(
                map,
                &Vector2f::new(window_size.x as f32, window_size.y as f32),
                no_ground,
            ),
            texture_loader,
            hud: HUD::new(
                &Vector2f::new(window_size.x as f32, window_size.y as f32),
                texture_loader,
            ),
            weapon: Weapon::new(
                &Vector2f::new(window_size.x as f32, window_size.y as f32),
                texture_loader,
            ),
            sky,
            ground,
        }
    }

    pub fn get_map() -> Map {
        let map_i32: Vec<i32> = vec![
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 0, 0, 0, 0, 3, 0, 3, 0, 3,
            0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1,
            0, 0, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 0, 3, 0, 0, 0, 3, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0,
            2, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 2, 2, 0, 2, 2,
            0, 0, 0, 0, 3, 0, 3, 0, 3, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 1, 1, 4, 4, 4, 4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 1, 1, 4, 0, 4, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 4, 0,
            0, 0, 0, 5, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 4, 0, 4, 0, 0, 0, 0,
            4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 4, 0, 4, 4, 4, 4, 4, 4, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 1, 1, 4, 4, 4, 4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        ];
        Map::new(map_i32, &Vector2f::new(24., 24.))
    }

    pub fn update(&mut self, event_handler: &EventHandler) {
        let mut rotation: f32 = 0.;
        if event_handler.is_key_pressed(Key::Left) {
            rotation = -5.25;
        }
        if event_handler.is_key_pressed(Key::Right) {
            rotation = 5.25;
        }
        if let Some(_) = event_handler.has_key_pressed_event(Key::M) {
            self.mini_map.toggle_active();
        }
        self.r_engine.update(event_handler);
        if self.mini_map.is_active() {
            self.mini_map
                .update(self.r_engine.get_player_pos(), rotation);
        }
        self.hud.update();
        self.weapon.update(event_handler);
    }

    pub fn draw(&mut self, render_window: &mut RenderWindow) {
        render_window.draw(&self.sky);
        render_window.draw(&self.ground);
        self.r_engine.draw(render_window, self.texture_loader);
        if self.mini_map.is_active() {
            self.mini_map.draw(render_window, self.texture_loader);
        }
        self.hud.draw(render_window);
        self.weapon.draw(render_window);
        render_window.set_mouse_cursor_visible(false);
        render_window.set_mouse_position(Vector2i::new(
            (self.window_size.x / 2) as i32,
            (self.window_size.y / 2) as i32,
        ));
    }
}

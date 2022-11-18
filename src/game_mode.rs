use rsfml::{
    graphics::{Color, RectangleShape, RenderTarget, RenderWindow, Shape, Transformable},
    system::{Vector2f, Vector2i, Vector2u},
    window::Key,
};

use crate::{
    core::{EventUpdate, RenderMut, TextureRender, Update},
    event_handler::EventHandler,
    hud::HUD,
    map::Map,
    mini_map::MiniMap,
    raycasting_engine::REngine,
    texture_loader::TextureLoader,
    weapon::Weapon,
};

pub struct GameMode<'s, 'a, 'm> {
    window_size: Vector2u,
    mini_map: MiniMap<'m>,
    r_engine: REngine<'m>,
    texture_loader: &'s TextureLoader,
    hud: HUD<'s, 'a>,
    weapon: Weapon<'s, 'a>,
    sky: RectangleShape<'s>,
    ground: RectangleShape<'s>,
}

static RAW_MAP: [i32; 576] = [
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 0, 0, 0, 0, 3, 0, 3, 0, 3, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 2, 0,
    0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 0, 3,
    0, 0, 0, 3, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 2, 2, 0, 2, 2, 0, 0, 0, 0, 3, 0, 3, 0, 3, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 4, 4, 4, 4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 4, 0, 4, 0, 0, 0, 0,
    4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 4, 0, 0, 0, 0, 5, 0, 4, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 1, 1, 4, 0, 4, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 4, 0, 4, 4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 4, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 4, 4, 4, 4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
];

impl<'s: 'a, 'a, 'm> GameMode<'s, 'a, 'm> {
    pub(crate) fn new(window_size: Vector2u, texture_loader: &'s TextureLoader) -> Self {
        let map = GameMode::get_map();
        let window_size_f32 = Vector2f::new(window_size.x as f32, window_size.y as f32);
        Self {
            window_size,
            texture_loader,
            sky: create_sky(window_size),
            ground: create_ground(window_size),
            mini_map: MiniMap::new(map.clone(), window_size),
            r_engine: REngine::new(map, window_size_f32),
            hud: HUD::new(window_size_f32, texture_loader),
            weapon: Weapon::new(window_size_f32, texture_loader),
        }
    }

    pub(crate) fn get_map() -> Map<'m> {
        Map::new(&RAW_MAP, Vector2f::new(24., 24.))
    }

    pub(crate) fn disable_ground(&mut self) {
        self.r_engine.disable_ground();
    }
}

fn create_ground(window_size: Vector2u) -> RectangleShape<'static> {
    let mut ground = RectangleShape::with_size(Vector2f {
        x: window_size.x as f32,
        y: window_size.y as f32 / 2. - 40.,
    });
    ground.set_fill_color(Color::rgb(109, 108, 112));
    ground.set_position(Vector2f::new(0., window_size.y as f32 / 2. - 40.));
    ground
}

fn create_sky(window_size: Vector2u) -> RectangleShape<'static> {
    let mut sky = RectangleShape::with_size(Vector2f {
        x: window_size.x as f32,
        y: window_size.y as f32 / 2. - 40.,
    });
    sky.set_fill_color(Color::rgb(63, 48, 21));
    sky
}

impl EventUpdate for GameMode<'_, '_, '_> {
    fn update(&mut self, event_handler: &EventHandler) {
        let rotation: f32 = if EventHandler::is_key_pressed(Key::Left) {
            -5.25
        } else if EventHandler::is_key_pressed(Key::Right) {
            5.25
        } else {
            0.0
        };
        if event_handler.has_key_pressed_event(Key::M).is_some() {
            self.mini_map.toggle_active();
        };
        self.r_engine.update(event_handler);
        if self.mini_map.is_active() {
            self.mini_map
                .update(self.r_engine.get_player_position(), rotation);
        }
        self.hud.update();
        self.weapon.update(event_handler);
    }
}

impl RenderMut for GameMode<'_, '_, '_> {
    fn draw(&mut self, render_window: &mut RenderWindow) {
        render_window.draw(&self.sky);
        render_window.draw(&self.ground);
        self.r_engine.draw(render_window, self.texture_loader);
        if self.mini_map.is_active() {
            self.mini_map.draw(render_window, self.texture_loader);
        }
        self.hud.draw(render_window);
        self.weapon.draw(render_window);
        render_window.set_mouse_cursor_visible(false);
        render_window.set_mouse_position(
            Vector2i::new(self.window_size.x as i32, self.window_size.y as i32) / 2,
        );
    }
}

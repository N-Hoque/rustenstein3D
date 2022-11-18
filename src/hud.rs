use rsfml::{
    graphics::{Color, RectangleShape, RenderTarget, RenderWindow, Shape, Transformable},
    system::{Clock, Vector2f},
    SfBox,
};

use crate::{
    animation::{Animation, PlayState},
    core::{RenderMut, Update},
    texture_loader::TextureLoader,
};

#[allow(clippy::upper_case_acronyms)]
pub struct HUD<'s, 'a> {
    window_size: Vector2f,
    background: RectangleShape<'s>,
    face: RectangleShape<'s>,
    face_animation: Animation<'a>,
    texture_loader: &'s TextureLoader,
    face_clock: SfBox<Clock>,
}

impl<'s> HUD<'s, '_> {
    pub fn new(window_size: Vector2f, texture_loader: &'s TextureLoader) -> HUD<'s, '_> {
        HUD {
            window_size,
            texture_loader,
            face: create_face(window_size),
            face_animation: Animation::new(&[40, 41, 42], 0, 1., PlayState::Play),
            face_clock: Clock::start(),
            background: RectangleShape::new(),
        }
    }
}

fn create_face(window_size: Vector2f) -> RectangleShape<'static> {
    let mut face = RectangleShape::with_size(Vector2f::new(43., 58.));
    face.set_position(Vector2f::new(window_size.x / 2. - 21., window_size.y - 71.));
    face
}

impl Update for HUD<'_, '_> {
    fn update(&mut self) {
        self.background
            .set_size(Vector2f::new(self.window_size.x - 21., 59.));
        self.background.set_fill_color(Color::rgb(6, 1, 162));
        self.background
            .set_position(Vector2f::new(10., self.window_size.y - 70.));
        self.face_animation.update();
        self.face.set_texture(
            self.texture_loader
                .get_texture(self.face_animation.get_current_texture_id()),
            false,
        );
        if self.face_clock.elapsed_time().as_seconds() >= 7. {
            self.face_animation.set_state(PlayState::Play);
            self.face_clock.restart();
        }
    }
}

impl RenderMut for HUD<'_, '_> {
    fn draw(&mut self, render_window: &mut RenderWindow) {
        render_window.draw(&self.background);
        render_window.draw(&self.face);
    }
}

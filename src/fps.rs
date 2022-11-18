use rsfml::{
    graphics::{Color, Font, RenderTarget, RenderWindow, Text, Transformable},
    system::{Clock, Vector2f},
    SfBox,
};

use crate::core::{Render, Update};

pub struct FPSHandler<'s> {
    fps_clock: SfBox<Clock>,
    text: Text<'s>,
}

impl<'s> FPSHandler<'s> {
    pub(crate) fn new(font: &'s Font) -> FPSHandler<'s> {
        FPSHandler {
            text: create_text(font),
            fps_clock: Clock::start(),
        }
    }
}

fn create_text(font: &Font) -> Text {
    let mut text = Text::new("0", font, 20);
    text.set_position(Vector2f::new(10., 10.));
    text.set_fill_color(Color::GREEN);
    text
}

impl Render for FPSHandler<'_> {
    fn draw(&self, render_window: &mut RenderWindow) {
        render_window.draw(&self.text);
    }
}

impl Update for FPSHandler<'_> {
    fn update(&mut self) {
        let delta = 1.0 / self.fps_clock.elapsed_time().as_seconds();

        self.text.set_string(&format!("{:.0}", delta));

        self.fps_clock.restart();
    }
}

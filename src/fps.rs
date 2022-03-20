use rsfml::{
    graphics::{Color, Font, RenderTarget, RenderWindow, Text, Transformable},
    system::{Clock, Vector2f},
};

pub struct FPSHandler<'s> {
    fps_clock: Clock,
    text: Text<'s>,
}

impl<'s> FPSHandler<'s> {
    pub fn new(font: &'s Font) -> FPSHandler<'s> {
        let mut text = Text::new("0", font, 20);
        text.set_position(Vector2f::new(10., 10.));
        text.set_fill_color(Color::GREEN);
        FPSHandler {
            text,
            fps_clock: Clock::start(),
        }
    }

    pub fn update(&mut self) {
        let delta = 1.0 / self.fps_clock.elapsed_time().as_seconds();

        self.text.set_string(&format!("{:.0}", delta));

        self.fps_clock.restart();
    }

    pub fn draw(&self, render_window: &mut RenderWindow) {
        render_window.draw(&self.text)
    }
}

use rsfml::{
    graphics::{Color, Font, RenderTarget, RenderWindow, Text, Transformable},
    system::{Clock, Vector2f},
};

pub struct FPSHandler<'s> {
    fps_clock: Clock,
    text: Text<'s>,
    images: usize,
}

impl<'s> FPSHandler<'s> {
    pub fn new(font: &'s Font) -> FPSHandler<'s> {
        let mut text = Text::new("0", font, 20);
        text.set_position(Vector2f::new(10., 10.));
        text.set_fill_color(Color::WHITE);
        FPSHandler {
            text,
            fps_clock: Clock::start(),
            images: 0,
        }
    }

    pub fn update(&mut self) {
        if self.fps_clock.elapsed_time().as_seconds() >= 0.33 {
            self.text.set_string(&(self.images * 3).to_string());
            self.images = 0;
            self.fps_clock.restart();
        }
        self.images += 1;
    }

    pub fn draw(&self, render_window: &mut RenderWindow) {
        render_window.draw(&self.text)
    }
}

//! Basic class to display the games frames per seconds (FPS).
//!
//! This class displays the current FPS in the bottom-left of the window.
#![allow(non_snake_case)]

use rsfml::{
    graphics::{Color, Font, RenderTarget, RenderWindow, Text, Transformable},
    system::{Clock, Vector2f},
};

pub struct FPSHandler<'t> {
    fps_clock: Clock,
    text: Text<'t>,
    images: usize,
}

impl<'t> FPSHandler<'t> {
    /// Constructs a new instance of FPSHandler
    ///
    /// # Arguments
    /// `font` - The font to render the text on the window.
    pub fn new(font: &'t Font) -> FPSHandler<'t> {
        let mut t = Text::new("0", font, 20);
        t.set_position(Vector2f::new(10., 10.));
        t.set_fill_color(Color::WHITE);
        FPSHandler {
            fps_clock: Clock::start(),
            text: t,
            images: 0,
        }
    }

    /// Update internal data of the FPSHandler
    ///
    /// Call this function at each end of the loop to update FPSHandler internal data.
    pub fn update(&mut self) {
        if self.fps_clock.elapsed_time().as_seconds() >= 0.33 {
            self.text.set_string((self.images * 3).to_string().as_str());
            self.images = 0;
            self.fps_clock.restart();
        }
        self.images += 1;
    }

    /// Draw the current FPS on the left bottom of the window
    ///
    /// # Arguments
    /// `&mut render_window` - The window to draw onto
    pub fn draw(&self, render_window: &mut RenderWindow) {
        render_window.draw(&self.text)
    }
}

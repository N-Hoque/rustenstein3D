#![allow(non_snake_case)]

use crate::event_handler::*;
use crate::fps::*;
use crate::game_mode::*;
use crate::texture_loader::TextureLoader;

use rsfml::{
    graphics::{Color, Font, RenderTarget, RenderWindow},
    window::Key,
};

const CLEAR_COLOR: Color = Color::rgb(3, 64, 59);

pub struct GameLoop<'s> {
    game_mode: GameMode<'s>,
    event_handler: EventHandler,
    render_window: RenderWindow,
    fps_handler: Option<FPSHandler<'s>>,
}

impl<'s> GameLoop<'s> {
    pub fn new(
        render_window: RenderWindow,
        texture_loader: &'s TextureLoader,
        no_ground: bool,
    ) -> GameLoop<'s> {
        let window_size = render_window.size();
        GameLoop {
            render_window,
            fps_handler: None,
            event_handler: EventHandler::new(),
            game_mode: GameMode::new(window_size, texture_loader, no_ground),
        }
    }

    pub fn activate_FPS(&mut self, font: &'s Font) {
        if self.fps_handler.is_none() {
            self.fps_handler = Some(FPSHandler::new(font))
        }
    }

    pub fn deactivate_FPS(&mut self) {
        if self.fps_handler.is_some() {
            self.fps_handler = None
        }
    }

    pub fn run(&mut self) {
        while self.render_window.is_open() {
            self.update();
            self.draw();
        }
    }

    pub fn update(&mut self) {
        self.event_handler.update_events(&mut self.render_window);
        if self.event_handler.has_closed_event() || self.event_handler.is_key_pressed(Key::ESCAPE) {
            self.render_window.close();
        }
        self.game_mode.update(&self.event_handler);
        self.fps_handler.as_mut().unwrap().update();
    }

    pub fn draw(&mut self) {
        self.render_window.clear(CLEAR_COLOR);
        self.game_mode.draw(&mut self.render_window);
        if self.fps_handler.is_some() {
            self.fps_handler
                .as_mut()
                .unwrap()
                .draw(&mut self.render_window)
        };
        self.render_window.display();
    }
}

#![allow(non_snake_case)]

use rsfml::{
    graphics::{Color, Font, RenderTarget, RenderWindow},
    window::Key,
};

use crate::event_handler::*;
use crate::fps::*;
use crate::game_mode::*;
use crate::texture_loader::TextureLoader;

pub struct GameLoop<'s> {
    render_window: RenderWindow,
    fps_handler: Option<FPSHandler<'s>>,
    event_handler: EventHandler,
    clear_color: Color,
    game_mode: GameMode<'s>,
    texture_loader: &'s TextureLoader,
}

impl<'s> GameLoop<'s> {
    pub fn new(
        render_window: RenderWindow,
        texture_loader: &'s TextureLoader,
        noground: bool,
    ) -> GameLoop<'s> {
        let tmp_size = render_window.size();
        GameLoop {
            render_window,
            fps_handler: None,
            event_handler: EventHandler::new(),
            clear_color: Color::rgb(3, 64, 59),
            game_mode: GameMode::new(tmp_size, texture_loader, noground),
            texture_loader,
        }
    }

    pub fn activate_FPS(&mut self, font: &'s Font) -> () {
        if let Some(_) = self.fps_handler {
            ()
        } else {
            self.fps_handler = Some(FPSHandler::new(font))
        }
    }

    pub fn deactivate_FPS(&mut self) -> () {
        if let Some(_) = self.fps_handler {
            self.fps_handler = None
        }
    }

    pub fn run(&mut self) -> () {
        while self.render_window.is_open() {
            self.update();
            self.draw();
        }
    }

    pub fn update(&mut self) -> () {
        self.event_handler.update_events(&mut self.render_window);
        if self.event_handler.has_closed_event() || self.event_handler.is_key_pressed(Key::ESCAPE) {
            self.render_window.close();
        }
        self.game_mode.update(&self.event_handler);
        self.fps_handler.as_mut().unwrap().update();
    }

    pub fn draw(&mut self) -> () {
        self.render_window.clear(self.clear_color);
        self.game_mode.draw(&mut self.render_window);
        if let Some(_) = self.fps_handler {
            self.fps_handler
                .as_mut()
                .unwrap()
                .draw(&mut self.render_window)
        };
        self.render_window.display();
    }
}

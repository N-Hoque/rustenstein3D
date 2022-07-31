use rsfml::{
    graphics::{Color, Font, RenderTarget, RenderWindow},
    window::Key,
};

use crate::{
    core::{DrawMut, EventUpdate, Render, RenderMut, RenderUpdate, Update},
    event_handler::EventHandler,
    fps::FPSHandler,
    game_mode::GameMode,
    texture_loader::TextureLoader,
};

const CLEAR_COLOR: Color = Color::rgb(3, 64, 59);

pub struct MainLoop<'s> {
    game_mode: GameMode<'s>,
    event_handler: EventHandler,
    render_window: RenderWindow,
    fps_handler: Option<FPSHandler<'s>>,
}

impl<'s> MainLoop<'s> {
    #[must_use]
    pub fn new(
        render_window: RenderWindow,
        texture_loader: &'s TextureLoader,
        no_ground: bool,
    ) -> MainLoop<'s> {
        let window_size = render_window.size();
        MainLoop {
            render_window,
            fps_handler: None,
            event_handler: EventHandler::default(),
            game_mode: GameMode::new(window_size, texture_loader, no_ground),
        }
    }

    pub fn enable_fps(&mut self, font: &'s Font) {
        if self.fps_handler.is_none() {
            self.fps_handler = Some(FPSHandler::new(font));
        }
    }

    pub fn disable_fps(&mut self) {
        if self.fps_handler.is_some() {
            self.fps_handler = None;
        }
    }

    pub fn run(&mut self) {
        while self.render_window.is_open() {
            self.update();
            self.draw();
        }
    }
}

impl Update for MainLoop<'_> {
    fn update(&mut self) {
        self.event_handler.update(&mut self.render_window);
        if self.event_handler.has_closed_event() || EventHandler::is_key_pressed(Key::ESCAPE) {
            self.render_window.close();
        }
        self.game_mode.update(&self.event_handler);
        self.fps_handler
            .as_mut()
            .expect("Updating FPS Handler")
            .update();
    }
}

impl DrawMut for MainLoop<'_> {
    fn draw(&mut self) {
        self.render_window.clear(CLEAR_COLOR);
        self.game_mode.draw(&mut self.render_window);

        if let Some(fps_handler) = self.fps_handler.as_mut() {
            fps_handler.draw(&mut self.render_window);
        }

        self.render_window.display();
    }
}

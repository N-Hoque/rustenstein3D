use rsfml::graphics::RenderWindow;

use crate::{event_handler::EventHandler, texture_loader::TextureLoader};

pub(crate) trait Draw {
    fn draw(&self);
}

pub(crate) trait Render {
    fn draw(&self, render_window: &mut RenderWindow);
}

pub(crate) trait TextureRender {
    fn draw(&self, render_window: &mut RenderWindow, texture_loader: &TextureLoader);
}

pub(crate) trait DrawMut {
    fn draw(&mut self);
}

pub(crate) trait RenderMut {
    fn draw(&mut self, render_window: &mut RenderWindow);
}

pub(crate) trait TextureRenderMut {
    fn draw(&mut self, render_window: &mut RenderWindow, texture_loader: &TextureLoader);
}

pub(crate) trait Update {
    fn update(&mut self);
}

pub(crate) trait EventUpdate {
    fn update(&mut self, event_handler: &EventHandler);
}

pub(crate) trait EventUpdateMut {
    fn update(&mut self, event_handler: &mut EventHandler);
}

pub(crate) trait RenderUpdate {
    fn update(&mut self, render_window: &mut RenderWindow);
}

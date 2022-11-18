use rsfml::graphics::RenderWindow;

use crate::{event_handler::EventHandler, texture_loader::TextureLoader};

pub trait Draw {
    fn draw(&self);
}

pub trait Render {
    fn draw(&self, render_window: &mut RenderWindow);
}

pub trait TextureRender {
    fn draw(&self, render_window: &mut RenderWindow, texture_loader: &TextureLoader);
}

pub trait DrawMut {
    fn draw(&mut self);
}

pub trait RenderMut {
    fn draw(&mut self, render_window: &mut RenderWindow);
}

pub trait TextureRenderMut {
    fn draw(&mut self, render_window: &mut RenderWindow, texture_loader: &TextureLoader);
}

pub trait Update {
    fn update(&mut self);
}

pub trait EventUpdate {
    fn update(&mut self, event_handler: &EventHandler);
}

pub trait EventUpdateMut {
    fn update(&mut self, event_handler: &mut EventHandler);
}

pub trait RenderUpdate {
    fn update(&mut self, render_window: &mut RenderWindow);
}

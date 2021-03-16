use rsfml::{
    graphics::{
        Color, PrimitiveType, RectangleShape, RenderTarget, RenderWindow, Shape, Transformable,
        Vertex, VertexArray,
    },
    system::{Clock, Vector2f},
};

use crate::animation::*;
use crate::texture_loader::TextureLoader;

pub struct HUD<'s> {
    window_size: Vector2f,
    background: RectangleShape<'s>,
    hud_vertex_array: VertexArray,
    face: RectangleShape<'s>,
    face_animation: Animation,
    texture_loader: &'s TextureLoader,
    face_clock: Clock,
}

impl<'s> HUD<'s> {
    pub fn new(window_size: &Vector2f, texture_loader: &'s TextureLoader) -> HUD<'s> {
        let mut array = VertexArray::default();
        array.set_primitive_type(PrimitiveType::LINE_STRIP);
        let mut tmp_face = RectangleShape::with_size(Vector2f::new(43., 58.));
        tmp_face.set_position(Vector2f::new(window_size.x / 2. - 21., window_size.y - 71.));
        HUD {
            window_size: window_size.clone(),
            background: RectangleShape::new(),
            hud_vertex_array: array,
            face: tmp_face,
            face_animation: Animation::new(
                vec![40, 41, 42],
                AnimationState::Play,
                AnimationPlayMode::Once,
                1.,
                0,
            ),
            texture_loader,
            face_clock: Clock::start(),
        }
    }

    pub fn update(&mut self) -> () {
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
            self.face_animation.set_state(AnimationState::Play);
            self.face_clock.restart();
        }
    }

    fn draw_line(
        &mut self,
        x1: f32,
        x2: f32,
        y1: f32,
        y2: f32,
        color: &Color,
        render_window: &mut RenderWindow,
    ) -> () {
        self.hud_vertex_array.clear();
        self.hud_vertex_array
            .append(&Vertex::with_pos_color(Vector2f::new(x1, y1), *color));
        self.hud_vertex_array
            .append(&Vertex::with_pos_color(Vector2f::new(x2, y2), *color));
        render_window.draw(&self.hud_vertex_array);
    }

    fn draw_2line(
        &mut self,
        x1: f32,
        x2: f32,
        x3: f32,
        y1: f32,
        y2: f32,
        y3: f32,
        color: &Color,
        render_window: &mut RenderWindow,
    ) -> () {
        self.hud_vertex_array.clear();
        self.hud_vertex_array
            .append(&Vertex::with_pos_color(Vector2f::new(x1, y1), *color));
        self.hud_vertex_array
            .append(&Vertex::with_pos_color(Vector2f::new(x2, y2), *color));
        self.hud_vertex_array
            .append(&Vertex::with_pos_color(Vector2f::new(x3, y3), *color));
        render_window.draw(&self.hud_vertex_array);
    }

    pub fn draw(&mut self, render_window: &mut RenderWindow) -> () {
        render_window.draw(&self.background);
        let window_x = self.window_size.x;
        let window_y = self.window_size.y;
        self.draw_2line(
            window_x - 9.,
            window_x - 9.,
            9.,
            window_y - 70.,
            window_y - 10.,
            window_y - 10.,
            &Color::rgba(255, 255, 255, 75),
            render_window,
        );
        self.draw_2line(
            window_x - 11.,
            window_x - 11.,
            11.,
            window_y - 70.,
            window_y - 12.,
            window_y - 12.,
            &Color::BLACK,
            render_window,
        );
        self.draw_2line(
            9.,
            9.,
            window_x - 9.,
            window_y - 12.,
            window_y - 71.,
            window_y - 71.,
            &Color::BLACK,
            render_window,
        );
        self.draw_2line(
            11.,
            11.,
            window_x - 11.,
            window_y - 11.,
            window_y - 69.,
            window_y - 69.,
            &Color::rgba(255, 255, 255, 75),
            render_window,
        );
        self.draw_line(
            window_x,
            0.,
            window_y - 80.,
            window_y - 80.,
            &Color::rgba(255, 255, 255, 50),
            render_window,
        );
        self.draw_line(
            window_x,
            0.,
            window_y - 79.,
            window_y - 79.,
            &Color::rgba(255, 255, 255, 75),
            render_window,
        );
        render_window.draw(&self.face);
    }
}

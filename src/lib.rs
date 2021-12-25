extern crate rsfml;

pub(crate) mod animation;
pub(crate) mod event_handler;
pub(crate) mod fps;
pub mod game;
pub(crate) mod game_mode;
pub(crate) mod hud;
pub(crate) mod map;
pub(crate) mod mini_map;
pub(crate) mod raycasting_engine;
pub(crate) mod texture_loader;
pub(crate) mod weapon;

use rsfml::{
    graphics::{Font, RenderWindow},
    system::Vector2i,
    window::{ContextSettings, Style, VideoMode},
};

use texture_loader::TextureLoader;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "Rustenstein 3D", about = "Options for Rustenstein3D")]
pub struct RustensteinOptions {
    #[structopt(short, long, help = "Disables floor and sky rendering")]
    pub no_ground: bool,

    #[structopt(
        short,
        long,
        help = "Sets the FPS (Frames per second)",
        default_value = "30"
    )]
    pub fps: u8,

    #[structopt(
        short,
        long,
        help = "Set the size of the window",
        number_of_values = 2,
        value_names = &["width", "height"],
    )]
    pub window_size: Vec<u16>,

    #[structopt(short, long, help = "Shows the cursor")]
    pub cursor: bool,
}

pub fn load_resources() -> TextureLoader {
    TextureLoader::with_textures(&[
        "resources/ground.tga",              // 0
        "resources/1.tga",                   // 1
        "resources/2.tga",                   // 2
        "resources/3.tga",                   // 3
        "resources/4.tga",                   // 4
        "resources/5.tga",                   // 5
        "resources/6.tga",                   // 6
        "resources/7.tga",                   // 7
        "resources/8.tga",                   // 8
        "resources/9.tga",                   // 9
        "resources/10.tga",                  // 10
        "resources/sky.tga",                 // 11
        "resources/weapons/gun_1.png",       // 12
        "resources/weapons/gun_2.png",       // 13
        "resources/weapons/gun_3.png",       // 14
        "resources/weapons/gun_4.png",       // 15
        "resources/weapons/gun_5.png",       // 16
        "resources/weapons/gun_6.png",       // 17
        "resources/weapons/gun_shadow.png",  // 18
        "resources/weapons/gun2_1.png",      // 19
        "resources/weapons/gun2_2.png",      // 20
        "resources/weapons/gun2_3.png",      // 21
        "resources/weapons/gun2_4.png",      // 22
        "resources/weapons/gun2_5.png",      // 23
        "resources/weapons/gun2_6.png",      // 24
        "resources/weapons/gun2_shadow.png", // 25
        "resources/weapons/gun3_1.png",      // 26
        "resources/weapons/gun3_2.png",      // 27
        "resources/weapons/gun3_3.png",      // 28
        "resources/weapons/gun3_4.png",      // 29
        "resources/weapons/gun3_5.png",      // 30
        "resources/weapons/gun3_6.png",      // 31
        "resources/weapons/gun3_shadow.png", // 32
        "resources/weapons/cut_1.png",       // 33
        "resources/weapons/cut_2.png",       // 34
        "resources/weapons/cut_3.png",       // 35
        "resources/weapons/cut_4.png",       // 36
        "resources/weapons/cut_5.png",       // 37
        "resources/weapons/cut_6.png",       // 38
        "resources/weapons/cut_shadow.png",  // 39
        "resources/face1.png",               // 40
        "resources/face2.png",               // 41
        "resources/face3.png",
    ])
}

pub fn load_font(font_filename: &str) -> rsfml::SfBox<Font> {
    Font::from_file(font_filename).unwrap_or_else(|| panic!("Loading font from {}", font_filename))
}

pub fn set_render_window_properties(render_window: &mut RenderWindow, args: &RustensteinOptions) {
    render_window.set_framerate_limit(args.fps.into());
    render_window.set_mouse_cursor_visible(args.cursor);
    render_window.set_mouse_position(Vector2i {
        x: args.window_size[0] as i32 / 2,
        y: args.window_size[1] as i32 / 2,
    });
}

pub fn create_render_window(title: &str, window_size: (u32, u32)) -> RenderWindow {
    let settings = ContextSettings::default();
    let video_mode = VideoMode::new(window_size.0, window_size.1, 32);
    RenderWindow::new(video_mode, title, Style::CLOSE, &settings)
}

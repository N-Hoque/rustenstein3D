#![allow(non_snake_case)]

extern crate rsfml;

pub mod animation;
pub mod event_handler;
pub mod fps;
pub mod game;
pub mod game_mode;
pub mod hud;
pub mod map;
pub mod mini_map;
pub mod raycasting_engine;
pub mod texture_loader;
pub mod weapon;

use rsfml::{
    graphics::{Font, RenderWindow},
    system::Vector2i,
    window::{ContextSettings, Style, VideoMode},
};

use game::GameLoop;
use texture_loader::TextureLoader;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "Rustenstein 3D", about = "Options for Rustenstein3D")]
struct RustensteinOptions {
    #[structopt(short, long, help = "Disables floor and sky rendering")]
    no_ground: bool,

    #[structopt(
        short,
        long,
        help = "Sets the FPS (Frames per second)",
        default_value = "30"
    )]
    fps: u8,

    #[structopt(
        short,
        long,
        help = "Set the size of the window",
        number_of_values = 2,
        value_names = &["width", "height"],
        default_value = "640 480"
    )]
    window_size: Vec<u16>,
}

fn load_resources() -> TextureLoader {
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

fn main() {
    // Check if a custom width is set.

    let args = RustensteinOptions::from_args();

    // Create the render_window.
    let settings = ContextSettings::default();

    let (width, height) = (args.window_size[0] as u32, args.window_size[1] as u32);

    let video_mode = VideoMode::new(width, height, 32);
    // let video_mode = VideoMode::new_init(512, 384, 32);
    let mut render_window =
        RenderWindow::new(video_mode, "Rustenstein 3D", Style::CLOSE, &settings);

    render_window.set_framerate_limit(args.fps.into());

    // hide the cursor.
    render_window.set_mouse_cursor_visible(false);

    // set the mouse positon on the center of the window
    render_window.set_mouse_position(Vector2i {
        x: width as i32 / 2,
        y: height as i32 / 2,
    });

    // Create the font for the FPS_handler.
    let font_filename = "resources/sansation.ttf";
    let font = Font::from_file(font_filename)
        .unwrap_or_else(|| panic!("Loading font from {}", font_filename));

    // Create the texture loader and load textures
    let texture_loader = load_resources();

    // Create the game_loop and activate the fps handler.
    let mut game_loop = GameLoop::new(render_window, &texture_loader, args.no_ground);
    game_loop.activate_FPS(&font);

    game_loop.run();
}

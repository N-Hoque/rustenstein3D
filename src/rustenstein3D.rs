#![allow(non_snake_case)]
#![crate_type = "bin"]

extern crate native;
extern crate rsfml;

pub mod FPS;
pub mod animation;
pub mod event_handler;
pub mod game;
pub mod game_mode;
pub mod hud;
pub mod map;
pub mod mini_map;
pub mod raycasting_engine;
pub mod texture_loader;
pub mod weapon;

use game::GameLoop;
use rsfml::graphics::{Font, RenderWindow};
use rsfml::system::Vector2i;
use rsfml::window::{ContextSettings, Style, VideoMode};

use texture_loader::TextureLoader;

#[cfg(target_os = "macos")]
#[start]
fn start(argc: int, argv: *const *const u8) -> int {
    native::start(argc, argv, main)
}

fn display_help() {
    println!("Rustenstein 3D:");
    println!("\t-w [width] [height]: Specify the window size.");
    println!("\t--no-ground: Disable the ground texturing (improves performance).");
    println!("\t--help: Display this help.");
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
    let args = std::env::args().collect::<Vec<_>>();
    let mut width = 768;
    let mut height = 480;
    let mut noground: bool = false;
    let mut i_args = 1;

    while i_args < args.len() {
        match args[i_args].as_str() {
            "--help" => {
                display_help();
                return;
            }
            "--noground" => noground = true,
            "-w" => {
                if i_args + 2 >= args.len() {
                    panic!("Rustenstein 3D: Missing arguments for -w option.");
                }
                width = args[i_args + 1].parse::<u32>().unwrap_or(768);
                height = args[i_args + 2].parse::<u32>().unwrap_or(480);
                i_args += 2;
            }
            _ => {
                println!("Rustenstein 3D: Unknown argument(s) given {:?}", &args[1..]);
                display_help();
                return;
            }
        }
        i_args += 1;
    }

    // Create the render_window.
    let settings = ContextSettings::default();
    let video_mode = VideoMode::new(width, height, 32);
    // let video_mode = VideoMode::new_init(512, 384, 32);
    let mut render_window =
        RenderWindow::new(video_mode, "Rustenstein 3D", Style::CLOSE, &settings);

    // set the framerate limit to 30 fps.
    render_window.set_framerate_limit(60);

    // hide the cursor.
    render_window.set_mouse_cursor_visible(false);

    // set the mouse positon on the center of the window
    render_window.set_mouse_position(Vector2i {
        x: width as i32 / 2,
        y: height as i32 / 2,
    });

    // Create the font for the FPS_handler.
    let font = Font::from_file("resources/sansation.ttf").expect("Loading resources/sansation.ttf");

    // Create the texture loader and load textures
    let texture_loader = load_resources();

    // Create the game_loop and activate the fps handler.
    let mut game_loop = GameLoop::new(render_window, &texture_loader, noground);
    game_loop.activate_FPS(&font);

    game_loop.run();
}

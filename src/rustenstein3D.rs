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
    println!("Arguments availables for rustenstein3D :");
    println!("\t-w [window_width] [window_height] : specify a new size for the window.");
    println!("\t--noground : diseable the ground texturing (improve performance).");
    println!("\t--help : display this help.");
}

fn load_texture() -> TextureLoader {
    let mut texture_loader = TextureLoader::new();
    if !texture_loader.load_texture("resources/ground.tga")  || // 0
       !texture_loader.load_texture("resources/1.tga")  || // 1
       !texture_loader.load_texture("resources/2.tga")  || // 2
       !texture_loader.load_texture("resources/3.tga")  || // 3
       !texture_loader.load_texture("resources/4.tga")  || // 4
       !texture_loader.load_texture("resources/5.tga")  || // 5
       !texture_loader.load_texture("resources/6.tga")  || // 6
       !texture_loader.load_texture("resources/7.tga")  || // 7
       !texture_loader.load_texture("resources/8.tga")  || // 8
       !texture_loader.load_texture("resources/9.tga")  || // 9
       !texture_loader.load_texture("resources/10.tga")  || // 10
       !texture_loader.load_texture("resources/sky.tga")  || // 11
       !texture_loader.load_texture("resources/weapons/gun_1.png")  || // 12
       !texture_loader.load_texture("resources/weapons/gun_2.png")  || // 13
       !texture_loader.load_texture("resources/weapons/gun_3.png")  || // 14
       !texture_loader.load_texture("resources/weapons/gun_4.png")  || // 15
       !texture_loader.load_texture("resources/weapons/gun_5.png")  || // 16
       !texture_loader.load_texture("resources/weapons/gun_6.png")  || // 17
       !texture_loader.load_texture("resources/weapons/gun_shadow.png")  || // 18
       !texture_loader.load_texture("resources/weapons/gun2_1.png")  || // 19
       !texture_loader.load_texture("resources/weapons/gun2_2.png")  || // 20
       !texture_loader.load_texture("resources/weapons/gun2_3.png")  || // 21
       !texture_loader.load_texture("resources/weapons/gun2_4.png")  || // 22
       !texture_loader.load_texture("resources/weapons/gun2_5.png")  || // 23
       !texture_loader.load_texture("resources/weapons/gun2_6.png")  || // 24
       !texture_loader.load_texture("resources/weapons/gun2_shadow.png")  || // 25
       !texture_loader.load_texture("resources/weapons/gun3_1.png")  || // 26
       !texture_loader.load_texture("resources/weapons/gun3_2.png")  || // 27
       !texture_loader.load_texture("resources/weapons/gun3_3.png")  || // 28
       !texture_loader.load_texture("resources/weapons/gun3_4.png")  || // 29
       !texture_loader.load_texture("resources/weapons/gun3_5.png")  || // 30
       !texture_loader.load_texture("resources/weapons/gun3_6.png")  || // 31
       !texture_loader.load_texture("resources/weapons/gun3_shadow.png")  || // 32
       !texture_loader.load_texture("resources/weapons/cut_1.png")  || // 33
       !texture_loader.load_texture("resources/weapons/cut_2.png")  || // 34
       !texture_loader.load_texture("resources/weapons/cut_3.png")  || // 35
       !texture_loader.load_texture("resources/weapons/cut_4.png")  || // 36
       !texture_loader.load_texture("resources/weapons/cut_5.png")  || // 37
       !texture_loader.load_texture("resources/weapons/cut_6.png")  || //38
       !texture_loader.load_texture("resources/weapons/cut_shadow.png")  || // 39
       !texture_loader.load_texture("resources/face1.png")  || //40
       !texture_loader.load_texture("resources/face2.png")  || //41
       !texture_loader.load_texture("resources/face3.png")
    {
        //42
        panic!("Error : Cannot load texture.");
    }
    texture_loader
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
                    panic!("Error missing arguments for -w option.");
                }
                width = args[i_args + 1].parse::<u32>().unwrap_or(768);
                height = args[i_args + 2].parse::<u32>().unwrap_or(480);
                i_args += 2;
            }
            _ => panic!("Error unknown argument."),
        }
        i_args += 1;
    }

    // Create the render_window.
    let settings = ContextSettings::default();
    let video_mode = VideoMode::new(width, height, 32);
    // let video_mode = VideoMode::new_init(512, 384, 32);
    let mut render_window = RenderWindow::new(video_mode, "Rustenstein3D", Style::CLOSE, &settings);

    // set the framerate limit to 30 fps.
    render_window.set_framerate_limit(30);

    // hide the cursor.
    render_window.set_mouse_cursor_visible(false);

    // set the mouse positon on the center of the window
    render_window.set_mouse_position(Vector2i {
        x: width as i32 / 2,
        y: height as i32 / 2,
    });

    // Create the font for the FPS_handler.
    let font = Font::from_file("resources/sansation.ttf")
        .expect("Error : Cannot load font, font resources/sansation.ttf doesn.t exist!");

    // Create the texture loader and load textures
    let texture_loader = load_texture();

    // Create the game_loop and activate the fps handler.
    let mut game_loop = game::GameLoop::new(render_window, &texture_loader, noground);
    game_loop.activate_FPS(&font);

    game_loop.run();
}

#![crate_type = "bin"]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(non_snake_case)]

extern crate native;
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

use std::error::Error;
use std::fs;
use std::path::Path;

use rsfml::{
    graphics::{Font, RenderWindow},
    system::Vector2i,
    window::{ContextSettings, Style, VideoMode},
};

use texture_loader::TextureLoader;

#[cfg(target_os = "macos")]
#[start]
fn start(argc: int, argv: *const *const u8) -> int {
    native::start(argc, argv, main)
}

fn display_help() -> () {
    println!("Arguments availables for rustenstein3D :");
    println!("\t-w [window_width] [window_height] : specify a new size for the window.");
    println!("\t--noground : diseable the ground texturing (improve performance).");
    println!("\t--help : display this help.");
}

fn get_resources_list<P: AsRef<Path>>(path: P) -> Result<Vec<String>, Box<dyn Error>> {
    let paths = fs::read_dir(path)?;

    let mut resource_list = Vec::new();

    for path in paths {
        let path_name = path?.path();
        if !path_name.ends_with(".png") && !path_name.ends_with(".tga") {
            continue;
        }

        if !path_name.is_dir() {
            resource_list.push(
                path_name
                    .to_str()
                    .ok_or("Cannot convert path to string")?
                    .to_string(),
            )
        } else {
            resource_list.append(&mut get_resources_list(path_name.as_path())?);
        }
    }

    Ok(resource_list)
}

fn load_texture() -> Result<TextureLoader, Box<dyn Error>> {
    let mut texture_loader = TextureLoader::new();
    let resources = get_resources_list("../../resources")?;
    for resource in resources {
        if !texture_loader.load_texture(resource.clone()) {
            panic!("ERROR: Cannot load texture ({}).", resource);
        }
    }
    Ok(texture_loader)
}

fn main() -> Result<(), Box<dyn Error>> {
    // Check if a custom width is set.
    let args = std::env::args().collect::<Vec<String>>();
    let arg_length = args.len();
    let mut width: u32 = 768;
    let mut height: u32 = 480;
    let mut noground: bool = false;
    let mut i_args = 1;

    while i_args < arg_length {
        let arg = &args[i_args];
        match arg.as_str() {
            "--help" => {
                display_help();
                return Ok(());
            }
            "--noground" => noground = true,
            "-w" => {
                if i_args + 2 >= arg_length {
                    panic!("Error missing arguments for -w option.");
                }
                width = args[i_args + 1]
                    .parse()
                    .expect("Error the first parameter after -w argument is not a width!");
                height = args[i_args + 2]
                    .parse()
                    .expect("Error the second parameter after -w argument is not a width!");
                i_args += 2;
            }
            _ => panic!("Error unknown argument ({}).", arg),
        }
        i_args += 1;
    }

    // Create the render_window.
    let settings = ContextSettings::default();
    let video_mode = VideoMode::new(width, height, 32);
    // let video_mode = VideoMode::new_init(512, 384, 32);
    let mut render_window = RenderWindow::new(video_mode, "Rustenstein3D", Style::CLOSE, &settings);

    // set the framerate limit to 30 fps.
    render_window.set_framerate_limit(40);

    // hide the cursor.
    render_window.set_mouse_cursor_visible(false);

    // set the mouse positon on the center of the window
    render_window.set_mouse_position(Vector2i::new(width as i32 / 2, height as i32 / 2));

    // Create the font for the FPS_handler.
    let font = Font::from_file("../../resources/sansation.ttf")
        .ok_or("ERROR: Cannot load font! Font (resources/sansation.ttf) does not exist!")
        .expect("The font has been loaded?");

    // Create the texture loader and load textures
    let texture_loader = load_texture()?;

    // Create the game_loop and activate the fps handler.
    let mut game_loop = game::GameLoop::new(render_window, &texture_loader, noground);
    game_loop.activate_FPS(&font);

    game_loop.run();
    Ok(())
}

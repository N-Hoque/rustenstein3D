#![allow(non_snake_case)]

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

use texture_loader::TextureLoader;

pub const RESOURCES_BASE_PATH: &'static str = "resources";

fn display_help() -> () {
    println!("Arguments available for Rustenstein3D:");
    println!("\t-w [window_width] [window_height] : Specify a new size for the window.");
    println!("\t--noground : Disable the ground texturing (improve performance).");
    println!("\t-f, --framerate [framerate_value] : Set the framerate of the game.");
    println!("\t--help : Display this help.");
}

fn get_resources_list<P: AsRef<Path>>(path: P) -> Result<Vec<String>, Box<dyn Error>> {
    let paths = fs::read_dir(path)?;

    let mut resource_list = Vec::new();

    for path in paths {
        let path_name = path?.path();
        if !path_name.ends_with(".png") && !path_name.ends_with(".tga") {
            continue;
        }

        if path_name.is_dir() {
            resource_list.append(&mut get_resources_list(path_name.as_path())?);
        } else {
            resource_list.push(
                path_name
                    .to_str()
                    .ok_or("Cannot convert path to string")?
                    .to_string(),
            )
        }
    }

    Ok(resource_list)
}

pub fn load_texture() -> Result<TextureLoader, Box<dyn Error>> {
    let mut texture_loader = TextureLoader::new();
    let resources = get_resources_list(RESOURCES_BASE_PATH)?;
    for resource in resources {
        if !texture_loader.load_texture(&resource) {
            panic!("ERROR: Cannot load texture ({}).", resource);
        }
    }
    Ok(texture_loader)
}

pub struct Arguments {
    window_dimensions: (u32, u32),
    no_ground: bool,
    framerate_limit: u32,
}

pub fn parse_arguments() -> Result<Arguments, Result<(), Box<dyn Error>>> {
    let args = std::env::args().collect::<Vec<String>>();
    let arg_length = args.len();

    let mut arguments = Arguments {
        window_dimensions: (768, 480),
        no_ground: false,
        framerate_limit: 30,
    };

    let mut i_args = 1;
    while i_args < arg_length {
        let arg = &args[i_args];
        match arg.as_str() {
            "--help" => {
                display_help();
                return Err(Ok(()));
            }
            "--noground" => arguments.no_ground = true,
            "-f" | "--framerate" => {
                if i_args + 1 >= arg_length {
                    panic!("ERROR: Missing argument for --framerate option.");
                }
                let framerate_value = &args[i_args + 1];
                arguments.framerate_limit = framerate_value.parse().expect(&format!(
                    "ERROR: Unable to parse value for --framerate ({})",
                    framerate_value
                ))
            }
            "-w" | "--width" => {
                if i_args + 2 >= arg_length {
                    panic!("Error missing arguments for -w option.");
                }
                let (width_arg, height_arg) = (&args[i_args + 1], &args[i_args + 2]);
                arguments.window_dimensions.0 = width_arg
                    .parse()
                    .expect("ERROR: First parameter after -w argument is not a width!");
                arguments.window_dimensions.1 = height_arg
                    .parse()
                    .expect("ERROR: Second parameter after -w argument is not a height!");
                i_args += 2;
            }
            _ => panic!("Error unknown argument ({}).", arg),
        }
        i_args += 1;
    }
    Ok(arguments)
}

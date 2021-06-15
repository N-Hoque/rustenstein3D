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

pub struct Arguments {
    pub window_dimensions: (u32, u32),
    pub no_ground: bool,
    pub framerate_limit: u32,
}

pub enum ParsedResult {
    Success,
    Parsed(Arguments),
    Failure(String),
}

fn display_help() -> () {
    println!("Arguments available for Rustenstein3D:");
    println!("\t-w [window_width] [window_height] : Specify a new size for the window.");
    println!("\t-f, --framerate [framerate_value] : Set the framerate of the game.");
    println!("\t--noground\t\t\t  : Disable the ground texturing (improve performance).");
    println!("\t--help\t\t\t\t  : Display this help.");
}

fn get_resources_list<P: AsRef<Path>>(path: P) -> Result<Vec<String>, Box<dyn Error>> {
    let paths = fs::read_dir(path)?;

    let mut resource_list = Vec::new();

    for path in paths {
        let path_name = path?.path();
        if path_name.is_dir() {
            let mut sub_resources = get_resources_list(path_name)?;
            resource_list.append(&mut sub_resources);
        } else {
            let extension = path_name
                .extension()
                .expect("ERROR: Cannot get file extension.");
            if extension != "wav" && extension != "ttf" {
                resource_list.push(path_name.display().to_string());
            }
        }
    }

    Ok(resource_list)
}

pub fn load_texture() -> Result<TextureLoader, Box<dyn Error>> {
    let mut texture_loader = TextureLoader::new();
    for resource in &get_resources_list(RESOURCES_BASE_PATH)? {
        texture_loader.load_texture(resource)?
    }
    Ok(texture_loader)
}

pub fn parse_arguments() -> ParsedResult {
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
                return ParsedResult::Success;
            }
            "--noground" => arguments.no_ground = true,
            "-f" | "--framerate" => {
                if i_args + 1 >= arg_length {
                    panic!("ERROR: Missing argument for --framerate option.");
                }
                let framerate_value = &args[i_args + 1];
                match framerate_value.parse() {
                    Ok(res) => arguments.framerate_limit = res,
                    Err(_) => {
                        return ParsedResult::Failure(format!(
                            "ERROR: Unable to parse value for --framerate ({})",
                            framerate_value
                        ))
                    }
                };
                i_args += 1;
            }
            "-w" | "--width" => {
                if i_args + 2 >= arg_length {
                    panic!("Error missing arguments for -w option.");
                }
                let (width_arg, height_arg) = (&args[i_args + 1], &args[i_args + 2]);
                match width_arg.parse() {
                    Ok(res) => arguments.window_dimensions.0 = res,
                    Err(_) => {
                        return ParsedResult::Failure(String::from(
                            "ERROR: First parameter after -w argument is not a width!",
                        ))
                    }
                };
                match height_arg.parse() {
                    Ok(res) => arguments.window_dimensions.1 = res,
                    Err(_) => {
                        return ParsedResult::Failure(String::from(
                            "ERROR: Second parameter after -w argument is not a height!",
                        ))
                    }
                };
                i_args += 2;
            }
            _ => panic!("Error unknown argument ({}).", arg),
        }
        i_args += 1;
    }
    ParsedResult::Parsed(arguments)
}

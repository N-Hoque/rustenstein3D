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

// TODO: Use this over the giant texture loading block
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
    if texture_loader.load_texture("resources/ground.tga").is_err() || // 0
       texture_loader.load_texture("resources/1.tga").is_err() || // 1
       texture_loader.load_texture("resources/2.tga").is_err() || // 2
       texture_loader.load_texture("resources/3.tga").is_err() || // 3
       texture_loader.load_texture("resources/4.tga").is_err() || // 4
       texture_loader.load_texture("resources/5.tga").is_err() || // 5
       texture_loader.load_texture("resources/6.tga").is_err() || // 6
       texture_loader.load_texture("resources/7.tga").is_err() || // 7
       texture_loader.load_texture("resources/8.tga").is_err() || // 8
       texture_loader.load_texture("resources/9.tga").is_err() || // 9
       texture_loader.load_texture("resources/10.tga").is_err() || // 10
       texture_loader.load_texture("resources/sky.tga").is_err() || // 11
       texture_loader.load_texture("resources/weapons/gun_1.png").is_err() || // 12
       texture_loader.load_texture("resources/weapons/gun_2.png").is_err() || // 13
       texture_loader.load_texture("resources/weapons/gun_3.png").is_err() || // 14
       texture_loader.load_texture("resources/weapons/gun_4.png").is_err() || // 15
       texture_loader.load_texture("resources/weapons/gun_5.png").is_err() || // 16
       texture_loader.load_texture("resources/weapons/gun_6.png").is_err() || // 17
       texture_loader.load_texture("resources/weapons/gun_shadow.png").is_err() || // 18
       texture_loader.load_texture("resources/weapons/gun2_1.png").is_err() || // 19
       texture_loader.load_texture("resources/weapons/gun2_2.png").is_err() || // 20
       texture_loader.load_texture("resources/weapons/gun2_3.png").is_err() || // 21
       texture_loader.load_texture("resources/weapons/gun2_4.png").is_err() || // 22
       texture_loader.load_texture("resources/weapons/gun2_5.png").is_err() || // 23
       texture_loader.load_texture("resources/weapons/gun2_6.png").is_err() || // 24
       texture_loader.load_texture("resources/weapons/gun2_shadow.png").is_err() || // 25
       texture_loader.load_texture("resources/weapons/gun3_1.png").is_err() || // 26
       texture_loader.load_texture("resources/weapons/gun3_2.png").is_err() || // 27
       texture_loader.load_texture("resources/weapons/gun3_3.png").is_err() || // 28
       texture_loader.load_texture("resources/weapons/gun3_4.png").is_err() || // 29
       texture_loader.load_texture("resources/weapons/gun3_2.png").is_err() || // 27
       texture_loader.load_texture("resources/weapons/gun3_3.png").is_err() || // 28
       texture_loader.load_texture("resources/weapons/gun3_4.png").is_err() || // 29
       texture_loader.load_texture("resources/weapons/gun3_5.png").is_err() || // 30
       texture_loader.load_texture("resources/weapons/gun3_6.png").is_err() || // 31
       texture_loader.load_texture("resources/weapons/gun3_shadow.png").is_err() || // 32
       texture_loader.load_texture("resources/weapons/cut_1.png").is_err() || // 33
       texture_loader.load_texture("resources/weapons/cut_2.png").is_err() || // 34
       texture_loader.load_texture("resources/weapons/cut_3.png").is_err() || // 35
       texture_loader.load_texture("resources/weapons/cut_4.png").is_err() || // 36
       texture_loader.load_texture("resources/weapons/cut_5.png").is_err() || // 37
       texture_loader.load_texture("resources/weapons/cut_6.png").is_err() || //38
       texture_loader.load_texture("resources/weapons/cut_shadow.png").is_err() || // 39
       texture_loader.load_texture("resources/face1.png").is_err() || //40
       texture_loader.load_texture("resources/face2.png").is_err() || //41
       texture_loader.load_texture("resources/face3.png").is_err()
    {
        panic!("Failed to load textures");
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

#![crate_type = "bin"]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(non_snake_case)]

extern crate native;
extern crate rsfml;

use std::error::Error;

use rsfml::{
    graphics::{Font, RenderWindow},
    system::Vector2i,
    window::{ContextSettings, Style, VideoMode},
};
use rustenstein3D::game::GameLoop;
use rustenstein3D::{load_texture, parse_arguments, RESOURCES_BASE_PATH};

#[cfg(target_os = "macos")]
#[start]
fn start(argc: int, argv: *const *const u8) -> int {
    native::start(argc, argv, main)
}

fn main() -> Result<(), Box<dyn Error>> {
    let (width, height, noground) = match parse_arguments() {
        Ok(value) => value,
        Err(value) => return value,
    };

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
    let font = Font::from_file(&format!("{}/sansation.ttf", RESOURCES_BASE_PATH))
        .ok_or("ERROR: Cannot load font! Font (resources/sansation.ttf) does not exist!")?;

    // Create the texture loader and load textures
    let texture_loader = load_texture()?;

    // Create the game_loop and activate the fps handler.
    let mut game_loop = GameLoop::new(render_window, &texture_loader, noground);
    game_loop.activate_FPS(&font);

    game_loop.run();
    Ok(())
}

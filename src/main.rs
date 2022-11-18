use rustenstein_3d::{
    create_render_window, game::MainLoop, load_font, load_resources, set_render_window_properties,
    RustensteinOptions,
};

use clap::Parser;

fn main() {
    let args = RustensteinOptions::parse();

    let mut render_window = create_render_window(
        "Rustenstein 3D",
        (args.window_size[0] as u32, args.window_size[1] as u32),
    );

    set_render_window_properties(&mut render_window, &args);

    let font = load_font("resources/sansation.ttf");

    let texture_loader = load_resources();

    let mut game_loop = MainLoop::new(render_window, &texture_loader);
    if args.disable_plane_rendering {
        game_loop.disable_planes();
    }
    game_loop.enable_fps(&font);
    if args.disable_fps_counter {
        game_loop.disable_fps();
    }
    game_loop.run();
}

use rustenstein_3d::{
    create_render_window, game::GameLoop, load_font, load_resources, set_render_window_properties,
    RustensteinOptions,
};
use structopt::StructOpt;

fn main() {
    let mut args = RustensteinOptions::from_args();

    if args.window_size.is_empty() {
        args.window_size = vec![640, 480];
    } else if args.window_size[0] == 0 {
        args.window_size[0] = 640;
    } else if args.window_size[1] == 0 {
        args.window_size[1] = 480;
    }

    let mut render_window = create_render_window(
        "Rustenstein 3D",
        (args.window_size[0] as u32, args.window_size[1] as u32),
    );

    set_render_window_properties(&mut render_window, &args);

    let font = load_font("resources/sansation.ttf");

    let texture_loader = load_resources();

    let mut game_loop = GameLoop::new(render_window, &texture_loader, args.no_ground);
    game_loop.enable_fps(&font);
    game_loop.run();
}

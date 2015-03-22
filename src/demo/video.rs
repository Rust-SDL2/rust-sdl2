use sdl2;
use sdl2_image;
use sdl2_image::LoadSurface;
use sdl2::event::Event;
use sdl2::keycode::KeyCode;
use std::path::Path;

// fail when error
macro_rules! trying(
    ($e:expr) => (match $e { Ok(e) => e, Err(e) => panic!("failed: {}", e) })
);

pub fn main(png: &Path) {
    let sdl_context = sdl2::init(sdl2::INIT_VIDEO).unwrap();
    sdl2_image::init(sdl2_image::INIT_PNG | sdl2_image::INIT_JPG);

    let window = trying!(sdl2::video::Window::new(
            "rust-sdl2 demo: Video", sdl2::video::WindowPos::PosCentered,
            sdl2::video::WindowPos::PosCentered, 800, 600, sdl2::video::OPENGL));

    let renderer = trying!(sdl2::render::Renderer::from_window(
            window, sdl2::render::RenderDriverIndex::Auto, sdl2::render::ACCELERATED));

    // Load a surface, and convert it to a texture bound to the renderer
    let surface = trying!(LoadSurface::from_file(png));
    let mut texture = trying!(renderer.create_texture_from_surface(&surface));

    // // Load a texture directly via the renderer
    // let texture = match renderer.load_texture(png) {
    //     Ok(texture) => texture,
    //     Err(err) => panic!(format!("Could not set render target: {}", err))
    // };

    let mut drawer = renderer.drawer();
    drawer.set_draw_color(sdl2::pixels::Color::RGBA(255, 255, 255, 255));
    drawer.clear();

    drawer.copy(&mut texture, None, None);
    drawer.present();

    let mut event_pump = sdl_context.event_pump();

    'mainloop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit{..} => break 'mainloop,
                Event::KeyDown {keycode: KeyCode::Escape, ..} => break 'mainloop,
                _ => {}
            }
        }
    }

    sdl2_image::quit();
}

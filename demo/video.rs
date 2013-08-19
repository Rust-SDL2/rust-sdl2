use sdl2;

pub fn main() {
    sdl2::init([sdl2::InitVideo]);

    let window = match sdl2::video::Window::new("rust-sdl2 demo: Video", sdl2::video::PosCentered, sdl2::video::PosCentered, 800, 600, [sdl2::video::OpenGL]) {
        Ok(window) => window,
        Err(err) => fail!(fmt!("failed to create window: %s", err))
    };

    let renderer = match sdl2::render::Renderer::from_window(window, sdl2::render::DriverAuto, [sdl2::render::Accelerated]) {
        Ok(renderer) => renderer,
        Err(err) => fail!(fmt!("failed to create renderer: %s", err))
    };

    renderer.set_draw_color(sdl2::video::RGB(255, 0, 0));
    renderer.clear();
    renderer.present();
    sdl2::quit();
}

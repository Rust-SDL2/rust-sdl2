use sdl2;
use sdl2_ttf;

static SCREEN_WIDTH : int = 800;
static SCREEN_HEIGHT : int = 600;

// fail when error
macro_rules! trying(
    ($e:expr) => (match $e { Ok(e) => e, Err(e) => fail!("failed: {}", e) })
)

// hadle the annoying Rect i32
macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        sdl2::rect::Rect::new($x as i32, $y as i32, $w as i32, $h as i32)
    )
)

pub fn main(filename: &Path) {
    sdl2::init(sdl2::InitVideo);
    sdl2_ttf::init();

    let window = trying!(sdl2::video::Window::new(
            "rust-sdl2 demo: Video", sdl2::video::PosCentered,
            sdl2::video::PosCentered, SCREEN_WIDTH, SCREEN_HEIGHT, sdl2::video::OpenGL));

    let renderer = trying!(sdl2::render::Renderer::from_window(
            window, sdl2::render::DriverAuto, sdl2::render::Accelerated));

    // Load a font
    let font = trying!(sdl2_ttf::Font::from_file(filename, 128));

    // render a surface, and convert it to a texture bound to the renderer
    let surface = trying!(font.render_str_blended("Hello Rust!", sdl2::pixels::Color::new(255, 0, 0, 255)));
    let texture = trying!(renderer.create_texture_from_surface(surface));

    renderer.set_draw_color(sdl2::pixels::Color::new(195, 217, 255, 255));
    renderer.clear();

    let (w, h) = match texture.query() {
        Ok(q) => (q.width, q.height),
        Err(err) => fail!(format!("Failed to query texture: {}", err))
    };
    renderer.copy(texture, None, Some(rect!((SCREEN_WIDTH - w)/ 2, (SCREEN_HEIGHT - h)/ 2, w, h)));

    renderer.present();

    'main : loop {
        'event : loop {
            match sdl2::event::poll_event() {
                sdl2::event::QuitEvent(_) => break 'main,
                sdl2::event::KeyDownEvent(_, _, key, _, _) => {
                    if key == sdl2::keycode::EscapeKey {
                        break 'main
                    }
                }
                _ => {}
            }
        }
    }
    sdl2_ttf::quit();
    sdl2::quit();
}

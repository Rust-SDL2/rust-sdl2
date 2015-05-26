extern crate sdl2;

use sdl2::pixels::Color;

pub fn main() {
    let mut sdl_context = sdl2::init().video().unwrap();

    let window = sdl_context.window("rust-sdl2 demo: Window", 800, 600)
        .resizable()
        .build()
        .unwrap();

    let mut renderer = window.renderer().present_vsync().build().unwrap();

    let mut running = true;
    let mut tick = 0;

    while running {
        for event in sdl_context.event_pump().poll_iter() {
            use sdl2::event::Event;
            use sdl2::keycode::KeyCode;

            match event {
                Event::Quit {..} | Event::KeyDown { keycode: KeyCode::Escape, .. } => {
                    running = false
                },
                _ => {}
            }
        }

        {
            // Update the window title.
            // &sdl_context is needed to safely access the Window and to ensure that the event loop
            // isn't running (which could mutate the Window).

            // Note: if you don't use renderer: window.properties(&sdl_context);
            let mut props = renderer.window_properties(&sdl_context).unwrap();

            let position = props.get_position();
            let size = props.get_size();
            let title = format!("Window - pos({}x{}), size({}x{}): {}", position.0, position.1, size.0, size.1, tick);
            props.set_title(&title);

            tick += 1;
        }

        let mut drawer = renderer.drawer();
        drawer.set_draw_color(Color::RGB(0, 0, 0));
        drawer.clear();
        drawer.present();
    }
}

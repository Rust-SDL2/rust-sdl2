extern crate sdl2;

use sdl2::event::Event;
use sdl2::impl_as_vertex_traits;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::FPoint;
use sdl2::render::{AsVertexColor, VertexIndices};
use std::thread;
use std::time::Duration;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Rust SDL2 render_geometry custom struct example", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut running = true;

    while running {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    running = false;
                }
                _ => {}
            }
        }

        // black background
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        // `render_geometry` supports any custom struct as long as it contains the needed data
        // (or other layout compatible of the needed data).
        // The struct does not need to be `repr(C)` or `Copy` for example.
        struct MyVertex {
            // For demonstration purposes color is `[u8; 4]` here. `[u8; 4]` is layout-compatible
            // with `sdl2::pixels::Color`
            color: [u8; 4],
            // The struct may contain data not needed by SDL.
            #[expect(dead_code)]
            foo: Vec<u8>,
            // When defining your own vertex struct, using `FPoint` for position and tex_coord
            // (and `Color` for color) is the easiest way (see the trait impls below)
            pos: FPoint,
        }

        // The unsafe trait to get the vertex position can simply be generated with a macro.
        // This macro makes sure the implementation is sound, this is only possible when the field
        // has the exact right type.
        impl_as_vertex_traits!(impl AsVertexPosition(self.pos) for MyVertex);

        // The unsafe trait to get the vertex color must be implementated manually because the type
        // of the `color` field is not `sdl2::pixels::Color`.
        // Also make sure to not violate the contract of this unsafe trait!
        // SAFETY: `as_vertex_color` only returns a borrow of a field of `self`.
        unsafe impl AsVertexColor for MyVertex {
            fn as_vertex_color(&self) -> &Color {
                // SAFETY: [u8; 4] has the same layout as Color
                unsafe { &*(&self.color as *const [u8; 4] as *const Color) }
            }
        }

        // Define the triangles
        let vertices = [
            MyVertex {
                color: [0xff, 0, 0, 0xff],
                foo: b"some".to_vec(),
                pos: FPoint::new(100.0, 500.0),
            },
            MyVertex {
                color: [0, 0xff, 0, 0xff],
                foo: b"unrelated".to_vec(),
                pos: FPoint::new(700.0, 500.0),
            },
            MyVertex {
                color: [0, 0, 0xff, 0xff],
                foo: b"data".to_vec(),
                pos: FPoint::new(400.0, 100.0),
            },
        ];

        // Actually render
        canvas
            .render_geometry(
                &vertices,
                &vertices,
                None::<(&sdl2::render::Texture<'_>, &[sdl2::render::Vertex])>,
                VertexIndices::Sequential,
            )
            .expect("render_geometry failed");

        canvas.present();
        thread::sleep(Duration::from_millis(16));
    }
}

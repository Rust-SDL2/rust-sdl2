extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::FPoint;
use sdl2::render::{RenderGeometryTextureParams, Vertex, VertexIndices};
use std::mem::offset_of;
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

        // First, draw a triangle using `render_geometry`. The `tex_coord` fields are unused but
        // must be provided, `render_geometry` only supports `sdl2::render::Vertex`.
        let vertices = [
            Vertex {
                position: FPoint::new(100.0, 200.0),
                color: Color::RED,
                tex_coord: FPoint::new(0.0, 0.0),
            },
            Vertex {
                position: FPoint::new(200.0, 200.0),
                color: Color::GREEN,
                tex_coord: FPoint::new(0.0, 0.0),
            },
            Vertex {
                position: FPoint::new(150.0, 100.0),
                color: Color::BLUE,
                tex_coord: FPoint::new(0.0, 0.0),
            },
        ];
        canvas
            .render_geometry(&vertices, None, VertexIndices::Sequential)
            .expect("render_geometry failed (probably unsupported, see error message)");

        // `render_geometry_raw` supports any custom struct as long as it contains the needed data
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
            // (and `Color` for color) is the easiest way. These are obviously layout-compatible
            // with `FPoint` and `Color`, respectively.
            pos: FPoint,
        }

        // Define the vertices of a square
        let vertices = [
            MyVertex {
                color: [0, 0, 0, 0xff],
                foo: b"some".to_vec(),
                pos: FPoint::new(300.0, 100.0),
            },
            MyVertex {
                color: [0, 0xff, 0, 0xff],
                foo: b"unrelated".to_vec(),
                pos: FPoint::new(400.0, 100.0),
            },
            MyVertex {
                color: [0xff, 0, 0, 0xff],
                foo: b"data".to_vec(),
                pos: FPoint::new(300.0, 200.0),
            },
            MyVertex {
                color: [0xff, 0xff, 0, 0xff],
                foo: b"!".to_vec(),
                pos: FPoint::new(400.0, 200.0),
            },
        ];

        // A square is rendered as two triangles (see indices)
        // SAFETY: core::mem::offset_of makes sure the offsets are right.
        unsafe {
            canvas.render_geometry_raw(
                &vertices,
                offset_of!(MyVertex, pos),
                &vertices,
                offset_of!(MyVertex, color),
                None::<RenderGeometryTextureParams<()>>,
                &[[0, 1, 2], [1, 2, 3]],
            )
        }
        .expect("render_geometry_raw failed (probably unsupported, see error message)");

        // Parameters can be reused, here only the positions are swapped out for new ones.
        // SAFETY: core::mem::offset_of makes sure the offsets are right.
        //         The offset 0 is correct because the element type of positions is `[f32; 2]`.
        unsafe {
            canvas.render_geometry_raw(
                &[
                    [500.0f32, 100.0],
                    [600.0, 100.0],
                    [500.0, 200.0],
                    [600.0, 200.0],
                ],
                0,
                &vertices,
                offset_of!(MyVertex, color),
                None::<RenderGeometryTextureParams<()>>,
                &[[0, 1, 2], [1, 2, 3]],
            )
        }
        .expect("render_geometry_raw failed (probably unsupported, see error message)");

        canvas.present();
        thread::sleep(Duration::from_millis(16));
    }
}

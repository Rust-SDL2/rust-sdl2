/// Minimal example for getting sdl2 and wgpu working together with raw-window-handle.
extern crate pollster;
extern crate sdl2;
extern crate wgpu;

use std::borrow::Cow;

use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;

fn main() -> Result<(), String> {
    // Show logs from wgpu
    env_logger::init();

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("Raw Window Handle Example", 800, 600)
        .position_centered()
        .resizable()
        .build()
        .map_err(|e| e.to_string())?;
    let (width, height) = window.size();

    let instance = wgpu::Instance::new(wgpu::Backends::PRIMARY);
    let surface = unsafe { instance.create_surface(&window) };
    let adapter_opt = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::HighPerformance,
        compatible_surface: Some(&surface),
    }));
    let adapter = match adapter_opt {
        Some(a) => a,
        None => return Err(String::from("No adapter found")),
    };

    let (device, queue) = match pollster::block_on(adapter.request_device(
        &wgpu::DeviceDescriptor {
            limits: wgpu::Limits::default(),
            label: Some("device"),
            features: wgpu::Features::empty(),
        },
        None,
    )) {
        Ok(a) => a,
        Err(e) => return Err(e.to_string()),
    };

    let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
        label: Some("shader"),
        source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl"))),
    });

    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        entries: &[],
        label: Some("bind_group_layout"),
    });
    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &bind_group_layout,
        entries: &[],
        label: Some("bind_group"),
    });
    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        bind_group_layouts: &[&bind_group_layout],
        label: None,
        push_constant_ranges: &[],
    });

    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            buffers: &[],
            module: &shader,
            entry_point: "vs_main",
        },
        fragment: Some(wgpu::FragmentState {
            targets: &[wgpu::ColorTargetState {
                format: wgpu::TextureFormat::Bgra8UnormSrgb,
                blend: None,
                write_mask: wgpu::ColorWrites::ALL,
            }],
            module: &shader,
            entry_point: "fs_main",
        }),
        //rasterization_state: Some(wgpu::RasterizationStateDescriptor {
        //    depth_bias: 0,
        //    depth_bias_slope_scale: 0.0,
        //    depth_bias_clamp: 0.0,
        //}),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Front),
            clamp_depth: false,
            polygon_mode: wgpu::PolygonMode::Fill,
            conservative: false,
        },
        //color_states: &[wgpu::ColorStateDescriptor {
        //    format: wgpu::TextureFormat::Bgra8UnormSrgb,
        //    color_blend: wgpu::BlendDescriptor::REPLACE,
        //    alpha_blend: wgpu::BlendDescriptor::REPLACE,
        //    write_mask: wgpu::ColorWrite::ALL,
        //}],
        //vertex_state: wgpu::VertexStateDescriptor {
        //    index_format: wgpu::IndexFormat::Uint16,
        //    vertex_buffers: &[],
        //},
        depth_stencil: None,
        label: None,
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
    });

    let mut config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface.get_preferred_format(&adapter).unwrap(),
        width,
        height,
        present_mode: wgpu::PresentMode::Mailbox,
    };
    surface.configure(&device, &config);

    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Window {
                    window_id,
                    win_event: WindowEvent::SizeChanged(width, height),
                    ..
                } if window_id == window.id() => {
                    config.width = width as u32;
                    config.height = height as u32;
                    surface.configure(&device, &config);
                }
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                e => {
                    dbg!(e);
                }
            }
        }

        let frame_res = surface.get_current_frame();
        let frame = match frame_res {
            Ok(a) => a,
            Err(e) => return Err(format!("Timeout getting next texture: {}", e)),
        };
        let output = frame
            .output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("command_encoder"),
        });

        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &output,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
                label: None,
            });
            rpass.set_pipeline(&render_pipeline);
            rpass.set_bind_group(0, &bind_group, &[]);
            rpass.draw(0..3, 0..1);
        }

        queue.submit([encoder.finish()]);
    }

    Ok(())
}

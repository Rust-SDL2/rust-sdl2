/// Minimal example for getting sdl2 and wgpu working together with raw-window-handle.
///
/// TODO: update wgpu and this example after https://github.com/gfx-rs/wgpu/pull/462 ships

extern crate glsl_to_spirv;
extern crate libc;
#[cfg(target_os = "macos")]
extern crate objc;
extern crate raw_window_handle;
extern crate sdl2;
extern crate wgpu;

use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};

use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use sdl2::video::Window;

/// sdl2 implements raw-window-handle correctly, but wgpu does not for macOS
/// wgpu wrongly expects an NSView to be provided by raw-window-handle, so we have to do a little more work
struct WindowWrapper<'a>(pub &'a Window);

unsafe impl<'a> HasRawWindowHandle for WindowWrapper<'a> {
    #[cfg(not(target_os = "macos"))]
    /// all non-mac platforms work correctly, so return the handle directly
    fn raw_window_handle(&self) -> RawWindowHandle {
        self.0.raw_window_handle()
    }

    #[cfg(target_os = "macos")]
    /// do some work on macOS to get the root NSView for the NSWindow returned by sdl2
    fn raw_window_handle(&self) -> RawWindowHandle {
        use objc::{msg_send, sel, sel_impl};
        use objc::runtime::Object;
        use raw_window_handle::macos::MacOSHandle;
        let handle = self.0.raw_window_handle();
        match handle {
            RawWindowHandle::MacOS(macos_handle) => {
                RawWindowHandle::MacOS(MacOSHandle {
                    ns_window: macos_handle.ns_window,
                    ns_view: unsafe { msg_send![macos_handle.ns_window as *mut Object, contentView] },
                    ..MacOSHandle::empty()
                })
            }
            _ => unreachable!()
        }
    }
}

fn load_glsl(code: &str, ty: glsl_to_spirv::ShaderType) -> Result<Vec<u32>, String> {
    let spirv = glsl_to_spirv::compile(&code, ty)?;
    let result = wgpu::read_spirv(spirv).map_err(|e| e.to_string())?;
    Ok(result)
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("Raw Window Handle Example", 800, 600)
        .position_centered()
        .resizable()
        .build()
        .map_err(|e| e.to_string())?;
    let (width, height) = window.size();

    let adapter_opt = wgpu::Adapter::request(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::Default,
        backends: wgpu::BackendBit::PRIMARY,
    });
    let adapter = match adapter_opt {
        Some(a) => a,
        None => return Err(String::from("No adapter found")),
    };

    let (device, mut queue) = adapter.request_device(&wgpu::DeviceDescriptor {
        extensions: wgpu::Extensions {
            anisotropic_filtering: false,
        },
        limits: wgpu::Limits::default(),
    });

    let vs = include_str!("shader.vert");
    let vs_spirv = &load_glsl(vs, glsl_to_spirv::ShaderType::Vertex)?;
    let vs_module = device.create_shader_module(vs_spirv);

    let fs = include_str!("shader.frag");
    let fs_spirv = &load_glsl(fs, glsl_to_spirv::ShaderType::Fragment)?;
    let fs_module = device.create_shader_module(fs_spirv);

    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { bindings: &[] });
    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &bind_group_layout,
        bindings: &[],
    });
    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        bind_group_layouts: &[&bind_group_layout],
    });

    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        layout: &pipeline_layout,
        vertex_stage: wgpu::ProgrammableStageDescriptor {
            module: &vs_module,
            entry_point: "main",
        },
        fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
            module: &fs_module,
            entry_point: "main",
        }),
        rasterization_state: Some(wgpu::RasterizationStateDescriptor {
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: wgpu::CullMode::None,
            depth_bias: 0,
            depth_bias_slope_scale: 0.0,
            depth_bias_clamp: 0.0,
        }),
        primitive_topology: wgpu::PrimitiveTopology::TriangleList,
        color_states: &[wgpu::ColorStateDescriptor {
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            color_blend: wgpu::BlendDescriptor::REPLACE,
            alpha_blend: wgpu::BlendDescriptor::REPLACE,
            write_mask: wgpu::ColorWrite::ALL,
        }],
        depth_stencil_state: None,
        index_format: wgpu::IndexFormat::Uint16,
        vertex_buffers: &[],
        sample_count: 1,
        sample_mask: !0,
        alpha_to_coverage_enabled: false,
    });

    // use the WindowWrapper to make sure wgpu will work on macOS
    let surface = wgpu::Surface::create(&WindowWrapper(&window));

    let mut sc_desc = wgpu::SwapChainDescriptor {
        usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
        format: wgpu::TextureFormat::Bgra8UnormSrgb,
        width,
        height,
        present_mode: wgpu::PresentMode::Vsync,
    };

    let mut swap_chain = device.create_swap_chain(&surface, &sc_desc);

    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Window {
                    win_event: WindowEvent::Resized(width, height),
                    ..
                } => {
                    sc_desc.width = width as u32;
                    sc_desc.height = height as u32;
                    swap_chain = device.create_swap_chain(&surface, &sc_desc);
                }
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                _ => {}
            }
        }

        let frame = swap_chain.get_next_texture();
        let mut encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { todo: 0 });
        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: &frame.view,
                    resolve_target: None,
                    load_op: wgpu::LoadOp::Clear,
                    store_op: wgpu::StoreOp::Store,
                    clear_color: wgpu::Color::GREEN,
                }],
                depth_stencil_attachment: None,
            });
            rpass.set_pipeline(&render_pipeline);
            rpass.set_bind_group(0, &bind_group, &[]);
            rpass.draw(0 .. 3, 0 .. 1);
        }

        queue.submit(&[encoder.finish()]);
    }

    Ok(())
}

use std::fmt::Debug;
use winit::{
    event::*,
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowBuilder},
};
use winit::event_loop::EventLoopWindowTarget;
use crate::render::renderer::{Renderer};

pub struct Application {}

impl Application {
    // Creating app
    pub async fn new() -> Application {
        cfg_if::cfg_if! {
            if #[cfg(target_arch = "wasm32")] {
                std::panic::set_hook(Box::new(console_error_panic_hook::hook));
                console_log::init_with_level(log::Level::Info).expect("Couldn't initialize logger");
            } else {
                env_logger::init();
            }
        }
        Self {}
    }

    pub async fn start(&mut self) {
        let event_loop = EventLoop::new().unwrap();
        let window = WindowBuilder::new().build(&event_loop).unwrap();
        #[cfg(target_arch = "wasm32")]
        {
            // Winit prevents sizing with CSS, so we have to set
            // the size manually when on web.
            use winit::dpi::PhysicalSize;

            use winit::platform::web::WindowExtWebSys;
            web_sys::window()
                .and_then(|win| win.document())
                .and_then(|doc| {
                    let dst = doc.get_element_by_id("root")?;
                    let canvas = web_sys::Element::from(window.canvas()?);
                    dst.append_child(&canvas).ok()?;
                    Some(())
                })
                .expect("Couldn't append canvas to document body.");

            let _ = window.request_inner_size(PhysicalSize::new(450, 400));
        }
        let mut renderer = Renderer::new(&window).await;
        event_loop.run(move |event, control_flow| {
            match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                }
                if window_id == renderer.window().id() => {
                    self.on_event(&mut renderer, &event, control_flow)
                }
                _ => {}
            }
        })
            .unwrap()
    }

    pub fn on_event(&mut self, renderer: &mut Renderer, event: &WindowEvent, control_flow: &EventLoopWindowTarget<()>) {
        if renderer.input(&event) {
            return;
        }
        // UPDATED!
        match event {
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                event:
                KeyEvent {
                    state: ElementState::Pressed,
                    physical_key: PhysicalKey::Code(KeyCode::Escape),
                    ..
                },
                ..
            } => control_flow.exit(),
            WindowEvent::Resized(physical_size) => {
                log::info!("physical_size: {physical_size:?}");
                renderer.resize(*physical_size, false);
            }
            WindowEvent::RedrawRequested => {
                // This tells winit that we want another frame after this one
                renderer.window().request_redraw();

                if !renderer.surface_configured() {
                    return;
                }

                renderer.update();
                match renderer.render() {
                    Ok(_) => {}
                    // Reconfigure the surface if it's lost or outdated
                    Err(
                        wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated,
                    ) => renderer.reinit_size(),
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SurfaceError::OutOfMemory) => {
                        log::error!("OutOfMemory");
                        control_flow.exit();
                    }
                    // This happens when the a frame takes too long to present
                    Err(wgpu::SurfaceError::Timeout) => {
                        log::warn!("Surface timeout")
                    }
                }
            }
            _ => {}
        }
    }
}

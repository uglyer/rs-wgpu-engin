use winit::{
    event::*,
    keyboard::{KeyCode, PhysicalKey},
    window::{Window},
};
use winit::event_loop::EventLoopWindowTarget;
use crate::render::renderer::{Renderer};

pub struct Application<'a> {
    renderer: Renderer<'a>,
}

impl<'a> Application<'a> {
    // Creating some of the wgpu types requires async code
    pub async fn new(window: &'a Window) -> Application<'a> {
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
        let renderer = Renderer::new(window).await;
        Self {
            renderer,
        }
    }

    pub fn window(&self) -> &Window {
        &self.renderer.window()
    }

    pub fn renderer(&self) -> &Renderer {
        &self.renderer
    }

    pub fn on_event(&mut self, event: &WindowEvent, control_flow: &EventLoopWindowTarget<()>) {
        if self.renderer.input(&event) {
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
                self.renderer.resize(*physical_size, false);
            }
            WindowEvent::RedrawRequested => {
                // This tells winit that we want another frame after this one
                self.window().request_redraw();

                if !self.renderer.surface_configured() {
                    return;
                }

                self.renderer.update();
                match self.renderer.render() {
                    Ok(_) => {}
                    // Reconfigure the surface if it's lost or outdated
                    Err(
                        wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated,
                    ) => self.renderer.reinit_size(),
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

use tabletop::Tabletop;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

mod card;
mod renderer;
mod tabletop;
mod texture;
use renderer::Renderer;

use winit::{
    event::*,
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::WindowBuilder,
};

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub async fn run() {
    cfg_if::cfg_if! {
        if #[cfg(target_arch="wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Info).expect("Couldn't initialize logger");
        } else {
            env_logger::init();
        }
    }

    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    cfg_if::cfg_if! {
        if #[cfg(target_arch="wasm32")] {
            use winit::dpi::PhysicalSize;
            let _ = window.request_inner_size(PhysicalSize::new(450, 400));

            use winit::platform::web::WindowExtWebSys;
            web_sys::window()
                .and_then(|win| win.document())
                .and_then(|doc| {
                    let dst = doc.get_element_by_id("wasm-example")?;
                    let canvas = web_sys::Element::from(window.canvas()?);
                    dst.append_child(&canvas).ok()?;
                    Some(())
                })
                .expect("Couldn't append canvas to document body");
        }
    }

    let mut surface_configured = false;
    let mut renderer = Renderer::new(&window).await;
    let tabletop = Tabletop::new();

    let _ = event_loop.run(move |event, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == renderer.window().id() => match event {
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        state: ElementState::Pressed,
                        physical_key: PhysicalKey::Code(KeyCode::Escape),
                        ..
                    },
                ..
            } => {
                log::info!("{:?}", event);
                control_flow.exit()
            }
            WindowEvent::CursorMoved {
                device_id,
                position,
            } => {
                log::info!("{:?}: {:?}", device_id, position);
            }
            WindowEvent::MouseInput {
                device_id,
                state,
                button,
            } => {
                log::info!("{:?}: {:?} ({:?})", device_id, button, state);
            }
            WindowEvent::Resized(physical_size) => {
                surface_configured = true;
                renderer.resize(*physical_size);
            }
            WindowEvent::RedrawRequested => {
                renderer.window().request_redraw();

                if !surface_configured {
                    return;
                }

                renderer.update();

                let (vertices, indices) = renderer.prerender(&tabletop);

                match renderer.render(vertices, indices) {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                        renderer.resize(renderer.size)
                    }
                    Err(wgpu::SurfaceError::OutOfMemory) => {
                        log::error!("OutOfMemory");
                        control_flow.exit();
                    }
                    Err(wgpu::SurfaceError::Timeout) => {
                        log::warn!("Surface Timeout");
                    }
                }
            }
            _ => {}
        },
        _ => {}
    });
}

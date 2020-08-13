use glow::*;
use glutin::event::Event;
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;
fn main() {
    let event_loop = EventLoop::new();
    let wb = WindowBuilder::new().with_title("A fantastic window!");

    let windowed_context = ContextBuilder::new()
        .with_vsync(true)
        .build_windowed(wb, &event_loop)
        .unwrap();

    let windowed_context = unsafe { windowed_context.make_current().unwrap() };

    let gl =
        glow::Context::from_loader_function(|s| windowed_context.get_proc_address(s) as *const _);
    unsafe {
        gl.clear_color(0.1, 0.2, 0.3, 1.0);
    }

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            Event::RedrawRequested(_) => {
                unsafe {
                    gl.clear(glow::COLOR_BUFFER_BIT);
                }

                 windowed_context.swap_buffers().unwrap();
                //std::thread::sleep(std::time::Duration::from_millis(16));
                windowed_context.window().request_redraw();
            }
            Event::LoopDestroyed => {
                return;
            }
            _ => (),
        }
    });
}

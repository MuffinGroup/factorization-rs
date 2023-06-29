extern crate glium;
use glium::implement_vertex;
#[allow(unused_imports)]
use glium::{glutin, Surface};

fn main() {
    let event_loop = glium::glutin::event_loop::EventLoop::new();
    let wb = glium::glutin::window::WindowBuilder::new()
        .with_inner_size(glium::glutin::dpi::LogicalSize::new(1024.0, 768.0))
        .with_title("Hello world");
    let cb = glium::glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
    }

    implement_vertex!(Vertex, position);

    // vertexes
    let vertex1 = Vertex { position: [-0.5, -0.5] };
    let vertex2 = Vertex { position: [0.0, -0.5] };
    let vertex3 = Vertex { position: [-0.5, -0.5] };

    // event loop
    event_loop.run(move |event, _, control_flow| {
        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        match event {
            // if the event is window realted
            glutin::event::Event::WindowEvent { event, .. } => match event {
                // if the window is request to be closed
                glutin::event::WindowEvent::CloseRequested => {
                    // close the window and end the program
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                }
                // else
                _ => {},
            },

            // starts a new loop if there's a cause for it
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => {},
            },
            _ => {},
        }
        // drawing to the screen
        let mut target = display.draw();
        // filling the screen
        target.clear_color(10.0, 0.0, 1.0, 1.0);
        // ending the drawing sequence
        target.finish().unwrap();
    });
}
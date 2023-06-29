#[macro_use]
extern crate glium;

mod glsl_reader;

#[allow(unused_imports)]
use glium::{glutin, Surface};

fn main() {

    // event loop creation
    let event_loop = glutin::event_loop::EventLoop::new();
    // the window
    let wb = glutin::window::WindowBuilder::new()
        .with_inner_size(glium::glutin::dpi::LogicalSize::new(1024.0, 768.0))
        .with_title("Hello world");
    let cb = glutin::ContextBuilder::new();
    // Displays the window
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    #[derive(Copy, Clone)]
    // Vertex struct creation
    struct Vertex {
        position: [f32; 2],
    }

    // Vertex implementation
    implement_vertex!(Vertex, position);

    // Vertex properties
    let vertex1 = Vertex {
        position: [0.5, 0.0],
    };
    let vertex2 = Vertex {
        position: [0.5, 0.5],
    };
    let vertex3 = Vertex {
        position: [-0.5, -0.0],
    };
    let vertex4 = Vertex {
        position: [0.5, 0.5],
    };
    let vertex5 = Vertex {
        position: [-0.5, 0.5],
    };
    let vertex6 = Vertex {
        position: [-0.5, -0.0],
    };
    
    let mut shape = vec![vertex1, vertex2, vertex3, vertex4, vertex5, vertex6];
    // Vertex handler
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    // Shaders
    // controls the position
    let vertex_shader_src = &glsl_reader::read("vertex_shader.vert");
    //controls the color
    let fragment_shader_src = &glsl_reader::read("fragment_shader.frag");

    let program =
        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
            .unwrap();

        event_loop.run(move |event, _, control_flow| {
            let next_frame_time =
                std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
            *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
        
            match event {
                glutin::event::Event::WindowEvent { event, .. } => match event {
                    glutin::event::WindowEvent::CloseRequested => {
                        *control_flow = glutin::event_loop::ControlFlow::Exit;
                        return;
                    }
                    glutin::event::WindowEvent::KeyboardInput { input, .. } => {
                        if let Some(keycode) = input.virtual_keycode {
                            match keycode {
                                glutin::event::VirtualKeyCode::Up => {
                                    // Update the vertex positions
                                    for vertex in &mut shape {
                                        vertex.position[1] += 0.01;  // Update Y coordinate
                                    }
                                }
                                glutin::event::VirtualKeyCode::Down => {
                                    // Update the vertex positions
                                    for vertex in &mut shape {
                                        vertex.position[1] -= 0.01;  // Update Y coordinate
                                    }
                                }
                                glutin::event::VirtualKeyCode::Left => {
                                    // Update the vertex positions
                                    for vertex in &mut shape {
                                        vertex.position[0] -= 0.01;  // Update X coordinate
                                    }
                                }
                                glutin::event::VirtualKeyCode::Right => {
                                    // Update the vertex positions
                                    for vertex in &mut shape {
                                        vertex.position[0] += 0.01;  // Update X coordinate
                                    }
                                }
                                _ => (),
                            }
                        }
                    }
                    _ => (),
                },
                glutin::event::Event::NewEvents(cause) => match cause {
                    glutin::event::StartCause::ResumeTimeReached { .. } => (),
                    glutin::event::StartCause::Init => (),
                    _ => return,
                },
                _ => return,
            }
        
        
            // Update the vertex buffer with the modified positions
            vertex_buffer.write(&shape);
        
            let mut target = display.draw();
            target.clear_color(0.0, 1.0, 1.0, 1.0);
            target
                .draw(
                    &vertex_buffer,
                    &indices,
                    &program,
                    &glium::uniforms::EmptyUniforms,
                    &Default::default(),
                )
                .unwrap();
            target.finish().unwrap();
        });
            
}

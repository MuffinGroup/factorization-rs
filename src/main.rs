#[macro_use]
extern crate glium;
extern crate chrono;
extern crate image;

mod shader_reader;
mod image_loader;
mod info_types;
mod logger;

use glium::{
    glutin::{self, event::MouseButton},
    Surface,
};
use info_types::InfoTypes::*;

use crate::{image_loader::load_image, logger::log};

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
        tex_coords: [f32; 2],
        rgb: [f32; 3], // TODO: !!! Move this to uniforms !!!
    }

    // Vertex implementation
    implement_vertex!(Vertex, position, tex_coords, rgb);

    // Vertex properties
    let vertex1 = Vertex {
        position: [-0.5, -0.5],
        tex_coords: [0.0, 0.0],
        rgb: [1.0, 1.0, 1.0],
    };
    let vertex2 = Vertex {
        position: [0.0, 0.5],
        tex_coords: [0.0, 1.0],
        rgb: [1.0, 1.0, 1.0],
    };
    let vertex3 = Vertex {
        position: [0.5, -0.25],
        tex_coords: [1.0, 0.0],
        rgb: [1.0, 1.0, 1.0],
    };

    let vertex4 = Vertex {
        position: [0.0, -0.5],
        tex_coords: [0.0, 0.0],
        rgb: [1.0, 1.0, 1.0],
    };
    let vertex5 = Vertex {
        position: [0.5, 0.5],
        tex_coords: [0.0, 1.0],
        rgb: [1.0, 1.0, 1.0],
    };
    let vertex6 = Vertex {
        position: [-0.5, 0.0],
        tex_coords: [1.0, 0.0],
        rgb: [1.0, 1.0, 1.0],
    };
    let vertex7 = Vertex {
        position: [0.0, 0.5],
        tex_coords: [0.0, 1.0],
        rgb: [1.0, 1.0, 1.0],
    };
    let vertex8 = Vertex {
        position: [-0.5, 0.0],
        tex_coords: [1.0, 0.0],
        rgb: [1.0, 1.0, 1.0],
    };
    let vertex9 = Vertex {
        position: [0.5, 0.0],
        tex_coords: [1.0, 0.0],
        rgb: [0.0, 0.0, 1.0],
    };

    let square_vertex1 = Vertex {
        position: [-0.5, -0.5],
        tex_coords: [0.0, 0.0],
        rgb: [0.0, 0.0, 0.0],
    };
    let square_vertex2 = Vertex {
        position: [-0.5, 0.5],
        tex_coords: [0.0, 0.0],
        rgb: [0.0, 0.0, 0.0],
    };
    let square_vertex3 = Vertex {
        position: [0.5, -0.5],
        tex_coords: [0.0, 0.0],
        rgb: [1.0, 1.0, 1.0],
    };
    let square_vertex4 = Vertex {
        position: [0.5, -0.5],
        tex_coords: [0.0, 0.0],
        rgb: [1.0, 1.0, 1.0],
    };
    let square_vertex5 = Vertex {
        position: [-0.5, 0.5],
        tex_coords: [0.0, 0.0],
        rgb: [0.0, 0.0, 0.0],
    };
    let square_vertex6 = Vertex {
        position: [0.5, 0.5],
        tex_coords: [0.0, 0.0],
        rgb: [1.0, 1.0, 1.0],
    };

    let mut shape = vec![vertex1, vertex2, vertex3];
    let mut vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let shape2 = vec![vertex4, vertex5, vertex6, vertex7, vertex8, vertex9];
    let vertex_buffer_shape_2 = glium::VertexBuffer::new(&display, &shape2).unwrap();

    let shape3 = vec![
        square_vertex1,
        square_vertex2,
        square_vertex3,
        square_vertex4,
        square_vertex5,
        square_vertex6,
    ];
    let vertex_buffer_shape_3 = glium::VertexBuffer::new(&display, &shape3).unwrap();

    let vertex_shader_src = include_str!("../resources/shaders/vertex_shader.vert");
    let fragment_shader_color_src = include_str!("../resources/shaders/fragment_shader_color.frag");
    let fragment_shader_texture_src = include_str!("../resources/shaders/fragment_shader_texture.frag");

    let vertex_shader = &shader_reader::read(vertex_shader_src, "vertex_shader");

    let fragment_shader_texture = &shader_reader::read(fragment_shader_texture_src, "fragment_shader_texture");

    let fragment_shader_color = &shader_reader::read(fragment_shader_color_src ,"fragment_shader_color.frag");

    let program =
        glium::Program::from_source(&display, vertex_shader, fragment_shader_texture, None)
            .unwrap();

    let program_2 =
        glium::Program::from_source(&display, vertex_shader, fragment_shader_color, None).unwrap();

    let texture_bytes = include_bytes!("../resources/textures/test_2.png");
    let texture = load_image(texture_bytes, "test2", &display);

    let mut t: f32 = -0.5;

    // execute once
    log("Started succesful", INFO.literal());
    
    // execute always
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
                                    vertex.position[1] += 0.01; // Update Y coordinate
                                }
                            }
                            glutin::event::VirtualKeyCode::Down => {
                                // Update the vertex positions
                                for vertex in &mut shape {
                                    vertex.position[1] -= 0.01; // Update Y coordinate
                                }
                            }
                            glutin::event::VirtualKeyCode::Left => {
                                // Update the vertex positions
                                for vertex in &mut shape {
                                    vertex.position[0] -= 0.01; // Update X coordinate
                                }
                            }
                            glutin::event::VirtualKeyCode::Right => {
                                // Update the vertex positions
                                for vertex in &mut shape {
                                    vertex.position[0] += 0.01; // Update X coordinate
                                }
                            }
                            glutin::event::VirtualKeyCode::Escape => {
                                *control_flow = glutin::event_loop::ControlFlow::Exit;
                                log("Exited succesfully", INFO.literal());
                                return;
                            }
                            _ => (),
                        }
                        vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
                    }
                }
                glutin::event::WindowEvent::CursorMoved { position, .. } => {
                    for vertex in &mut shape {
                        vertex.position[0] += position.x as f32 / 10000000.0 - 0.005;
                        // Update X coordinate
                    }
                    vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
                }
                glutin::event::WindowEvent::MouseInput { state, button, .. } => {
                    match (state, button) {
                        (glutin::event::ElementState::Released, MouseButton::Left) => {
                            for vertex in &mut shape {
                                vertex.position[0] /= 1.2;
                                vertex.position[1] /= 1.2;
                            }
                        }
                        (glutin::event::ElementState::Released, MouseButton::Right) => {
                            for vertex in &mut shape {
                                vertex.position[0] *= 1.2;
                                vertex.position[1] *= 1.2;
                            }
                        }
                        _ => (),
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

        t += 0.002;
        if t > 0.5 {
            t = -0.5;
        }

        let mut target = display.draw();
        target.clear_color(0.0, 1.0, 1.0, 1.0);

        let uniforms = uniform! {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [ t , 0.0, 0.0, 1.0f32],
            ],
            tex: &texture
        };

        target
            .draw(
                &vertex_buffer,
                &indices,
                &program,
                &uniforms,
                &Default::default(),
            )
            .unwrap();
        target
            .draw(
                &vertex_buffer_shape_2,
                &indices,
                &program_2,
                &uniforms,
                &Default::default(),
            )
            .unwrap();

        target
            .draw(
                &vertex_buffer_shape_3,
                &indices,
                &program_2,
                &uniforms,
                &Default::default(),
            )
            .unwrap();
        target.finish().unwrap();
    });
}

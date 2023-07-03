#[macro_use]
extern crate glium;
extern crate image;
extern crate chrono;

mod glsl_reader;
mod logger;
mod info_types;

use glium::glutin::{self, event::MouseButton};
use std::io::Cursor;
use info_types::InfoTypes;
#[allow(unused_imports)]
use glium::Surface;

use crate::logger::log;

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
    }

    // Vertex implementation
    implement_vertex!(Vertex, position, tex_coords);

    // Vertex properties
    let vertex1 = Vertex {
        position: [-0.5, -0.5],
        tex_coords: [0.0, 0.0],
    };
    let vertex2 = Vertex {
        position: [0.0, 0.5],
        tex_coords: [0.0, 1.0],
    };
    let vertex3 = Vertex {
        position: [0.5, -0.25],
        tex_coords: [1.0, 0.0],
    };

    let mut shape = vec![vertex1, vertex2, vertex3];
    let mut vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let shape2 = vec![vertex1, vertex2, vertex3];
    let vertex_buffer_shape_2 = glium::VertexBuffer::new(&display, &shape2).unwrap();

    let vertex_shader_src = &glsl_reader::read("vertex_shader.vert");

    let fragment_shader_src = &glsl_reader::read("fragment_shader.frag");

    let program =
        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
            .unwrap();

    // execute once
    log("Started succesful", Some(InfoTypes::INFO.info_type()));

    let image = image::load(
        Cursor::new(&include_bytes!(
            "resources/textures/test.png"
        )),
        image::ImageFormat::Png,
    )
    .unwrap()
    .to_rgba8();

    let image_dimensions = image.dimensions();
    let image =
        glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    let texture = glium::texture::SrgbTexture2d::new(&display, image).unwrap();

    let mut t: f32 = -0.5;

    for vertex in &mut shape {
        vertex.position[1] += 1.0;
    }

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
                                println!("wheee");
                            }
                        }
                        (glutin::event::ElementState::Released, MouseButton::Right) => {
                            for vertex in &mut shape {
                                vertex.position[0] *= 1.2;
                                vertex.position[1] *= 1.2;
                                println!("wheee");
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
        
        // log("This is being printed every tick", Some(InfoTypes::WARNING.info_type()));
        // log("Print, print, print...", None); <- sets it to the INFO type

        let mut target = display.draw();
        target.clear_color(0.0, 1.0, 1.0, 1.0);

        let uniforms = uniform! {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [ t , 0.0, 0.0, 1.0f32],
            ],
            tex: &texture,
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
                &program,
                &glium::uniforms::EmptyUniforms,
                &Default::default(),
            )
            .unwrap();
        target.finish().unwrap();
    });
}

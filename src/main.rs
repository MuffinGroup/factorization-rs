#[macro_use]
#[allow(macro_use_extern_crate)]
extern crate glium;

mod teapot;
#[macro_use]
mod logger;
mod colors;
mod shapes;
mod util;
mod camera;

use shapes::get_shape;
use util::*;

fn main() {
    #[allow(unused_imports)]
    use glium::{glutin, Surface};

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let positions = glium::VertexBuffer::new(&display, &teapot::VERTICES).unwrap();
    let normals = glium::VertexBuffer::new(&display, &teapot::NORMALS).unwrap();
    let indices = glium::IndexBuffer::new(
        &display,
        glium::index::PrimitiveType::TrianglesList,
        &teapot::INDICES,
    )
    .unwrap();

    let mut zoom = 10.0;

    let square_shape = get_shape();

    let vertex_buffer = glium::VertexBuffer::new(&display, &square_shape).unwrap();
    let indices_2d = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let example_image_src = include_bytes!("../resources/textures/test.png");
    let example_image = load_image(example_image_src, "test.png", &display);

    let vertex_shader_src = include_str!("../resources/shaders/vertex_shader_3d.vert");
    let vertex_shader = load_shader(vertex_shader_src, "vertex_shader.vert");

    let fragment_shader_src = include_str!("../resources/shaders/fragment_shader_3d.frag");
    let fragment_shader = load_shader(fragment_shader_src, "fragment_shader_3d.frag");

    let vertex_shader_color_src = include_str!("../resources/shaders/vertex_shader_color.vert");
    let vertex_shader_color = load_shader(vertex_shader_color_src, "vertex_shader_color.vert");

    let fragment_shader_color_src = include_str!("../resources/shaders/fragment_shader_color.frag");
    let fragment_shader_color =
        load_shader(fragment_shader_color_src, "fragment_shader_color.frag");

    let vertex_shader_texture_src = include_str!("../resources/shaders/vertex_shader_texture.vert");
    let vertex_shader_texture =
        load_shader(vertex_shader_texture_src, "vertex_shader_texture.vert");

    let fragment_shader_texture_src =
        include_str!("../resources/shaders/fragment_shader_texture.frag");
    let fragment_shader_texture =
        load_shader(fragment_shader_texture_src, "fragment_shader_texture.frag");

    let program =
        glium::Program::from_source(&display, &vertex_shader, &fragment_shader, None).unwrap();

    let program_color =
        glium::Program::from_source(&display, &vertex_shader_color, &fragment_shader_color, None)
            .unwrap();

    let program_texture = glium::Program::from_source(
        &display,
        &vertex_shader_texture,
        &fragment_shader_texture,
        None,
    )
    .unwrap();

    log!("Started sucessful");

    event_loop.run(move |event, _, control_flow| {

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
                                zoom += 0.2;
                            }
                            glutin::event::VirtualKeyCode::Down => {
                                zoom -= 0.2;
                            }
                            _ => (),
                        }
                    }
                }
                _ => return,
            }
            glutin::event::Event::RedrawRequested(_) => {
                let mut target = display.draw();
                target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);
                let uniforms = uniform! {
                    matrix: [
                        [0.5, 0.0, 0.0, 0.0],
                        [0.0, 0.5, 0.0, 0.0],
                        [0.0, 0.0, 0.5, 0.0],
                        [0.5, 0.0, 0.0, 1.0f32],
                    ]
                };
        
                let texture_uniforms = uniform! {
                    matrix: [
                        [0.5, 0.0, 0.0, 0.0],
                        [0.0, 0.5, 0.0, 0.0],
                        [0.0, 0.0, 0.5, 0.0],
                        [-0.5, 0.0, 0.0, 1.0f32],
                    ],
                    tex: &example_image
                };
        
                let model = [
                    [0.0,    0.0,  -1.0,  0.0], // X-axis rotation (no change to X-axis values)
                    [0.0,    1.0,   0.0,  0.0], // Y-axis rotation (no change to Y-axis values)
                    [1.0,    0.0,   0.0,  0.0], // Z-axis rotation (no change to Z-axis values)
                    [0.0,    0.0,   0.0,  2.0f32], // Translation along the Z-axis (2.0)
                ];
                
                let perspective = {
                    let (width, height) = target.get_dimensions();
                    let aspect_ratio = height as f32 / width as f32;
        
                    let fov: f32 = 3.141592 / 3.0;
                    let zfar = 1024.0;
                    let znear = 0.1;
        
                    let f = 1.0 / (fov / 2.0).tan();
        
                    [
                        [f * aspect_ratio, 0.0, 0.0, 0.0],
                        [0.0, f, 0.0, 0.0],
                        [0.0, 0.0, (zfar + znear) / (zfar - znear), 1.0],
                        [0.0, 0.0, -(2.0 * zfar * znear) / (zfar - znear), 0.0],
                    ]
                };
                let light = [-1.0, 0.4, 0.9f32];
        
                let params = glium::DrawParameters {
                    depth: glium::Depth {
                        test: glium::DepthTest::IfLess,
                        write: true,
                        .. Default::default()
                        // TODO: Readd backface culling
                    },
                    .. Default::default()
                };
        
                let view = camera::view_matrix(&[2.0, -1.0, 1.0], &[-2.0, 1.0, 1.0], &[0.0, 1.0, 0.0], zoom);
        
                target
                    .draw(
                        (&positions, &normals),
                        &indices,
                        &program,
                        &uniform! { model: model, view: view, perspective: perspective, u_light: light },
                        &params,
                    )
                    .unwrap();
                target
                    .draw(
                        &vertex_buffer,
                        &indices_2d,
                        &program_color,
                        &uniforms,
                        &Default::default(),
                    )
                    .unwrap();
        
                target
                    .draw(
                        &vertex_buffer,
                        &indices_2d,
                        &program_texture,
                        &texture_uniforms,
                        &Default::default(),
                    )
                    .unwrap();
                target.finish().unwrap();
            }
            glutin::event::Event::RedrawEventsCleared => {
                display.gl_window().window().request_redraw();
            },
            _ => {}
        }
    });
}


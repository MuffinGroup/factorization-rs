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
// use std::io::Cursor;

use crate::{image_loader::load_image, logger::log};

mod teapot;

fn main() {
    #[allow(unused_imports)]
    use glium::{glutin, Surface};

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    #[derive(Copy, Clone)]
    // Vertex struct creation
    struct Vertex {
        position: [f32; 2],
        tex_coords: [f32; 2],
        rgb: [f32; 3], // TODO: !!! Move this to uniforms !!!
    }
  
    let positions = glium::VertexBuffer::new(&display, &teapot::VERTICES).unwrap();
    let normals = glium::VertexBuffer::new(&display, &teapot::NORMALS).unwrap();
    let indices = glium::IndexBuffer::new(
        &display,
        glium::index::PrimitiveType::TrianglesList,
        &teapot::INDICES,
    )
    .unwrap();

    let mut zoom = 10.0;

    let vertex_shader_src = r#"
    #version 140

    in vec3 position;
    in vec3 normal;
    
    out vec3 v_normal;
    
    uniform mat4 perspective;
    uniform mat4 view;
    uniform mat4 model;
    
    void main() {
        mat4 modelview = view * model;
        v_normal = transpose(inverse(mat3(modelview))) * normal;
        gl_Position = perspective * modelview * vec4(position, 1.0);
    }
    "#;
  
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

    let fragment_shader_src = r#"
        #version 150

        in vec3 v_normal;
        out vec4 color;
        uniform vec3 u_light;

        void main() {
            float brightness = dot(normalize(v_normal), normalize(u_light));
            vec3 dark_color = vec3(0.6, 0.0, 0.0);
            vec3 regular_color = vec3(1.0, 0.0, 0.0);
            color = vec4(mix(dark_color, regular_color, brightness), 1.0);
        }
    "#;

    let program =
        glium::Program::from_source(&display, vertex_shader, fragment_shader_texture, None)
            .unwrap();

    let program_2 =
        glium::Program::from_source(&display, vertex_shader, fragment_shader_color, None).unwrap();


    /*    
    let image = image::load(
        Cursor::new(&include_bytes!("../resources/textures/test.png")),
        image::ImageFormat::Png,
    )
    .unwrap()
    .to_rgba8();
    let image_dimensions = image.dimensions();
    let image =
        glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    let texture = glium::texture::SrgbTexture2d::new(&display, image).unwrap();
    */
    let texture_bytes = include_bytes!("../resources/textures/test_2.png");
    let texture = load_image(texture_bytes, "test2", &display);

    let mut t: f32 = -0.5;

    // execute once
    log("Started succesful", INFO.types());
    
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
                                zoom += 0.02;
                            }
                            glutin::event::VirtualKeyCode::Down => {
                                zoom -= 0.02;
                            }
                            glutin::event::VirtualKeyCode::Escape => {
                                *control_flow = glutin::event_loop::ControlFlow::Exit;
                                log("Exited succesfully", INFO.types());
                                return;
                            }
                            _ => (),
                        }
                    }
                }
                _ => return,
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => return,
            },
            _ => return,
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
            tex: &texture
        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

        let model = [
            [0.01, 0.0, 0.0, 0.0],
            [0.0, 0.01, 0.0, 0.0],
            [0.0, 0.0, 0.01, 0.0],
            [0.0, 0.0, 2.0, 1.0f32],
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
                ..Default::default()
            },
            backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
            ..Default::default()
        };

        let view = view_matrix(&[2.0, -1.0, 1.0], &[-2.0, 1.0, 1.0], &[0.0, 1.0, 0.0], zoom);

        target
            .draw(
                (&positions, &normals),
                &indices,
                &program,
                &uniform! { model: model, view: view, perspective: perspective, u_light: light },
                &params,
            )
            .unwrap();
        target.finish().unwrap();
    });
}

fn view_matrix(position: &[f32; 3], direction: &[f32; 3], up: &[f32; 3], zoom_factor: f32) -> [[f32; 4]; 4] {
    let f = {
        let f = direction;
        let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
        let len = len.sqrt();
        [f[0] / len, f[1] / len, f[2] / len]
    };

    // Calculate the normalized side vector (right)
    let s = [
        up[1] * f[2] - up[2] * f[1],
        up[2] * f[0] - up[0] * f[2],
        up[0] * f[1] - up[1] * f[0],
    ];

    // Calculate the normalized up vector
    let s_norm = {
        let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
        let len = len.sqrt();
        [s[0] / len, s[1] / len, s[2] / len]
    };

    // Calculate the final up vector
    let u = [
        f[1] * s_norm[2] - f[2] * s_norm[1],
        f[2] * s_norm[0] - f[0] * s_norm[2],
        f[0] * s_norm[1] - f[1] * s_norm[0],
    ];

    // Calculate the final position with zoom applied
    let p = [
        position[0] + f[0] * zoom_factor,
        position[1] + f[1] * zoom_factor,
        position[2] + f[2] * zoom_factor,
    ];

    // Construct the view matrix
    [
        [s_norm[0], u[0], -f[0], 0.0],
        [s_norm[1], u[1], -f[1], 0.0],
        [s_norm[2], u[2], -f[2], 0.0],
        [-s_norm[0] * p[0] - s_norm[1] * p[1] - s_norm[2] * p[2],
         -u[0] * p[0] - u[1] * p[1] - u[2] * p[2],
         f[0] * p[0] + f[1] * p[1] + f[2] * p[2], 1.0],
    ]
}
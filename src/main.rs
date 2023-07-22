#[macro_use]
#[allow(macro_use_extern_crate)]
extern crate glium;

mod teapot;
#[macro_use]
mod logger;
mod colors;
mod shapes;
mod util;

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

    let program_texture =
        glium::Program::from_source(&display, &vertex_shader_texture, &fragment_shader_texture, None)
            .unwrap();

    log!("Started sucessful");

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
    });
}

fn view_matrix(
    position: &[f32; 3],
    direction: &[f32; 3],
    up: &[f32; 3],
    zoom_factor: f32,
) -> [[f32; 4]; 4] {
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
        [
            -s_norm[0] * p[0] - s_norm[1] * p[1] - s_norm[2] * p[2],
            -u[0] * p[0] - u[1] * p[1] - u[2] * p[2],
            f[0] * p[0] + f[1] * p[1] + f[2] * p[2],
            1.0,
        ],
    ]
}
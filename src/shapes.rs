#[derive(Clone, Copy)]
pub struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
    rgb: [f32; 3],
}

implement_vertex!(Vertex, position, tex_coords, rgb);

#[allow(unused_variables)]
pub fn get_shape() -> Vec<Vertex> {
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

    let vertex1 = Vertex {
        position: [-0.5, -0.5],
        tex_coords: [0.0, 0.0],
        rgb: [1.0, 0.0, 1.0],
    };
    let vertex2 = Vertex {
        position: [0.0, 0.5],
        tex_coords: [0.0, 1.0],
        rgb: [0.0, 1.0, 1.0],
    };
    let vertex3 = Vertex {
        position: [0.5, -0.25],
        tex_coords: [1.0, 0.0],
        rgb: [1.0, 1.0, 1.0],
    };

    let square_shape = vec![
        Vertex {
            position: [-0.5, -0.5],
            rgb: [1.0, 1.0, 1.0],
            tex_coords: [0.0, 0.0]
        },
        Vertex {
            position: [ 0.5, -0.5],
            rgb: [1.0, 1.0, 1.0],
            tex_coords: [1.0, 0.0] 
        },
        Vertex {
            position: [ 0.5,  0.5],
            rgb: [1.0, 1.0, 1.0],
            tex_coords: [1.0, 1.0]
        },
    
        Vertex { 
            position: [ 0.5,  0.5],
            rgb: [1.0, 1.0, 1.0],
            tex_coords: [1.0, 1.0] 
        },
        Vertex { 
            position: [-0.5,  0.5],
            rgb: [1.0, 1.0, 1.0],
            tex_coords: [0.0, 1.0] 
        },
        Vertex { 
            position: [-0.5, -0.5],
            rgb: [1.0, 1.0, 1.0],
            tex_coords: [0.0, 0.0]
        },
    ];

    let triangle_shape = vec![
        vertex1,
        vertex2,
        vertex3
    ];
    // square_shape // returns the square shape
    square_shape
}
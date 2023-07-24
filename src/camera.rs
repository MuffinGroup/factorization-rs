pub fn view_matrix(
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

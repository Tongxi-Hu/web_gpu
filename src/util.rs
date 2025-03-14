pub fn create_annulus_vertices<const DIVISION: usize>(
    radius: f32,
    inner_radius: f32,
    start_angle: f32,
    end_angle: f32,
) -> [[f32; 12]; DIVISION] {
    let mut vertices = [[0.0; 12]; DIVISION];
    let angle_gap = (end_angle - start_angle) / (DIVISION as f32);
    for i in 0..DIVISION {
        let angle1 = start_angle + (i as f32) * angle_gap;
        let angle2 = start_angle + ((i + 1) as f32) * angle_gap;
        let (c1, s1, c2, s2) = (
            f32::cos(angle1),
            f32::sin(angle1),
            f32::cos(angle2),
            f32::sin(angle2),
        );

        // 2 triangles per subdivision
        //
        // 0--1 4
        // | / /|
        // |/ / |
        // 2 3--5
        vertices[i] = [
            // first triangle
            c1 * radius,
            s1 * radius,
            c2 * radius,
            s2 * radius,
            c1 * inner_radius,
            s1 * inner_radius,
            // second triangle
            c1 * inner_radius,
            s1 * inner_radius,
            c2 * radius,
            s2 * radius,
            c2 * inner_radius,
            s2 * inner_radius,
        ]
    }

    vertices
}

struct Vertex {
    @location(0) position: vec2<f32>,
}

struct Inter {
    @builtin(position) position: vec4<f32>,
}

@vertex
fn vs(vertex: Vertex) -> Inter {
    var output: Inter;
    output.position = vec4<f32>(vertex.position, 0.0, 1.0);
    return output;
}

@fragment
fn fs() -> @location(0) vec4<f32> {
    return vec4<f32>(0.5,0.5,0.5,1.0);
}
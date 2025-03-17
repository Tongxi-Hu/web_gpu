struct Info {
    color: vec4<f32>,
    offset: vec2<f32>,
}

struct Vertex {
    @location(0) position: vec2<f32>,
    @location(1) color: vec4<f32>,
    @location(2) offset: vec2<f32>,
    @location(3) scales: vec2<f32>,
}

struct Inter {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec4<f32>,
}

@vertex
fn vs(vertex: Vertex, @builtin(instance_index) instance_index: u32) -> Inter {
    var output: Inter;
    output.color = vertex.color;
    output.position = vec4<f32>(vertex.position * vertex.scales + vertex.offset, 0.0, 1.0);
    return output;
}

@fragment
fn fs(inter: Inter) -> @location(0) vec4<f32> {
    return inter.color;
}
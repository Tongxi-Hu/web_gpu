struct Vertex {
    @location(0) position: vec2<f32>,
}

struct Uni {
    color: vec4<f32>,
    resolution: vec2<f32>,
    transform: mat3x3<f32>,
}

struct Inter {
    @builtin(position) position: vec4<f32>,
}

@group(0) @binding(0)
var<uniform> uni: Uni;

@vertex
fn vs(vertex: Vertex) -> Inter {
    let position = (uni.transform * vec3<f32>(vertex.position,1.0)).xy;
    let zero_to_one = position / uni.resolution;
    let zero_to_two = zero_to_one * 2.0;
    let flipped_clip_space = zero_to_two - 1.0;
    let clip_space = flipped_clip_space * vec2<f32>(1.0, - 1.0);
    var output: Inter;
    output.position = vec4<f32>(clip_space, 0.0, 1.0);
    return output;
}

@fragment
fn fs() -> @location(0) vec4<f32> {
    return uni.color;
}
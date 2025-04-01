struct Vertex {
    @location(0) position: vec2<f32>,
}

struct Uniform {
    color: vec4<f32>,
    resolution: vec2<f32>,
    translation: vec2<f32>
}

struct Inter {
    @builtin(position) position: vec4<f32>,
}

@group(0) @binding(0)
var<uniform> uni: Uniform;

@vertex
fn vs(vert: Vertex) -> Inter {
    let position = vert.position + uni.translation;
    let zero_to_one = position / uni.resolution;
    let zero_to_two = zero_to_one * 2.0;
    let flipped_clip_space = zero_to_two - 1.0;
    let clip_space = flipped_clip_space * vec2<f32>(1.0, - 1.0);
    var out: Inter;
    out.position = vec4<f32>(clip_space, 0.0, 1.0);
    return out;
}

@fragment
fn fs(vs: Inter) -> @location(0) vec4<f32> {
    return uni.color;
}
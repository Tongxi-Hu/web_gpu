struct Info{
    color: vec4<f32>,
    scale: vec2<f32>,
    offset:vec2<f32>,
}

@group(0) @binding(0) var<uniform> info: Info;

@vertex
fn vs(@builtin(vertex_index) vertex_index: u32,) -> @builtin(position) vec4<f32>{
    var pos= array<vec2<f32>, 3>(vec2<f32>(0.0,0.5),vec2<f32>(-0.5,-0.5),vec2<f32>(0.5,-0.5));
    return vec4<f32>(pos[vertex_index] * info.scale + info.offset, 0.0, 1.0);
}

@fragment
fn fs() -> @location(0) vec4<f32>{
    return info.color;
}
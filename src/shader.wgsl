struct Info{
    color: vec4<f32>,
    offset:vec2<f32>,
}

struct Inter{
    @builtin(position) position:vec4<f32>,
    @location(0) color: vec4<f32>,
}

@group(0) @binding(0) var<storage,read> infos: array<Info>;
@group(0) @binding(1) var<storage,read> scales: array<vec2<f32>>;
@group(0) @binding(2) var<storage,read> positions: array<vec2<f32>>;

@vertex
fn vs(
    @builtin(vertex_index) vertex_index: u32, 
    @builtin(instance_index) instance_index:u32
    ) -> Inter{
    let info=infos[instance_index];
    let scale=scales[instance_index];
    var output:Inter;
    output.color= info.color;
    output.position=vec4<f32>(positions[vertex_index] * scale + info.offset, 0.0, 1.0);
    return output;
}



@fragment
fn fs(inter: Inter) -> @location(0) vec4<f32>{
    return inter.color;
}
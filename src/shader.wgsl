struct VertexOutput{
    @builtin(position) position: vec4f,
    @location(0) color: vec4f,
}

@vertex
fn vs(@builtin(vertex_index) vertex_index: u32,) -> VertexOutput{
    var out: VertexOutput;

    var pos= array<vec2f,3>(vec2f(0.0,0.5),vec2f(-0.5,-0.5),vec2f(0.5,-0.5));

    var color=array<vec4f,3>(vec4f(1.0,0.0,0.0,1.0),vec4f(0.0,1.0,0.0,1.0),vec4f(0.0,0.0,1.0,1.0));
    
    out.position=vec4f(pos[vertex_index], 0.0, 1.0);

    out.color=color[vertex_index];

    return out;
}

@fragment
fn fs(fs_input:VertexOutput) -> @location(0) vec4<f32>{
    return fs_input.color;
}
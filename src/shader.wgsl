struct VertexOutput{
    @builtin(position) position: vec4f,
}

@vertex
fn vs(@builtin(vertex_index) vertex_index: u32,) -> VertexOutput{
    var out: VertexOutput;
    var pos= array<vec2f,3>(vec2<f32>(0.0,0.5),vec2<f32>(-0.5,-0.5),vec2<f32>(0.5,-0.5));
    out.position=vec4f(pos[vertex_index], 0.0, 1.0);
    return out;
}

@fragment
fn fs(fs_input: VertexOutput) -> @location(0) vec4<f32>{
    let red = vec4f(1.0, 0.0, 0.0, 1.0);
    let cyan = vec4f(0.0, 1.0, 1.0, 1.0);
    let grid = vec2i(fs_input.position.xy) / 8;
    let checker = (grid.x + grid.y) % 2 == 1;
    return select(red, cyan, checker);
}
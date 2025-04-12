#import bevy_sprite::mesh2d_vertex_output::VertexOutput
#import noisy_bevy::simplex_noise_2d
#import bevy_pbr::mesh_view_bindings::view;

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

struct Globals {
    time: f32,
    delta_time: f32,
    frame_count: u32,
}

@group(0) @binding(1)
var<uniform> globals: Globals;

@vertex
fn vertex(input: Vertex) -> VertexOutput {
    var output: VertexOutput;
    output.position = vec4(input.position.xy * 2.0, 0.0, 1.0);
    output.uv = input.uv;
    return output;
}

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    var t = globals.time * 0.5;
    var noise2 = simplex_noise_2d(mesh.uv * 2.5 * vec2(view.viewport.z / view.viewport.w, 5.0) + vec2(sin(t) * 0.5, -t)) / 2.0 + 0.5;
    var noise1 = simplex_noise_2d(mesh.uv * 2.5 * vec2(view.viewport.z / view.viewport.w, 5.0) + vec2(sin(t) * 0.5, t)) / 2.0 + 0.5;
    var noise3 = simplex_noise_2d(mesh.uv * 20.0 * vec2(view.viewport.z / view.viewport.w, 5.0) + vec2(sin(t * 2.0) * 0.5, t * 2.0)) / 2.0 + 0.5;
    var light = noise1 * 0.3 + 0.25 + noise2 * 0.20 + noise3 * 0.1;
    return vec4(vec3(floor(light * 8.0) / 8.0) * (1.0 - mesh.uv.y), 1.0);
}

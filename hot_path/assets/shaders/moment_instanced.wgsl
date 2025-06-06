#import bevy_pbr::{
    mesh_functions,
    view_transformations::position_world_to_clip,
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::alpha_discard,
}

@group(2) @binding(0) var<uniform> base_color: vec4<f32>;
@group(2) @binding(1) var<uniform> time: f32;
@group(2) @binding(2) var<uniform> glow_intensity: f32;

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    @location(3) color: vec4<f32>,
};

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;

    // Get the tag data encoded for this instance
    let tag = mesh_functions::get_tag(vertex.instance_index);
    var world_from_local = mesh_functions::get_world_from_local(vertex.instance_index);

    // Decode moment data from tag
    let decay_strength = f32((tag >> 24u) & 0xFFu) / 255.0;
    let thread_id = (tag >> 16u) & 0xFFu;
    let binding_count = (tag >> 8u) & 0xFFu;

    // Transform vertex to world space
    out.world_position = mesh_functions::mesh_position_local_to_world(world_from_local, vec4(vertex.position, 1.0));
    out.clip_position = position_world_to_clip(out.world_position.xyz);
    out.world_normal = mesh_functions::mesh_normal_local_to_world(vertex.normal, vertex.instance_index);
    out.uv = vertex.uv;

    // Create thread-based coloring
    let thread_hue = f32(thread_id) * 0.618033988749; // Golden ratio for nice color distribution
    let golden_base = vec3<f32>(1.0, 0.9, 0.6);
    let thread_tint = vec3<f32>(
        0.5 + 0.5 * sin(thread_hue * 6.28318 + 0.0),
        0.5 + 0.5 * sin(thread_hue * 6.28318 + 2.094),
        0.5 + 0.5 * sin(thread_hue * 6.28318 + 4.188)
    );
    
    // Pulse based on binding count
    let binding_pulse = 1.0 + 0.3 * sin(time * 3.0 + f32(binding_count) * 0.5);
    
    out.color = vec4<f32>(
        mix(golden_base, thread_tint, 0.3) * decay_strength * binding_pulse,
        decay_strength * 0.9
    );

    return out;
}

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    // Create orb glow effect
    let center = vec2<f32>(0.5, 0.5);
    let dist = distance(mesh.uv, center);
    let glow = 1.0 - smoothstep(0.3, 0.5, dist);
    
    // Pulsing glow based on time
    let pulse = 0.8 + 0.2 * sin(time * 2.0);
    
    var final_color = mesh.color;
    final_color = vec4<f32>(
        final_color.rgb * glow * pulse * glow_intensity,
        final_color.a * glow
    );
    
    return final_color;
} 
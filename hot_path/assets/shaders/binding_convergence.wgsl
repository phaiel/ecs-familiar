#import bevy_pbr::{
    mesh_functions,
    view_transformations::position_world_to_clip,
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::alpha_discard,
}

@group(2) @binding(0) var<uniform> convergence_color: vec4<f32>;
@group(2) @binding(1) var<uniform> time: f32;
@group(2) @binding(2) var<uniform> binding_strength: f32;

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

    // Decode binding data from tag
    let decay_strength = f32((tag >> 24u) & 0xFFu) / 255.0;
    let moment_count = (tag >> 16u) & 0xFFu;
    let thread_distance = f32((tag >> 8u) & 0xFFu) / 255.0;

    // Pulsing convergence effect
    let pulse_frequency = 2.0 + f32(moment_count) * 0.5;
    let pulse = 0.8 + 0.4 * sin(time * pulse_frequency);
    
    // Scale based on binding strength and pulse
    let scale_factor = 1.0 + binding_strength * pulse * 0.3;
    var scaled_position = vertex.position * scale_factor;

    // Transform vertex to world space
    out.world_position = mesh_functions::mesh_position_local_to_world(world_from_local, vec4(scaled_position, 1.0));
    out.clip_position = position_world_to_clip(out.world_position.xyz);
    out.world_normal = mesh_functions::mesh_normal_local_to_world(vertex.normal, vertex.instance_index);
    out.uv = vertex.uv;

    // Energy convergence coloring
    let energy_base = vec3<f32>(0.9, 0.4, 0.9);
    let energy_peak = vec3<f32>(1.0, 0.8, 1.0);
    
    // Convergence intensity based on thread proximity
    let convergence_intensity = (1.0 - thread_distance) * binding_strength;
    let energy_ripple = 0.5 + 0.5 * sin(time * 6.0 + thread_distance * 10.0);
    
    out.color = vec4<f32>(
        mix(energy_base, energy_peak, convergence_intensity * energy_ripple) * decay_strength * pulse,
        decay_strength * binding_strength * 0.7
    );

    return out;
}

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    // Create energy convergence patterns
    let center = vec2<f32>(0.5, 0.5);
    let dist_from_center = distance(mesh.uv, center);
    
    // Radial energy waves
    let wave_freq = 8.0;
    let wave_speed = 4.0;
    let energy_wave = sin(dist_from_center * wave_freq - time * wave_speed) * 0.3 + 0.7;
    
    // Convergence rings
    let ring_pattern = smoothstep(0.2, 0.25, dist_from_center) * 
                      smoothstep(0.45, 0.4, dist_from_center);
    
    // Core energy glow
    let core_glow = 1.0 - smoothstep(0.0, 0.3, dist_from_center);
    
    // Pulsing alpha for breathing effect
    let breath = 0.6 + 0.4 * sin(time * 2.0);
    
    var final_color = mesh.color;
    final_color = vec4<f32>(
        final_color.rgb * (energy_wave * (0.7 + 0.3 * ring_pattern) + core_glow * 0.5),
        final_color.a * breath * (0.5 + 0.5 * core_glow)
    );
    
    return final_color;
} 
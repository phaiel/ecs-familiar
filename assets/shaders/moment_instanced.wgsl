#import bevy_pbr::{
    mesh_functions,
    view_transformations::position_world_to_clip,
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::alpha_discard,
}

struct MomentMaterial {
    base_color: vec4<f32>,
    time: f32,
    glow_intensity: f32,
}

@group(2) @binding(0) var<uniform> material: MomentMaterial;

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    @location(3) decay_strength: f32,
    @location(4) thread_id: f32,
    @location(5) binding_count: f32,
    @location(6) glow_factor: f32,
}

// Decode moment data from MeshTag
// Bits: [decay_strength: 16][thread_id: 8][binding_count: 8]
fn decode_moment_data(tag: u32) -> vec3<f32> {
    let decay_strength = f32(tag & 0xFFFFu) / 65535.0;
    let thread_id = f32((tag >> 16u) & 0xFFu) / 255.0;
    let binding_count = f32((tag >> 24u) & 0xFFu) / 255.0;
    return vec3<f32>(decay_strength, thread_id, binding_count);
}

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;

    // Get the MeshTag data for this instance
    let tag = mesh_functions::get_tag(vertex.instance_index);
    let moment_data = decode_moment_data(tag);
    
    let decay_strength = moment_data.x;
    let thread_id = moment_data.y;
    let binding_count = moment_data.z;

    // Get world transform for this instance
    var world_from_local = mesh_functions::get_world_from_local(vertex.instance_index);
    
    // Apply pulsing animation based on decay and bindings
    let pulse_speed = 2.0 + binding_count * 3.0;
    let pulse_amplitude = 0.1 + binding_count * 0.05;
    let pulse_factor = 1.0 + sin(material.time * pulse_speed + thread_id * 10.0) * pulse_amplitude * decay_strength;
    
    // Scale the orb based on decay strength and pulse
    let scale_factor = (0.5 + decay_strength * 0.5) * pulse_factor;
    world_from_local[0] = world_from_local[0] * scale_factor;
    world_from_local[1] = world_from_local[1] * scale_factor;
    world_from_local[2] = world_from_local[2] * scale_factor;
    
    // Apply binding convergence effect (subtle position offset)
    var position = vertex.position;
    if binding_count > 0.0 {
        let convergence_offset = sin(material.time * 1.5 + thread_id * 5.0) * 0.02 * binding_count;
        position.y += convergence_offset;
    }

    // Transform to world space
    out.world_position = mesh_functions::mesh_position_local_to_world(world_from_local, vec4(position, 1.0));
    out.clip_position = position_world_to_clip(out.world_position.xyz);
    
    // Transform normal to world space
    out.world_normal = mesh_functions::mesh_normal_local_to_world(vertex.normal, vertex.instance_index);
    
    out.uv = vertex.uv;
    out.decay_strength = decay_strength;
    out.thread_id = thread_id;
    out.binding_count = binding_count;
    
    // Calculate glow factor based on decay and pulse
    out.glow_factor = decay_strength * pulse_factor * material.glow_intensity;
    
    return out;
}

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    // Base color varies by thread ID (subtle rainbow effect)
    let thread_hue = mesh.thread_id * 6.28318; // 2*PI for full color wheel
    let thread_color = vec3<f32>(
        0.5 + 0.5 * cos(thread_hue),
        0.5 + 0.5 * cos(thread_hue + 2.094), // 2*PI/3
        0.5 + 0.5 * cos(thread_hue + 4.188)  // 4*PI/3
    );
    
    // Blend thread color with golden moment base
    let golden_base = vec3<f32>(1.0, 0.9, 0.6);
    let final_color = mix(golden_base, thread_color, 0.3);
    
    // Add glow effect
    let glow_color = final_color * (1.0 + mesh.glow_factor * 2.0);
    
    // Add binding sparkle effect
    let sparkle = step(0.98, sin(material.time * 8.0 + mesh.thread_id * 20.0)) * mesh.binding_count;
    let sparkle_color = vec3<f32>(1.0, 1.0, 1.0) * sparkle * 0.5;
    
    // Fade based on decay strength
    let alpha = mesh.decay_strength * 0.9 + 0.1; // Never fully transparent
    
    return vec4<f32>(glow_color + sparkle_color, alpha);
} 
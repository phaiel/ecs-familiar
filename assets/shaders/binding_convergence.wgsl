#import bevy_pbr::{
    mesh_functions,
    view_transformations::position_world_to_clip,
}

struct BindingMaterial {
    convergence_color: vec4<f32>,
    time: f32,
    binding_strength: f32,
}

@group(2) @binding(0) var<uniform> material: BindingMaterial;

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
    @location(3) binding_strength: f32,
    @location(4) moment_count: f32,
    @location(5) thread_distance: f32,
    @location(6) energy_pulse: f32,
}

// Decode binding data from MeshTag
// Bits: [strength: 16][moment_count: 8][thread_distance: 8]
fn decode_binding_data(tag: u32) -> vec3<f32> {
    let strength = f32(tag & 0xFFFFu) / 65535.0;
    let moment_count = f32((tag >> 16u) & 0xFFu) / 255.0;
    let thread_distance = f32((tag >> 24u) & 0xFFu) / 255.0;
    return vec3<f32>(strength, moment_count, thread_distance);
}

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;

    // Get the MeshTag data for this instance
    let tag = mesh_functions::get_tag(vertex.instance_index);
    let binding_data = decode_binding_data(tag);
    
    let strength = binding_data.x;
    let moment_count = binding_data.y;
    let thread_distance = binding_data.z;

    // Get world transform for this instance
    var world_from_local = mesh_functions::get_world_from_local(vertex.instance_index);
    
    // Create pulsing convergence effect
    let pulse_speed = 4.0 + strength * 6.0;
    let pulse_amplitude = 0.3 + strength * 0.2;
    let pulse_factor = 1.0 + sin(material.time * pulse_speed) * pulse_amplitude;
    
    // Scale based on binding strength and pulse
    let scale_factor = (0.3 + strength * 0.7) * pulse_factor;
    world_from_local[0] = world_from_local[0] * scale_factor;
    world_from_local[1] = world_from_local[1] * scale_factor;
    world_from_local[2] = world_from_local[2] * scale_factor;
    
    // Add energy ripple effect
    var position = vertex.position;
    let ripple_intensity = strength * moment_count;
    let ripple_offset = sin(material.time * 8.0 + length(position) * 10.0) * ripple_intensity * 0.1;
    position += vertex.normal * ripple_offset;

    // Transform to world space
    out.world_position = mesh_functions::mesh_position_local_to_world(world_from_local, vec4(position, 1.0));
    out.clip_position = position_world_to_clip(out.world_position.xyz);
    
    // Transform normal to world space
    out.world_normal = mesh_functions::mesh_normal_local_to_world(vertex.normal, vertex.instance_index);
    
    out.uv = vertex.uv;
    out.binding_strength = strength;
    out.moment_count = moment_count;
    out.thread_distance = thread_distance;
    
    // Calculate energy pulse for fragment shader
    out.energy_pulse = pulse_factor * strength;
    
    return out;
}

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    // Base convergence color (purple/magenta)
    let base_color = vec3<f32>(0.9, 0.4, 0.9);
    
    // Add energy patterns based on UV coordinates
    let energy_pattern = sin(mesh.uv.x * 20.0 + material.time * 4.0) *
                        sin(mesh.uv.y * 20.0 + material.time * 3.0) * 0.5 + 0.5;
    
    // Create radial energy waves from center
    let center_distance = length(mesh.uv - vec2<f32>(0.5, 0.5));
    let radial_wave = sin(center_distance * 30.0 - material.time * 8.0) * 0.5 + 0.5;
    
    // Combine energy effects
    let energy_factor = (energy_pattern * 0.3 + radial_wave * 0.7) * mesh.binding_strength;
    
    // Intensify color based on moment count
    let intensity_boost = 1.0 + mesh.moment_count * 0.5;
    let final_color = base_color * intensity_boost * (1.0 + energy_factor);
    
    // Add white energy core
    let core_intensity = step(center_distance, 0.2) * mesh.energy_pulse;
    let core_color = vec3<f32>(1.0, 1.0, 1.0) * core_intensity;
    
    // Alpha based on binding strength and energy
    let alpha = mesh.binding_strength * (0.6 + mesh.energy_pulse * 0.4);
    
    return vec4<f32>(final_color + core_color, alpha);
} 
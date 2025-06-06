#import bevy_pbr::{
    mesh_functions,
    view_transformations::position_world_to_clip,
}

struct ThreadMaterial {
    base_color: vec4<f32>,
    time: f32,
    rope_texture_scale: f32,
}

@group(2) @binding(0) var<uniform> material: ThreadMaterial;

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
    @location(4) segment_index: f32,
    @location(5) binding_force: f32,
    @location(6) rope_pattern: f32,
}

// Decode thread segment data from MeshTag
// Bits: [decay_strength: 16][segment_index: 8][binding_force: 8]
fn decode_thread_data(tag: u32) -> vec3<f32> {
    let decay_strength = f32(tag & 0xFFFFu) / 65535.0;
    let segment_index = f32((tag >> 16u) & 0xFFu) / 255.0;
    let binding_force = f32((tag >> 24u) & 0xFFu) / 255.0;
    return vec3<f32>(decay_strength, segment_index, binding_force);
}

// Generate rope/fiber texture pattern
fn rope_pattern(uv: vec2<f32>, segment: f32, time: f32) -> f32 {
    // Create twisted rope effect
    let twist_speed = time * 0.5;
    let twist_offset = segment * 3.14159;
    let twisted_u = uv.x + sin(uv.y * 20.0 + twist_speed + twist_offset) * 0.1;
    
    // Fiber strands
    let strand1 = sin(twisted_u * 40.0) * 0.5 + 0.5;
    let strand2 = sin(twisted_u * 40.0 + 2.094) * 0.5 + 0.5; // Offset by 2π/3
    let strand3 = sin(twisted_u * 40.0 + 4.188) * 0.5 + 0.5; // Offset by 4π/3
    
    // Combine strands with varying intensity
    let pattern = (strand1 * 0.4 + strand2 * 0.3 + strand3 * 0.3);
    
    // Add subtle variation along length
    let length_variation = sin(uv.y * 8.0 + time * 0.3) * 0.1 + 0.9;
    
    return pattern * length_variation;
}

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;

    // Get the MeshTag data for this instance
    let tag = mesh_functions::get_tag(vertex.instance_index);
    let thread_data = decode_thread_data(tag);
    
    let decay_strength = thread_data.x;
    let segment_index = thread_data.y;
    let binding_force = thread_data.z;

    // Get world transform for this instance
    var world_from_local = mesh_functions::get_world_from_local(vertex.instance_index);
    
    // Apply binding convergence - pull segments toward binding points
    var position = vertex.position;
    if binding_force > 0.0 {
        // Create subtle convergence curve
        let convergence_factor = binding_force * 0.5;
        let curve_offset = sin(material.time * 2.0 + segment_index * 10.0) * convergence_factor;
        
        // Pull segments inward (toward center) when bindings are active
        position.x += curve_offset * 0.1;
        position.y += curve_offset * 0.05; // Less Y movement to maintain thread flow
    }
    
    // Apply gentle wave motion based on decay (like rope swaying)
    let wave_amplitude = decay_strength * 0.02;
    let wave_speed = 1.5;
    let wave_offset = sin(material.time * wave_speed + segment_index * 5.0) * wave_amplitude;
    position += vertex.normal * wave_offset;
    
    // Scale thickness based on decay strength
    let thickness_scale = 0.5 + decay_strength * 0.5;
    if vertex.normal.x != 0.0 || vertex.normal.z != 0.0 {
        // Scale only radial directions (not along thread length)
        position.x *= thickness_scale;
        position.z *= thickness_scale;
    }

    // Transform to world space
    out.world_position = mesh_functions::mesh_position_local_to_world(world_from_local, vec4(position, 1.0));
    out.clip_position = position_world_to_clip(out.world_position.xyz);
    
    // Transform normal to world space
    out.world_normal = mesh_functions::mesh_normal_local_to_world(vertex.normal, vertex.instance_index);
    
    // Adjust UV for rope texture
    out.uv = vertex.uv * material.rope_texture_scale;
    out.decay_strength = decay_strength;
    out.segment_index = segment_index;
    out.binding_force = binding_force;
    
    // Calculate rope pattern value
    out.rope_pattern = rope_pattern(out.uv, segment_index, material.time);
    
    return out;
}

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    // Base thread color (rope-like brown/amber)
    let base_color = vec3<f32>(0.8, 0.6, 0.4);
    
    // Apply rope fiber pattern
    let fiber_color = base_color * (0.7 + mesh.rope_pattern * 0.3);
    
    // Add binding glow effect
    let binding_glow = vec3<f32>(0.9, 0.4, 0.9) * mesh.binding_force * 0.3;
    
    // Add subtle pulsing for active bindings
    let pulse = sin(material.time * 3.0 + mesh.segment_index * 15.0) * 0.5 + 0.5;
    let binding_pulse = binding_glow * pulse * mesh.binding_force;
    
    // Combine colors
    let final_color = fiber_color + binding_glow + binding_pulse;
    
    // Add slight emissive glow based on decay
    let emissive = final_color * mesh.decay_strength * 0.2;
    
    // Alpha based on decay strength
    let alpha = mesh.decay_strength * 0.8 + 0.2; // Threads are more opaque than moments
    
    return vec4<f32>(final_color + emissive, alpha);
} 
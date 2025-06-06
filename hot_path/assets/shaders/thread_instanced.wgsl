#import bevy_pbr::{
    mesh_functions,
    view_transformations::position_world_to_clip,
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::alpha_discard,
}

@group(2) @binding(0) var<uniform> base_color: vec4<f32>;
@group(2) @binding(1) var<uniform> time: f32;
@group(2) @binding(2) var<uniform> rope_texture_scale: f32;
@group(2) @binding(3) var<uniform> emissive_strength: f32;

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

    // Decode thread data from tag
    let decay_strength = f32((tag >> 24u) & 0xFFu) / 255.0;
    let segment_index = (tag >> 16u) & 0xFFu;
    let binding_force = f32((tag >> 8u) & 0xFFu) / 255.0;

    // Transform vertex to world space
    out.world_position = mesh_functions::mesh_position_local_to_world(world_from_local, vec4(vertex.position, 1.0));
    out.clip_position = position_world_to_clip(out.world_position.xyz);
    out.world_normal = mesh_functions::mesh_normal_local_to_world(vertex.normal, vertex.instance_index);
    
    // Create rope-like UV mapping
    out.uv = vec2<f32>(
        vertex.uv.x * rope_texture_scale,
        vertex.uv.y + f32(segment_index) * 0.1 + time * 0.2 // Flowing texture
    );

    // Use base color from material with dynamic effects
    let binding_highlight = vec3<f32>(0.9, 0.7, 1.0);
    
    // Convergence effect when bindings are present
    let convergence_strength = binding_force * (0.5 + 0.5 * sin(time * 4.0));
    
    // Blend base color with binding effects and apply decay
    let final_rgb = mix(base_color.rgb, binding_highlight, convergence_strength) * max(decay_strength, 0.3);
    
    out.color = vec4<f32>(
        final_rgb,
        base_color.a * max(decay_strength, 0.6) // Maintain some visibility even when decayed
    );

    return out;
}

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    // Create rope fiber texture
    let fiber_u = fract(mesh.uv.x * 8.0);
    let fiber_v = fract(mesh.uv.y * 16.0);
    
    // Twisted rope pattern
    let twist = sin(mesh.uv.y * 20.0 + time) * 0.1;
    let fiber_pattern = sin((fiber_u + twist) * 6.28318) * 0.1 + 0.9;
    
    // Rope segments
    let segment_pattern = smoothstep(0.0, 0.1, fract(mesh.uv.y * 4.0)) * 
                         smoothstep(1.0, 0.9, fract(mesh.uv.y * 4.0));
    
    var final_color = mesh.color;
    let textured_rgb = final_color.rgb * fiber_pattern * (0.8 + 0.2 * segment_pattern);
    
    // Add emissive glow for better visibility
    let emissive_glow = textured_rgb * emissive_strength;
    final_color = vec4<f32>(
        textured_rgb + emissive_glow,
        final_color.a
    );
    
    return final_color;
} 
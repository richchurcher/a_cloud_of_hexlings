#import bevy_sprite::mesh2d_vertex_output::VertexOutput;
#import bevy_pbr::view_transformations::{frag_coord_to_ndc, position_world_to_ndc};

@group(1) @binding(0) var<uniform> light_radius: f32;
@group(1) @binding(1) var<uniform> player_position: vec3<f32>;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    var light_center = position_world_to_ndc(player_position);
    var mesh_pos = frag_coord_to_ndc(mesh.position);
    var d = distance(mesh_pos, light_center);

    if d < light_radius {
        // Transparent
        return vec4<f32>(0.0, 0.0, 0.0, 0.0);
    } else {
        // Obscured
        return vec4<f32>(0.0, 0.0, 0.0, 1.0);
    }
}

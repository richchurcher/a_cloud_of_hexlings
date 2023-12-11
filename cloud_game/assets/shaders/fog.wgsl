#import bevy_sprite::mesh2d_vertex_output::VertexOutput;

@group(2) @binding(0) var<uniform> light_radius: f32;
@group(2) @binding(1) var<uniform> player_position: vec2<f32>;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    var light_center = vec2<f32>(player_position.xy);
    var d = distance(mesh.position.xy, light_center);

    if d < light_radius {
        // Transparent
        return vec4<f32>(0.0, 0.0, 0.0, 0.0);
    } else {
        // Obscured
        return vec4<f32>(0.0, 0.0, 0.0, 1.0);
    }
}

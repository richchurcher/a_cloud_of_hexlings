
[[stage(vertex)]]
fn vs_main([[builtin(vertex_index)]] VertexIndex : u32) -> [[builtin(position)]] vec4<f32> {
    var pos = array<vec2<f32>, 6>(
        vec2<f32>(-1.0,  1.0),
        vec2<f32>(-1.0, -1.0),
        vec2<f32>( 1.0, -1.0),
        vec2<f32>(-1.0,  1.0),
        vec2<f32>( 1.0, -1.0),
        vec2<f32>( 1.0,  1.0)
    );
    return vec4<f32>(pos[VertexIndex], 0.0, 1.0);
}


[[group(0), binding(0)]]
var<uniform> playerPosition: vec2<f32>; // Player's position

[[group(0), binding(1)]]
var<uniform> lightRadius: f32; // Radius of the light circle


[[stage(fragment)]]
fn fs_main([[builtin(position)]] FragCoord: vec4<f32>) -> [[location(0)]] vec4<f32> {
    var lightCenter = vec2<f32>(playerPosition.x, playerPosition.y);

    // Calculate distance from current fragment to player position
    var distance = distance(FragCoord.xy, lightCenter);

    // Determine if the current fragment is within the light radius
    if (distance < lightRadius) {
        // Within the circle, render normally
        return vec4<f32>(1.0, 1.0, 1.0, 1.0); // Placeholder for normal rendering
    } else {
        // Outside the circle, apply fog of war
        return vec4<f32>(0.0, 0.0, 0.0, 1.0); // Render the fragment as black or darkened
    }
}

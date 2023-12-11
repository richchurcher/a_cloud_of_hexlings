#import bevy_sprite::mesh2d_vertex_output::VertexOutput;
#import bevy_pbr::{mesh_view_bindings::view, view_transformations::{frag_coord_to_ndc, position_world_to_ndc}};

@group(1) @binding(0) var<uniform> light_radius: f32;
@group(1) @binding(1) var<uniform> player_position: vec3<f32>;
// Yes, this is ridiculous.
@group(1) @binding(2) var<uniform> hexling_a: vec3<f32>;
@group(1) @binding(3) var<uniform> hexling_b: vec3<f32>;
@group(1) @binding(4) var<uniform> hexling_c: vec3<f32>;
@group(1) @binding(5) var<uniform> hexling_d: vec3<f32>;
@group(1) @binding(6) var<uniform> hexling_e: vec3<f32>;
@group(1) @binding(7) var<uniform> hexling_f: vec3<f32>;
@group(1) @binding(8) var<uniform> hexling_g: vec3<f32>;
@group(1) @binding(9) var<uniform> hexling_h: vec3<f32>;
@group(1) @binding(10) var<uniform> hexling_i: vec3<f32>;
@group(1) @binding(11) var<uniform> hexling_j: vec3<f32>;
@group(1) @binding(12) var<uniform> hexling_k: vec3<f32>;
@group(1) @binding(13) var<uniform> hexling_l: vec3<f32>;
@group(1) @binding(14) var<uniform> hexling_m: vec3<f32>;
@group(1) @binding(15) var<uniform> hexling_n: vec3<f32>;
@group(1) @binding(16) var<uniform> hexling_o: vec3<f32>;
@group(1) @binding(17) var<uniform> hexling_p: vec3<f32>;
@group(1) @binding(18) var<uniform> hexling_q: vec3<f32>;
@group(1) @binding(19) var<uniform> hexling_r: vec3<f32>;
@group(1) @binding(20) var<uniform> hexling_s: vec3<f32>;
@group(1) @binding(21) var<uniform> hexling_t: vec3<f32>;
@group(1) @binding(22) var<uniform> hexling_u: vec3<f32>;
@group(1) @binding(23) var<uniform> hexling_v: vec3<f32>;
@group(1) @binding(24) var<uniform> hexling_w: vec3<f32>;
@group(1) @binding(25) var<uniform> hexling_x: vec3<f32>;
@group(1) @binding(26) var<uniform> hexling_y: vec3<f32>;
@group(1) @binding(27) var<uniform> hexling_z: vec3<f32>;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    var light_center = position_world_to_ndc(player_position).xy * view.viewport.zw + view.viewport.xz;
    var mesh_pos = frag_coord_to_ndc(mesh.position).xy * view.viewport.zw + view.viewport.xz;
    var d = distance(mesh_pos, light_center);

    if d < light_radius {
        // Transparent
        return vec4<f32>(0.0, 0.0, 0.0, 0.0);
    } else {
        // Add transparency for all hexling positions
        var hex_pos = position_world_to_ndc(hexling_a).xy * view.viewport.zw + view.viewport.xz;
        var hd = distance(mesh_pos, hex_pos);
        if hd < (light_radius / 1.5) {
            return vec4<f32>(0.0, 0.0, 0.0, 0.0);
        }
        hex_pos = position_world_to_ndc(hexling_b).xy * view.viewport.zw + view.viewport.xz;
        hd = distance(mesh_pos, hex_pos);
        if hd < (light_radius / 1.5) {
            return vec4<f32>(0.0, 0.0, 0.0, 0.0);
        }
        hex_pos = position_world_to_ndc(hexling_c).xy * view.viewport.zw + view.viewport.xz;
        hd = distance(mesh_pos, hex_pos);
        if hd < (light_radius / 1.5) {
            return vec4<f32>(0.0, 0.0, 0.0, 0.0);
        }
        hex_pos = position_world_to_ndc(hexling_d).xy * view.viewport.zw + view.viewport.xz;
        hd = distance(mesh_pos, hex_pos);
        if hd < (light_radius / 1.5) {
            return vec4<f32>(0.0, 0.0, 0.0, 0.0);
        }
        hex_pos = position_world_to_ndc(hexling_e).xy * view.viewport.zw + view.viewport.xz;
        hd = distance(mesh_pos, hex_pos);
        if hd < (light_radius / 1.5) {
            return vec4<f32>(0.0, 0.0, 0.0, 0.0);
        }
        hex_pos = position_world_to_ndc(hexling_f).xy * view.viewport.zw + view.viewport.xz;
        hd = distance(mesh_pos, hex_pos);
        if hd < (light_radius / 1.5) {
            return vec4<f32>(0.0, 0.0, 0.0, 0.0);
        }
        hex_pos = position_world_to_ndc(hexling_g).xy * view.viewport.zw + view.viewport.xz;
        hd = distance(mesh_pos, hex_pos);
        if hd < (light_radius / 1.5) {
            return vec4<f32>(0.0, 0.0, 0.0, 0.0);
        }
        hex_pos = position_world_to_ndc(hexling_h).xy * view.viewport.zw + view.viewport.xz;
        hd = distance(mesh_pos, hex_pos);
        if hd < (light_radius / 1.5) {
            return vec4<f32>(0.0, 0.0, 0.0, 0.0);
        }
        hex_pos = position_world_to_ndc(hexling_i).xy * view.viewport.zw + view.viewport.xz;
        hd = distance(mesh_pos, hex_pos);
        if hd < (light_radius / 1.5) {
            return vec4<f32>(0.0, 0.0, 0.0, 0.0);
        }
        hex_pos = position_world_to_ndc(hexling_j).xy * view.viewport.zw + view.viewport.xz;
        hd = distance(mesh_pos, hex_pos);
        if hd < (light_radius / 1.5) {
            return vec4<f32>(0.0, 0.0, 0.0, 0.0);
        }
        hex_pos = position_world_to_ndc(hexling_k).xy * view.viewport.zw + view.viewport.xz;
        hd = distance(mesh_pos, hex_pos);
        if hd < (light_radius / 1.5) {
            return vec4<f32>(0.0, 0.0, 0.0, 0.0);
        }
        hex_pos = position_world_to_ndc(hexling_l).xy * view.viewport.zw + view.viewport.xz;
        hd = distance(mesh_pos, hex_pos);
        if hd < (light_radius / 1.5) {
            return vec4<f32>(0.0, 0.0, 0.0, 0.0);
        }
        hex_pos = position_world_to_ndc(hexling_m).xy * view.viewport.zw + view.viewport.xz;
        hd = distance(mesh_pos, hex_pos);
        if hd < (light_radius / 1.5) {
            return vec4<f32>(0.0, 0.0, 0.0, 0.0);
        }
        hex_pos = position_world_to_ndc(hexling_n).xy * view.viewport.zw + view.viewport.xz;
        hd = distance(mesh_pos, hex_pos);
        if hd < (light_radius / 1.5) {
            return vec4<f32>(0.0, 0.0, 0.0, 0.0);
        }
        hex_pos = position_world_to_ndc(hexling_o).xy * view.viewport.zw + view.viewport.xz;
        hd = distance(mesh_pos, hex_pos);
        if hd < (light_radius / 1.5) {
            return vec4<f32>(0.0, 0.0, 0.0, 0.0);
        }
        hex_pos = position_world_to_ndc(hexling_p).xy * view.viewport.zw + view.viewport.xz;
        hd = distance(mesh_pos, hex_pos);
        if hd < (light_radius / 1.5) {
            return vec4<f32>(0.0, 0.0, 0.0, 0.0);
        }
        hex_pos = position_world_to_ndc(hexling_q).xy * view.viewport.zw + view.viewport.xz;
        hd = distance(mesh_pos, hex_pos);
        if hd < (light_radius / 1.5) {
            return vec4<f32>(0.0, 0.0, 0.0, 0.0);
        }
        hex_pos = position_world_to_ndc(hexling_r).xy * view.viewport.zw + view.viewport.xz;
        hd = distance(mesh_pos, hex_pos);
        if hd < (light_radius / 1.5) {
            return vec4<f32>(0.0, 0.0, 0.0, 0.0);
        }
        hex_pos = position_world_to_ndc(hexling_s).xy * view.viewport.zw + view.viewport.xz;
        hd = distance(mesh_pos, hex_pos);
        if hd < (light_radius / 1.5) {
            return vec4<f32>(0.0, 0.0, 0.0, 0.0);
        }
        hex_pos = position_world_to_ndc(hexling_t).xy * view.viewport.zw + view.viewport.xz;
        hd = distance(mesh_pos, hex_pos);
        if hd < (light_radius / 1.5) {
            return vec4<f32>(0.0, 0.0, 0.0, 0.0);
        }
        hex_pos = position_world_to_ndc(hexling_u).xy * view.viewport.zw + view.viewport.xz;
        hd = distance(mesh_pos, hex_pos);
        if hd < (light_radius / 1.5) {
            return vec4<f32>(0.0, 0.0, 0.0, 0.0);
        }
        hex_pos = position_world_to_ndc(hexling_v).xy * view.viewport.zw + view.viewport.xz;
        hd = distance(mesh_pos, hex_pos);
        if hd < (light_radius / 1.5) {
            return vec4<f32>(0.0, 0.0, 0.0, 0.0);
        }
        hex_pos = position_world_to_ndc(hexling_w).xy * view.viewport.zw + view.viewport.xz;
        hd = distance(mesh_pos, hex_pos);
        if hd < (light_radius / 1.5) {
            return vec4<f32>(0.0, 0.0, 0.0, 0.0);
        }
        hex_pos = position_world_to_ndc(hexling_x).xy * view.viewport.zw + view.viewport.xz;
        hd = distance(mesh_pos, hex_pos);
        if hd < (light_radius / 1.5) {
            return vec4<f32>(0.0, 0.0, 0.0, 0.0);
        }
        hex_pos = position_world_to_ndc(hexling_y).xy * view.viewport.zw + view.viewport.xz;
        hd = distance(mesh_pos, hex_pos);
        if hd < (light_radius / 1.5) {
            return vec4<f32>(0.0, 0.0, 0.0, 0.0);
        }
        hex_pos = position_world_to_ndc(hexling_z).xy * view.viewport.zw + view.viewport.xz;
        hd = distance(mesh_pos, hex_pos);
        if hd < (light_radius / 1.5) {
            return vec4<f32>(0.0, 0.0, 0.0, 0.0);
        }
    }

        // Obscured
    return vec4<f32>(0.0, 0.0, 0.0, 1.0);
}

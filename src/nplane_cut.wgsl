#import bevy_pbr::{

    mesh_view_bindings::view,
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::alpha_discard,
        // mesh::vertex,
}

#ifdef PREPASS_PIPELINE
#import bevy_pbr::{
    prepass_io::{VertexOutput, FragmentOutput},
    pbr_deferred_functions::deferred_output,
}
#else
#import bevy_pbr::{
    forward_io::{VertexOutput, FragmentOutput},
    pbr_functions::{apply_pbr_lighting, main_pass_post_lighting_processing},
}
#endif
struct NPlaneCutExt {
    plane: array<vec4<f32>, 6>,
    color: array<vec4<f32>, 6>,
    flags: u32,
    count: u32,
}
const PLANE_CUT_FLAGS_SCREENSPACE_BIT: u32 = 1u;
const PLANE_CUT_FLAGS_SHADED_BIT: u32 = 4u;

@group(2) @binding(101)
var<uniform> plane_cut_ext: NPlaneCutExt;

// @vertex
// fn my_vertex(vertex_no_morph: Vertex) -> VertexOutput {
//     return vertex(vertex_no_morph);
// }

fn intersect_plane_line(plane: vec4<f32>, v: vec3<f32>, r: vec3<f32>) -> f32 {
    let n = plane.xyz;
    let denom = dot(n, v);

    // Avoid division by zero — line is parallel to the plane
    if (abs(denom) < 1e-6) {
        let nan_val: f32 = f32(bitcast<u32>(0x7FC00000));
        return nan_val;
    }

    let t = (plane.w - dot(n, r)) / denom;
    return t;
}

@fragment
fn fragment(
    in_: VertexOutput,
    @builtin(front_facing) is_front: bool,
) -> FragmentOutput {
    var in = in_;

    let shaded = (plane_cut_ext.flags & PLANE_CUT_FLAGS_SHADED_BIT) != 0u;

    var plane_index: i32 = -1;
    var t_max: f32 = 0;
    var position: vec3<f32>;
    if ((plane_cut_ext.flags & PLANE_CUT_FLAGS_SCREENSPACE_BIT) != 0u) {
        // Screenspace
        position = in.position.xyz;
    } else {
        // World space
        position = in.world_position.xyz;
    }

    let view_ray = normalize(in.world_position.xyz - view.world_position);
    for (var i: u32 = 0u; i < plane_cut_ext.count; i++) {
        if dot(position, plane_cut_ext.plane[i].xyz) < plane_cut_ext.plane[i].w {
            discard;
        }
        // Make sure the view ray and the plane are anti-collinear
        if dot(view_ray, plane_cut_ext.plane[i].xyz) > 0 {
            let t = intersect_plane_line(plane_cut_ext.plane[i], view_ray, view.world_position);
            if t > t_max {
                plane_index = i32(i);
                t_max = t;
            }
        }
    }

    if (!is_front && shaded && plane_index >= 0) {
        // The in.world_position is not actually correct, but I don't see any
        // difference visually.
        //
        // We're drawing the backface, so it must be one of the planes, but we
        // don't know which yet.
        in.world_normal = -plane_cut_ext.plane[plane_index].xyz;
    }
    // Generate a PbrInput struct from the StandardMaterial bindings
    var pbr_input = pbr_input_from_standard_material(in, is_front);

    if (!is_front && shaded && plane_index >= 0) {
        pbr_input.material.base_color = plane_cut_ext.color[plane_index];
    }

    // Alpha discard
    pbr_input.material.base_color = alpha_discard(pbr_input.material, pbr_input.material.base_color);

    // Object space
    // XXX: Might have to do object space in the vertex shader.

#ifdef PREPASS_PIPELINE
    // In deferred mode we can't modify anything after that, as lighting is run
    // in a separate fullscreen shader.
    let out = deferred_output(in, pbr_input);
#else
    var out: FragmentOutput;
    // Apply lighting
    out.color = apply_pbr_lighting(pbr_input);

    out.color = main_pass_post_lighting_processing(pbr_input, out.color);
    if (!shaded && !is_front && plane_index >= 0) {
        out.color = plane_cut_ext.color[plane_index];
    }
#endif

    return out;
}

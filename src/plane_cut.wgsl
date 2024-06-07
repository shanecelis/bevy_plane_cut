#import bevy_pbr::{
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

struct PlaneCutExt {
    plane: vec4<f32>,
    color: vec4<f32>,
    flags: u32
}
const PLANE_CUT_FLAGS_SCREENSPACE_BIT: u32 = 1u;
const PLANE_CUT_FLAGS_SHADED_BIT: u32 = 4u;

@group(2) @binding(100)
var<uniform> plane_cut_ext: PlaneCutExt;

// @vertex
// fn my_vertex(vertex_no_morph: Vertex) -> VertexOutput {
//     return vertex(vertex_no_morph);
// }

@fragment
fn fragment(
    in_: VertexOutput,
    @builtin(front_facing) is_front: bool,
) -> FragmentOutput {
    var in = in_;

    let shaded = (plane_cut_ext.flags & PLANE_CUT_FLAGS_SHADED_BIT) != 0u;
    if !is_front && shaded {
            // The in.world_position is not actually correct, but I don't see any difference visually.
            in.world_normal = -plane_cut_ext.plane.xyz;
    }
    // generate a PbrInput struct from the StandardMaterial bindings
    var pbr_input = pbr_input_from_standard_material(in, is_front);

    if !is_front && shaded {
            pbr_input.material.base_color = plane_cut_ext.color;
    }

    // alpha discard
    pbr_input.material.base_color = alpha_discard(pbr_input.material, pbr_input.material.base_color);

#ifdef PREPASS_PIPELINE
    // in deferred mode we can't modify anything after that, as lighting is run in a separate fullscreen shader.
    let out = deferred_output(in, pbr_input);
#else
    var out: FragmentOutput;
    // apply lighting
    out.color = apply_pbr_lighting(pbr_input);

    if ((plane_cut_ext.flags & PLANE_CUT_FLAGS_SCREENSPACE_BIT) != 0u) {
        // Screenspace
        if (dot(in.position.xyz, plane_cut_ext.plane.xyz) < plane_cut_ext.plane.w) {
            discard;
        }
    } else {
        // World space
        if (dot(in.world_position.xyz, plane_cut_ext.plane.xyz) < plane_cut_ext.plane.w) {
            discard;
        }
    }
    // Object space
    // XXX: Might have to do object space in the vertex shader.
    out.color = main_pass_post_lighting_processing(pbr_input, out.color);
    if (!shaded && !is_front) {
        out.color = plane_cut_ext.color;
    }
#endif

    return out;
}

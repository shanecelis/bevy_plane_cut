#import bevy_pbr::{
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::alpha_discard,
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

struct DoublePlaneCutExt {
    plane1: vec4<f32>,
    color1: vec4<f32>,
    flags1: u32,
    plane2: vec4<f32>,
    color2: vec4<f32>,
    flags2: u32,
}

const PLANE_CUT_FLAGS_SCREENSPACE_BIT: u32 = 1u;
const PLANE_CUT_FLAGS_SHADED_BIT: u32 = 4u;

@group(2) @binding(100)
var<uniform> double_plane_cut_ext: DoublePlaneCutExt;

@fragment
fn fragment(
    in_: VertexOutput,
    @builtin(front_facing) is_front: bool,
) -> FragmentOutput {
    var in = in_;

    let shaded1 = (double_plane_cut_ext.flags1 & PLANE_CUT_FLAGS_SHADED_BIT) != 0u;
    let shaded2 = (double_plane_cut_ext.flags2 & PLANE_CUT_FLAGS_SHADED_BIT) != 0u;
    
    // Check which plane cut we're on (if any)
    var cut_by_plane1 = false;
    var cut_by_plane2 = false;
    
    if ((double_plane_cut_ext.flags1 & PLANE_CUT_FLAGS_SCREENSPACE_BIT) != 0u) {
        // Screenspace for plane 1
        if (dot(in.position.xyz, double_plane_cut_ext.plane1.xyz) < double_plane_cut_ext.plane1.w) {
            cut_by_plane1 = true;
        }
    } else {
        // World space for plane 1
        if (dot(in.world_position.xyz, double_plane_cut_ext.plane1.xyz) < double_plane_cut_ext.plane1.w) {
            cut_by_plane1 = true;
        }
    }
    
    if ((double_plane_cut_ext.flags2 & PLANE_CUT_FLAGS_SCREENSPACE_BIT) != 0u) {
        // Screenspace for plane 2
        if (dot(in.position.xyz, double_plane_cut_ext.plane2.xyz) < double_plane_cut_ext.plane2.w) {
            cut_by_plane2 = true;
        }
    } else {
        // World space for plane 2
        if (dot(in.world_position.xyz, double_plane_cut_ext.plane2.xyz) < double_plane_cut_ext.plane2.w) {
            cut_by_plane2 = true;
        }
    }
    
    // Discard if cut by either plane
    if (cut_by_plane1 || cut_by_plane2) {
        discard;
    }

    // Determine which cut surface we're on
    var use_plane1_color = false;
    var use_plane2_color = false;
    
    if (!is_front) {
        if (shaded1) {
            // Check if we're on the cut surface of plane 1
            let distance1 = abs(dot(in.world_position.xyz, double_plane_cut_ext.plane1.xyz) - double_plane_cut_ext.plane1.w);
            let distance2 = abs(dot(in.world_position.xyz, double_plane_cut_ext.plane2.xyz) - double_plane_cut_ext.plane2.w);
            
            // Use the color of the closest plane
            if (distance1 < distance2) {
                in.world_normal = -double_plane_cut_ext.plane1.xyz;
                use_plane1_color = true;
            } else {
                in.world_normal = -double_plane_cut_ext.plane2.xyz;
                use_plane2_color = true;
            }
        } else if (shaded2) {
            // Similar logic but prefer plane 2
            let distance1 = abs(dot(in.world_position.xyz, double_plane_cut_ext.plane1.xyz) - double_plane_cut_ext.plane1.w);
            let distance2 = abs(dot(in.world_position.xyz, double_plane_cut_ext.plane2.xyz) - double_plane_cut_ext.plane2.w);
            
            if (distance2 <= distance1) {
                in.world_normal = -double_plane_cut_ext.plane2.xyz;
                use_plane2_color = true;
            } else {
                in.world_normal = -double_plane_cut_ext.plane1.xyz;
                use_plane1_color = true;
            }
        }
    }

    // Generate a PbrInput struct from the StandardMaterial bindings
    var pbr_input = pbr_input_from_standard_material(in, is_front);

    if (!is_front) {
        if (use_plane1_color && shaded1) {
            pbr_input.material.base_color = double_plane_cut_ext.color1;
        } else if (use_plane2_color && shaded2) {
            pbr_input.material.base_color = double_plane_cut_ext.color2;
        }
    }

    // Alpha discard
    pbr_input.material.base_color = alpha_discard(pbr_input.material, pbr_input.material.base_color);

#ifdef PREPASS_PIPELINE
    // In deferred mode we can't modify anything after that, as lighting is run
    // in a separate fullscreen shader.
    let out = deferred_output(in, pbr_input);
#else
    var out: FragmentOutput;
    // Apply lighting
    out.color = apply_pbr_lighting(pbr_input);

    out.color = main_pass_post_lighting_processing(pbr_input, out.color);
    
    if (!is_front) {
        if (use_plane1_color && !shaded1) {
            out.color = double_plane_cut_ext.color1;
        } else if (use_plane2_color && !shaded2) {
            out.color = double_plane_cut_ext.color2;
        }
    }
#endif

    return out;
}

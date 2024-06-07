//! Demonstrates using a custom extension to the `StandardMaterial` to modify the results of the builtin pbr shader.

use bevy::{
    asset::embedded_asset,
    pbr::{ExtendedMaterial, MaterialExtension, OpaqueRendererMethod},
    prelude::*,
    render::render_resource::*,
};

pub struct PlaneCutPlugin;

impl Plugin for PlaneCutPlugin {

    fn build(&self, app: &mut App) {
        embedded_asset!(app, "plane_cut.wgsl");
        app
        .add_plugins(MaterialPlugin::<
            ExtendedMaterial<StandardMaterial, PlaneCutExt>,
        >::default());
    }
}

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
pub struct PlaneCutExt {
    // We need to ensure that the bindings of the base material and the extension do not conflict,
    // so we start from binding slot 100, leaving slots 0-99 for the base material.
    #[uniform(100)]
    // plane: Vec4,
    pub quantize_steps: u32,
}

impl MaterialExtension for PlaneCutExt {
    fn fragment_shader() -> ShaderRef {
        "embedded://bevy_plane_cut/plane_cut.wgsl".into()
    }

    fn deferred_fragment_shader() -> ShaderRef {
        "embedded://bevy_plane_cut/plane_cut.wgsl".into()
    }
}

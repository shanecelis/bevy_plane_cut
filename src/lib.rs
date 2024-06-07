//! Demonstrates using a custom extension to the `StandardMaterial` to modify the results of the builtin pbr shader.

use bevy::{
    reflect::Reflect,
    utils::Hashed,
    asset::embedded_asset,
    pbr::{ExtendedMaterial, MaterialExtension, MaterialExtensionPipeline, MaterialExtensionKey},
    prelude::*,
    render::{mesh::InnerMeshVertexBufferLayout, render_resource::*, render_asset::RenderAssets},
};

pub struct PlaneCutPlugin;
pub type PlaneCutMaterial = ExtendedMaterial<StandardMaterial, PlaneCutExt>;

impl Plugin for PlaneCutPlugin {

    fn build(&self, app: &mut App) {
        embedded_asset!(app, "plane_cut.wgsl");
        app
        .add_plugins(MaterialPlugin::<
            ExtendedMaterial<StandardMaterial, PlaneCutExt>,
        >::default());
    }
}
#[derive(Default, Reflect, Debug, Clone)]
pub enum Space {
    #[default]
    World,
    Screen,
    //Model
}

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]

#[uniform(100, PlaneCutExtUniform)]
pub struct PlaneCutExt {
    // We need to ensure that the bindings of the base material and the extension do not conflict,
    // so we start from binding slot 100, leaving slots 0-99 for the base material.
    pub plane: Vec4,
    pub color: Color,
    pub space: Space,
    pub shaded: bool,
}

impl Default for PlaneCutExt {
    fn default() -> Self {
        Self {
            plane: Vec4::new(1.0, 0.0, 0.0, 0.0),
            color: Color::BLACK,
            space: Space::default(),
            shaded: true,
        }
    }
}

/// The GPU representation of the uniform data of a [`PlaneCutExt`].
#[derive(Clone, Default, ShaderType)]
pub struct PlaneCutExtUniform {
    pub plane: Vec4,
    pub color: Vec4,
    pub flags: u32,
}

impl AsBindGroupShaderType<PlaneCutExtUniform> for PlaneCutExt {
    fn as_bind_group_shader_type(&self, _images: &RenderAssets<Image>) -> PlaneCutExtUniform {
        let mut flags = 0;
        if matches!(self.space, Space::Screen) {
            flags |= 1;
        }
        if self.shaded {
            flags |= 4;
        }
        PlaneCutExtUniform {
            plane: self.plane,

            color: self.color.as_linear_rgba_f32().into(),
            flags
        }
    }
}

impl MaterialExtension for PlaneCutExt {
    fn fragment_shader() -> ShaderRef {
        "embedded://bevy_plane_cut/plane_cut.wgsl".into()
    }

    fn deferred_fragment_shader() -> ShaderRef {
        "embedded://bevy_plane_cut/plane_cut.wgsl".into()
    }

    fn specialize(_pipeline: &MaterialExtensionPipeline,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &Hashed<InnerMeshVertexBufferLayout>,
        _key: MaterialExtensionKey<Self>
    ) -> Result<(), SpecializedMeshPipelineError> {
        descriptor.primitive.cull_mode = None;
        Ok(())
    }

}

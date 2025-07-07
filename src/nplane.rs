use bevy::{
    app::{App},
    asset::{embedded_asset, Asset},
    math::Vec4,
    pbr::{
        ExtendedMaterial, MaterialExtension, MaterialExtensionKey, MaterialExtensionPipeline,
        MaterialPlugin, StandardMaterial,
    },
    color::{Color, LinearRgba, ColorToComponents},
    reflect::Reflect,
    render::{
        mesh::MeshVertexBufferLayoutRef,
        render_asset::RenderAssets,
        render_resource::{
            AsBindGroup, AsBindGroupShaderType, RenderPipelineDescriptor, ShaderRef, ShaderType,
            SpecializedMeshPipelineError,
        },
        texture::{GpuImage },
    },
};
use super::*;

/// The maximum number of plane cuts.
pub const PLANE_MAX: usize = 6;

pub(crate) fn plugin(app: &mut App) {
    embedded_asset!(app, "nplane_cut.wgsl");
    app.add_plugins(MaterialPlugin::<NPlaneCutMaterial>::default());
}

/// Type alias for two plane cut material.
pub type NPlaneCutMaterial = ExtendedMaterial<StandardMaterial, NPlaneCutExt>;

/// The plane cut extension.
#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
#[uniform(101, NPlaneCutExtUniform)]
pub struct NPlaneCutExt {
    /// The plane is defined with a normal vector _n_ and displacment scalar
    /// _w_, represented with a vector _(nx, ny, nz, w)_. Its equation is _n .
    /// position = w_. The portion that is cut is _n . position < w_.
    ///
    /// Each vector has an associated color of its cut.
    ///
    /// Respects a maximum of `PLANE_MAX`.
    pub planes_and_colors: Vec<(Vec4, Color)>,
    /// Define the space the plane is tested in.
    pub space: Space,
    /// Is the cut shaded or unlit? Shaded is the default. Note: using the
    /// deferred renderer will not respect an unlit option.
    pub shaded: bool,
}

impl Default for NPlaneCutExt {
    fn default() -> Self {
        Self {
            planes_and_colors: vec![(Vec4::new(1.0, 0.0, 0.0, 0.0), Color::BLACK)],
            space: Space::default(),
            shaded: true,
        }
    }
}

/// The GPU representation of the uniform data of a [`PlaneCutExt`].
#[derive(Clone, Default, ShaderType)]
struct NPlaneCutExtUniform {
    planes: [Vec4; PLANE_MAX],
    colors: [Vec4; PLANE_MAX],
    flags: u32,
    count: u32,
}

impl AsBindGroupShaderType<NPlaneCutExtUniform> for NPlaneCutExt {
    fn as_bind_group_shader_type(&self, _images: &RenderAssets<GpuImage>) -> NPlaneCutExtUniform {
        let mut flags = 0;
        if matches!(self.space, Space::Screen) {
            flags |= 1;
        }
        if self.shaded {
            flags |= 4;
        }
        let mut planes = [Vec4::ZERO; PLANE_MAX];
        let mut colors = [Vec4::ZERO; PLANE_MAX];
        for (i, (v, c)) in self.planes_and_colors.iter().enumerate() {
            planes[i] = *v;
            colors[i] = LinearRgba::from(*c).to_f32_array().into();
        }
        NPlaneCutExtUniform {
            planes,
            colors,
            flags,
            count: self.planes_and_colors.len() as u32,
        }
    }
}

impl MaterialExtension for NPlaneCutExt {
    fn fragment_shader() -> ShaderRef {
        "embedded://bevy_plane_cut/nplane_cut.wgsl".into()
    }

    fn deferred_fragment_shader() -> ShaderRef {
        "embedded://bevy_plane_cut/nplane_cut.wgsl".into()
    }

    fn specialize(
        _pipeline: &MaterialExtensionPipeline,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayoutRef,
        _key: MaterialExtensionKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        descriptor.primitive.cull_mode = None;
        Ok(())
    }
}

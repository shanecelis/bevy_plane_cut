#![doc(html_root_url = "https://docs.rs/bevy_plane_cut/0.2.0")]
#![doc = include_str!("../README.md")]
#![forbid(missing_docs)]

use bevy::{
    app::{App, Plugin},
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

/// Type alias for `ExtendedMaterial<StandardMaterial, PlaneCutExt>`.
pub type PlaneCutMaterial = ExtendedMaterial<StandardMaterial, PlaneCutExt>;

/// The plane cut plugin.
pub struct PlaneCutPlugin;

impl Plugin for PlaneCutPlugin {
    fn build(&self, app: &mut App) {
        embedded_asset!(app, "plane_cut.wgsl");
        app.add_plugins(MaterialPlugin::<PlaneCutMaterial>::default());
    }
}

/// Define what space to test the plane cut in: world space (default) or screen space.
///
/// TODO: Consider adding object/model space as an option.
#[derive(Default, Reflect, Debug, Clone)]
pub enum Space {
    /// Run plane cut in world space (default).
    #[default]
    World,
    /// Run plane cut in screen space. This turns the plane into more of a line cut.
    Screen,
    //Model
}

/// The plane cut extension.
#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
#[uniform(100, PlaneCutExtUniform)]
pub struct PlaneCutExt {
    /// The planes are defined with a normal vector _n_ and displacment scalar
    /// _w_, represented with a vector _(nx, ny, nz, w)_. Its equation is _n .
    /// position = w_. The portion that is cut is _n . position < w_.
    /// For a unit cube centered at the origin, the planes are:
    /// - Right: normal (-1,0,0), w = -0.5 (cuts x > 0.5)
    /// - Left: normal (1,0,0), w = -0.5 (cuts x < -0.5)
    /// - Top: normal (0,-1,0), w = -0.5 (cuts y > 0.5)
    /// - Bottom: normal (0,1,0), w = -0.5 (cuts y < -0.5)
    /// - Front: normal (0,0,-1), w = -0.5 (cuts z > 0.5)
    /// - Back: normal (0,0,1), w = -0.5 (cuts z < -0.5)
    pub planes: [Vec4; 6],
    /// Define the color of the cut.
    pub color: Color,
    /// Define the space the plane is tested in.
    pub space: Space,
    /// Is the cut shaded or unlit? Shaded is the default. Note: using the
    /// deferred renderer will not respect an unlit option.
    pub shaded: bool,
}

impl Default for PlaneCutExt {
    fn default() -> Self {
        Self {
            planes: [
                Vec4::new(-1.0, 0.0, 0.0, -0.5), // Cuts x > 0.5 (Right)
                Vec4::new(1.0, 0.0, 0.0, -0.5),  // Cuts x < -0.5 (Left)
                Vec4::new(0.0, -1.0, 0.0, -0.5), // Cuts y > 0.5 (Top)
                Vec4::new(0.0, 1.0, 0.0, -0.5),  // Cuts y < -0.5 (Bottom)
                Vec4::new(0.0, 0.0, -1.0, -0.5), // Cuts z > 0.5 (Front)
                Vec4::new(0.0, 0.0, 1.0, -0.5),  // Cuts z < -0.5 (Back)
            ],
            color: Color::BLACK,
            space: Space::default(),
            shaded: true,
        }
    }
}

/// The GPU representation of the uniform data of a [`PlaneCutExt`].
#[derive(Clone, Default, ShaderType)]
struct PlaneCutExtUniform {
    planes: [Vec4; 6],
    color: Vec4,
    flags: u32,
}

impl AsBindGroupShaderType<PlaneCutExtUniform> for PlaneCutExt {
    fn as_bind_group_shader_type(&self, _images: &RenderAssets<GpuImage>) -> PlaneCutExtUniform {
        let mut flags = 0;
        if matches!(self.space, Space::Screen) {
            flags |= 1;
        }
        if self.shaded {
            flags |= 4;
        }
        PlaneCutExtUniform {
            planes: self.planes,

            color: LinearRgba::from(self.color).to_f32_array().into(),
            flags,
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

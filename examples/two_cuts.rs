//! Demonstrates two plane cuts with a nested `ExtendedMaterial`. NOT WORKING!
//!
//! Happy to have some eyes on this to see how to fix it. The error reported
//! is this:
//!
//! ```text
//! ❯ cargo run --example two_cuts --features bevy/embedded_watcher
//!    Compiling bevy_plane_cut v0.1.0 (/Users/shane/Projects/bevy_plane_cut)
//!     Finished `dev` profile [unoptimized + debuginfo] target(s) in 4.68s
//!      Running `target/debug/examples/two_cuts`
//! 2024-06-07T22:30:25.701348Z  INFO bevy_render::renderer: AdapterInfo { name: "AMD Radeon Pro Vega 64", vendor: 0, device: 0, device_type: DiscreteGpu, driver: "", driver_info: "", backend: Metal }
//! 2024-06-07T22:30:26.387124Z ERROR log: Handling wgpu errors as fatal by default
//! thread 'main' panicked at /Users/shane/.cargo/registry/src/index.crates.io-6f17d22bba15001f/wgpu-0.19.4/src/backend/wgpu_core.rs:3006:5:
//! wgpu error: Validation Error
//!
//! Caused by:
//!     In Device::create_bind_group_layout
//!     Conflicting binding at index 100
//!
//! note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
//! ```

use bevy::{
    pbr::{ExtendedMaterial, OpaqueRendererMethod, MaterialExtension},
    prelude::*,
    color::palettes::basic,
    asset::Asset,
    render::render_resource::{AsBindGroup, AsBindGroupShaderType, ShaderRef, ShaderType},
    render::render_asset::RenderAssets,
    render::texture::GpuImage,
    color::{LinearRgba, ColorToComponents},
};

use bevy_plane_cut::{PlaneCutPlugin, Space};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PlaneCutPlugin)
        .add_plugins(MaterialPlugin::<DoublePlaneCutMaterial>::default())
        .add_systems(Startup, setup)
        .add_systems(Update, (rotate_things, translate_things, update_plane))
        .run();
}

type DoublePlaneCutMaterial = ExtendedMaterial<StandardMaterial, DoublePlaneCutExt>;
#[derive(Component)]
struct Plane(Handle<DoublePlaneCutMaterial>);

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<DoublePlaneCutMaterial>>,
) {
    let handle = materials.add(ExtendedMaterial {
        base: StandardMaterial {
            base_color: basic::RED.into(),
            opaque_render_method: OpaqueRendererMethod::Forward,
            ..Default::default()
        },
        extension: DoublePlaneCutExt {
            plane1: Vec4::new(1.0, 0.0, 0.0, 0.5),  // Cut from the right side, offset
            color1: Color::linear_rgb(1.0, 0.0, 0.0), // Red color for first cut
            space1: Space::World,
            shaded1: false, // Make it unlit so it's more visible
            plane2: Vec4::new(0.0, 1.0, 0.0, 0.2),  // Cut from the top, different offset
            color2: Color::linear_rgb(0.0, 0.0, 1.0), // Blue color for second cut  
            space2: Space::World,
            shaded2: false, // Make it unlit so it's more visible
        },
    });
    commands.spawn((
        Transform::default(),
        Plane(handle.clone()),
        // Rotate(Vec3::new(1.0, 1.0, 0.0))
        Translate(Vec3::new(1.0, 0.0, 0.0)),
    ));
    // sphere
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(1.0))),
        MeshMaterial3d(handle),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));

    // light
    commands.spawn((
        DirectionalLight::default(),
        Transform::from_xyz(1.0, 1.0, 1.0).looking_at(Vec3::ZERO, Vec3::Y),
        Rotate(Dir3::Y),
    ));

    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn update_plane(
    q: Query<(&GlobalTransform, &Plane)>,
    mut materials: ResMut<Assets<DoublePlaneCutMaterial>>,
) {
    for (t, p) in &q {
        let Some(m) = materials.get_mut(&p.0) else {
            continue;
        };
        trace!("Updating plane");
        let normal = t.left();
        let w = normal.dot(t.translation());
        m.extension.plane2 = (*normal, w).into();
    }
}

#[derive(Component)]
struct Translate(Vec3);

#[derive(Component)]
struct Rotate(Dir3);

fn rotate_things(mut q: Query<(&mut Transform, &Rotate)>, time: Res<Time>) {
    for (mut t, r) in &mut q {
        t.rotate_axis(r.0, time.delta_secs());
    }
}

fn translate_things(mut q: Query<(&mut Transform, &Translate)>, time: Res<Time>) {
    for (mut t, r) in &mut q {
        t.translation = (time.elapsed_secs() / 1.0).sin().abs() * r.0;
    }
}

/// Material extension that supports two plane cuts
#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
#[uniform(100, DoublePlaneCutExtUniform)]
pub struct DoublePlaneCutExt {
    /// The first plane cut
    pub plane1: Vec4,
    /// Color for the first cut
    pub color1: Color,
    /// Space for the first cut
    pub space1: Space,
    /// Is the first cut shaded or unlit?
    pub shaded1: bool,
    /// The second plane cut
    pub plane2: Vec4,
    /// Color for the second cut
    pub color2: Color,
    /// Space for the second cut
    pub space2: Space,
    /// Is the second cut shaded or unlit?
    pub shaded2: bool,
}

impl Default for DoublePlaneCutExt {
    fn default() -> Self {
        Self {
            plane1: Vec4::new(1.0, 0.0, 0.0, 0.0),
            color1: Color::BLACK,
            space1: Space::default(),
            shaded1: true,
            plane2: Vec4::new(0.0, 1.0, 0.0, 0.0),
            color2: Color::BLACK,
            space2: Space::default(),
            shaded2: true,
        }
    }
}

/// The GPU representation of the uniform data for DoublePlaneCutExt
#[derive(Clone, Default, ShaderType)]
struct DoublePlaneCutExtUniform {
    plane1: Vec4,
    color1: Vec4,
    flags1: u32,
    plane2: Vec4,
    color2: Vec4,
    flags2: u32,
}

impl AsBindGroupShaderType<DoublePlaneCutExtUniform> for DoublePlaneCutExt {
    fn as_bind_group_shader_type(&self, _images: &RenderAssets<GpuImage>) -> DoublePlaneCutExtUniform {
        let mut flags1 = 0;
        if matches!(self.space1, Space::Screen) {
            flags1 |= 1;
        }
        if self.shaded1 {
            flags1 |= 4;
        }

        let mut flags2 = 0;
        if matches!(self.space2, Space::Screen) {
            flags2 |= 1;
        }
        if self.shaded2 {
            flags2 |= 4;
        }

        DoublePlaneCutExtUniform {
            plane1: self.plane1,
            color1: LinearRgba::from(self.color1).to_f32_array().into(),
            flags1,
            plane2: self.plane2,
            color2: LinearRgba::from(self.color2).to_f32_array().into(),
            flags2,
        }
    }
}

impl MaterialExtension for DoublePlaneCutExt {
    fn fragment_shader() -> ShaderRef {
        "embedded://bevy_plane_cut/double_plane_cut.wgsl".into()
    }

    fn deferred_fragment_shader() -> ShaderRef {
        "embedded://bevy_plane_cut/double_plane_cut.wgsl".into()
    }

    fn specialize(
        _pipeline: &bevy::pbr::MaterialExtensionPipeline,
        descriptor: &mut bevy::render::render_resource::RenderPipelineDescriptor,
        _layout: &bevy::render::mesh::MeshVertexBufferLayoutRef,
        _key: bevy::pbr::MaterialExtensionKey<Self>,
    ) -> Result<(), bevy::render::render_resource::SpecializedMeshPipelineError> {
        descriptor.primitive.cull_mode = None;
        Ok(())
    }
}

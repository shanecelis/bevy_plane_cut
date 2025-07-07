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

use bevy_plane_cut::{nplane::{NPlaneCutMaterial, NPlaneCutExt}, PlaneCutPlugin, Space};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PlaneCutPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (rotate_things, translate_things, update_plane))
        .run();
}

#[derive(Component)]
struct Plane(Handle<NPlaneCutMaterial>);

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<NPlaneCutMaterial>>,
) {
    let handle = materials.add(ExtendedMaterial {
        base: StandardMaterial {
            base_color: basic::RED.into(),
            opaque_render_method: OpaqueRendererMethod::Forward,
            ..Default::default()
        },
        extension: NPlaneCutExt {
            planes_and_colors: vec![
                (Vec4::new(1.0, 0.0, 0.0, 0.1),  // Cut from the right side, offset
                 Color::linear_rgb(1.0, 0.0, 0.0)), // Red color for first cut
                (Vec4::new(0.0, 1.0, 0.0, 0.2),  // Cut from the top, different offset
                 Color::linear_rgb(0.0, 0.0, 1.0))], // Blue color for second cut
            space: Space::World,
            shaded: false, // Make it unlit so it's more visible
        },
    });
    commands.spawn((
        Transform::default(),
        Plane(handle.clone()),
        // Rotate(Dir3::from_xyz(1.0, 0.0, 0.0).unwrap()),
        // Translate(Vec3::new(1.0, 0.0, 0.0)),
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
    mut materials: ResMut<Assets<NPlaneCutMaterial>>,
) {
    for (t, p) in &q {
        let Some(m) = materials.get_mut(&p.0) else {
            continue;
        };
        trace!("Updating plane");
        // let normal = t.left();
        // let w = normal.dot(t.translation());
        let w = m.extension.planes_and_colors[0].0.w;
        m.extension.planes_and_colors[0].0 = (t.left().as_vec3(), w).into();
        let w = m.extension.planes_and_colors[1].0.w;
        m.extension.planes_and_colors[1].0 = (t.up().as_vec3(), w).into();
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


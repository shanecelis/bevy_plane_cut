//! Demonstrates a plane cut using the deferred renderer.
//!
//! This one looks weird. But I'm not sure why. The `deferred` example that
//! doesn't have any plane cut also looks weird.

use bevy::{
    core_pipeline::prepass::DeferredPrepass,
    pbr::{ExtendedMaterial, OpaqueRendererMethod},
    prelude::*,
};

use bevy_plane_cut::{PlaneCutExt, PlaneCutMaterial, PlaneCutPlugin, Space};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PlaneCutPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, rotate_things)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<PlaneCutMaterial>>,
) {
    let handle = materials.add(ExtendedMaterial {
        base: StandardMaterial {
            base_color: Color::RED,
            opaque_render_method: OpaqueRendererMethod::Deferred,
            // In deferred mode, only the PbrInput can be modified (uvs,
            // color and other material properties), in forward mode, the
            // output can also be modified after lighting is applied. see
            // the fragment shader `extended_material.wgsl` for more info.
            // Note: to run in deferred mode, you must also add a
            // `DeferredPrepass` component to the camera and either change
            // the above to `OpaqueRendererMethod::Deferred` or add the
            // `DefaultOpaqueRendererMethod` resource.
            ..Default::default()
        },
        extension: PlaneCutExt {
            plane: Vec4::new(-1.0, 1.0, -2.0, 0.0),
            color: Color::rgb_linear(0.0, 0.0, 0.7),
            shaded: true,
            space: Space::World,
        },
    });
    // sphere
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Sphere::new(1.0)),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        material: handle,
        ..default()
    });

    // light
    commands.spawn((
        DirectionalLightBundle {
            transform: Transform::from_xyz(1.0, 1.0, 1.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        Rotate(Vec3::Y),
    ));

    // camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        DeferredPrepass,
    ));
}

#[derive(Component)]
struct Rotate(Vec3);

fn rotate_things(mut q: Query<(&mut Transform, &Rotate)>, time: Res<Time>) {
    for (mut t, r) in &mut q {
        t.rotate_axis(r.0, time.delta_seconds());
    }
}

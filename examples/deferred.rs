//! Demonstrates a sphere using the deferred renderer.
//!
//! This one looks weird. But I'm not sure why. Did I not setup the prepass
//! correctly?
use bevy::{core_pipeline::prepass::DeferredPrepass, pbr::OpaqueRendererMethod, prelude::*,
    color::palettes::basic,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, rotate_things)
        .insert_resource(Msaa::Off)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let handle = materials.add(StandardMaterial {
        base_color: basic::RED.into(),
        // Can be used in forward or deferred mode.
        opaque_render_method: OpaqueRendererMethod::Deferred,
        // In deferred mode, only the PbrInput can be modified (uvs,
        // color and other material properties). In forward mode, the
        // output can also be modified after lighting is applied. See
        // the fragment shader `extended_material.wgsl` for more info.
        // Note: to run in deferred mode, you must also add a
        // `DeferredPrepass` component to the camera and either change
        // the above to `OpaqueRendererMethod::Deferred` or add the
        // `DefaultOpaqueRendererMethod` resource.
        ..Default::default()
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
        Rotate(Dir3::Y),
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
struct Rotate(Dir3);

fn rotate_things(mut q: Query<(&mut Transform, &Rotate)>, time: Res<Time>) {
    for (mut t, r) in &mut q {
        t.rotate_axis(r.0, time.delta_seconds());
    }
}

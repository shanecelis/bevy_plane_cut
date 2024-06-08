//! Demonstrates a moving plane cut on a sphere.

use bevy::{
    pbr::{ExtendedMaterial, OpaqueRendererMethod},
    prelude::*,
    color::palettes::basic,
};

use bevy_plane_cut::{PlaneCutExt, PlaneCutMaterial, PlaneCutPlugin, Space};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PlaneCutPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (rotate_things, translate_things, update_plane))
        .run();
}

#[derive(Component)]
struct Plane(Handle<PlaneCutMaterial>);

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<PlaneCutMaterial>>,
) {
    let handle = materials.add(ExtendedMaterial {
        base: StandardMaterial {
            base_color: basic::RED.into(),
            opaque_render_method: OpaqueRendererMethod::Forward,
            ..Default::default()
        },
        extension: PlaneCutExt {
            plane: Vec4::new(-1.0, 1.0, -2.0, 0.0),
            color: Color::rgb_linear(0.0, 0.0, 0.7),
            shaded: true,
            space: Space::World,
        },
    });
    commands.spawn((
        TransformBundle {
            local: Transform { ..default() },
            ..default()
        },
        Plane(handle.clone()),
        // Rotate(Vec3::new(1.0, 1.0, 0.0))
        Translate(Vec3::new(1.0, 0.0, 0.0)),
    ));
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
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn update_plane(
    q: Query<(&GlobalTransform, &Plane)>,
    mut materials: ResMut<Assets<PlaneCutMaterial>>,
) {
    for (t, p) in &q {
        let Some(m) = materials.get_mut(&p.0) else {
            continue;
        };
        trace!("Updating plane");
        let normal = t.left();
        let w = normal.dot(t.translation());
        m.extension.plane = (*normal, w).into();
    }
}

#[derive(Component)]
struct Translate(Vec3);

#[derive(Component)]
struct Rotate(Dir3);

fn rotate_things(mut q: Query<(&mut Transform, &Rotate)>, time: Res<Time>) {
    for (mut t, r) in &mut q {
        t.rotate_axis(r.0, time.delta_seconds());
    }
}

fn translate_things(mut q: Query<(&mut Transform, &Translate)>, time: Res<Time>) {
    for (mut t, r) in &mut q {
        t.translation = (time.elapsed_seconds() / 1.0).sin().abs() * r.0;
    }
}

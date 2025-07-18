# bevy_plane_cut
![Maintenance](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)
[![CI](https://github.com/shanecelis/bevy_plane_cut/actions/workflows/rust.yml/badge.svg)](https://github.com/shanecelis/bevy_plane_cut/actions)
  [![crates-io](https://img.shields.io/crates/v/bevy_plane_cut.svg)](https://crates.io/crates/bevy_plane_cut)
  [![api-docs](https://docs.rs/bevy_plane_cut/badge.svg)](https://docs.rs/bevy_plane_cut)

A plane cut material for the [bevy game engine](https://bevyengine.org).

![simple example](https://github.com/shanecelis/bevy_plane_cut/assets/54390/d220108d-a0c0-4da7-bb84-b5a3dc223463)

# Install

Install the crate.

```sh
cargo add bevy_plane_cut
```

# Usage

## Add Plugin to App

```rust,no_run
use bevy::prelude::*;
fn main() {
    App::new()
        .add_plugins(bevy_plane_cut::PlaneCutPlugin)
        .run();
}
```

## Add Material to Object

```rust,compile
use bevy::{
    prelude::*,
    color::palettes::basic,
    pbr::{ExtendedMaterial, OpaqueRendererMethod},
};
use bevy_plane_cut::*;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<PlaneCutMaterial>>) {

    // sphere
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(1.0))),
        MeshMaterial3d(materials.add(ExtendedMaterial {
            base: StandardMaterial {
                base_color: basic::RED.into(),
                // Let's use the forward renderer.
                opaque_render_method: OpaqueRendererMethod::Forward,
                ..default()
            },
            extension: PlaneCutExt {
                plane: Vec4::new(-1.0, 1.0, -2.0, 0.0),
                color: Color::linear_rgb(0.0, 0.0, 0.7),
                shaded: true,
                space: Space::World,
            },
        })),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));
}
```

# Examples

Run the "simple" example like so:

```sh
cargo run --example simple
```

This will show a red sphere with a light rotating around it and blue plane cut.

* `simple` - A red sphere with a plane cut.
* `simple_screenspace` - A red sphere with a plane cut in screen space.
* `moving_cut` - A red sphere with a plane cut moving in and out.

## Not Working Examples

![simple_deferred](https://github.com/shanecelis/bevy_plane_cut/assets/54390/0b2fa6f5-6202-4301-b502-8fa37ae74c3f)

* `simple_deferred` - same as simple but using deferred renderer.
  NOTE: This one does not look right on my macOS machine.
* `deferred` - A red sphere rendered with deferred renderer. This has no plane
  cut at all and it still does not look right. I'm using macOS, so I'd be
  curious if it looks correct on other platforms.
* `two_cuts` - This is a material that has been extended by `PlaneCutExt` twice.
  However, it has a bug. See `two_cuts.rs` example for more details. PRs welcome!

# Compatibility

| bevy_plane_cut | bevy |
|----------------|------|
| 0.3            | 0.16 |
| 0.2            | 0.14 |
| 0.1            | 0.13 |

# License

This crate is licensed under the MIT License or the Apache License 2.0. The
examples are licensed under the CC0 license.

# Acknowlegments

* [Clipping a Model with a Plane](https://www.ronja-tutorials.com/post/021-plane-clipping/) by [Ronja](https://eldritch.cafe/@ronja) taught me the technique many years ago.

* [Extending Materials in Bevy 0.12 with Material Extension](https://www.rustadventure.dev/extending-materials-in-bevy-0-12-with-materialextension) by [Chris Biscardi](https://hachyderm.io/@chrisbiscardi) showed off how cool `ExtendedMaterial` is.

* Thanks to [robtfm](https://github.com/robtfm) who wrote the original [extended_material.rs](https://github.com/bevyengine/bevy/blob/release-0.13.2/examples/shader/extended_material.rs) example in bevy.

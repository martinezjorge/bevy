//! Hot reloading allows you to modify assets files to be immediately reloaded while your game is
//! running. This lets you immediately see the results of your changes without restarting the game.
//! This example illustrates hot reloading mesh changes.
//!
//! Note that hot asset reloading requires the [`AssetWatcher`](bevy::asset::io::AssetWatcher) to be enabled
//! for your current platform. For desktop platforms, enable the `file_watcher` cargo feature.

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Load our mesh:
    let scene_handle = asset_server.load("models/torus/torus.gltf#Scene0");

    // Any changes to the mesh will be reloaded automatically! Try making a change to torus.gltf.
    // You should see the changes immediately show up in your app.

    // mesh
    commands.spawn(SceneBundle {
        scene: scene_handle,
        ..default()
    });
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 150_000.0,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 5.0, 4.0),
        ..default()
    });
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(2.0, 2.0, 6.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

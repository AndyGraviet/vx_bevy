#![feature(map_first_last)]
use std::f32::consts::PI;

use bevy::prelude::*;

mod debug;
mod voxel;

fn main() {
    App::default()
        .add_plugins(DefaultPlugins)
        .add_plugin(voxel::VoxelWorldPlugin)
        .add_plugin(debug::DebugUIPlugins)
        .add_startup_system(setup)
        .add_startup_system(setup2)
        .run();
}

fn setup(mut cmds: Commands) {
    cmds.spawn_bundle(Camera3dBundle {
        projection: bevy::render::camera::Projection::Perspective(PerspectiveProjection {
            fov: PI / 2.,
            ..Default::default()
        }),
        transform: Transform::from_xyz(2.0, 160.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    })
    .insert(voxel::player::PlayerController::default())
    .insert(bevy_atmosphere::plugin::AtmosphereCamera(None));
}

fn setup2(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    let music = asset_server.load("../assets/sounds/sound1.ogg");
    audio.play(music);
}

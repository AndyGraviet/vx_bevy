use bevy::prelude::{Component, Plugin};
use ndshape::ConstShape3u32;

use super::storage::{VoxelMap, VoxelMapKey};

/// Systems for dynamically loading / unloading regions (aka chunks) of the world according to camera position.
mod chunks;
pub use chunks::{ChunkLoadingRadius, DirtyChunks};

/// Stuff and utilities for generating terrain.
mod terrain;

mod voxel;
pub use voxel::*;

mod render;

/// Registers all resources and systems for simulating and rendering an editable and interactive voxel world.
pub struct VoxelWorldPlugin;

impl Plugin for VoxelWorldPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(VoxelMap::<Voxel, ChunkShape>::new(ChunkShape {}))
            .add_plugin(chunks::VoxelWorldChunkingPlugin)
            .add_plugin(render::VoxelWorldRenderingPlugin)
            // ordering of plugin insertion matters here.
            .add_plugin(terrain::VoxelWorldTerrainGenPlugin);
        //insert voxel pipeline
    }
}

/// Component tagging a player.
#[derive(Component)]
pub struct Player;

pub type ChunkKey = VoxelMapKey<Voxel>;
pub const CHUNK_LENGTH: u32 = 32;
pub type ChunkShape = ConstShape3u32<CHUNK_LENGTH, CHUNK_LENGTH, CHUNK_LENGTH>;

// A component tagging an entity as a chunk.
#[derive(Component)]
pub struct Chunk(pub ChunkKey);

use bevy::prelude::{Color, Plugin};

use crate::{
    voxel::material::{MaterialRegistryInfo, VoxelMaterialFlags, VoxelMaterialRegistry},
    voxel_material,
};

voxel_material!(Dirt, 1);
voxel_material!(Sand, 2);
voxel_material!(Grass, 3);
voxel_material!(Rock, 4);
voxel_material!(Snow, 5);
voxel_material!(Water, 6);
voxel_material!(Sandstone, 7);
voxel_material!(Bedrock, 8);
voxel_material!(Cactus, 9);
voxel_material!(Wood, 10);
voxel_material!(Leaves, 11);

pub struct VoxelWorldBaseMaterialsPlugin;

impl Plugin for VoxelWorldBaseMaterialsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        let mut registry = app
            .world
            .get_resource_mut::<VoxelMaterialRegistry>()
            .unwrap();

        registry.register_material::<Dirt>(MaterialRegistryInfo {
            base_color: Color::rgb_u8(245,231,171),
            name: Dirt::NAME,
            flags: VoxelMaterialFlags::SOLID,
        });

        registry.register_material::<Sand>(MaterialRegistryInfo {
            base_color: Color::rgba_u8(245,231,171,96),
            name: Sand::NAME,
            flags: VoxelMaterialFlags::SOLID,
        });

        registry.register_material::<Grass>(MaterialRegistryInfo {
            base_color: Color::rgba_u8(155,245,122,96),
            name: Grass::NAME,
            flags: VoxelMaterialFlags::SOLID,
        });

        registry.register_material::<Rock>(MaterialRegistryInfo {
            base_color: Color::GRAY,
            name: Rock::NAME,
            flags: VoxelMaterialFlags::SOLID,
        });

        registry.register_material::<Snow>(MaterialRegistryInfo {
            base_color: Color::WHITE,
            name: Snow::NAME,
            flags: VoxelMaterialFlags::SOLID,
        });

        registry.register_material::<Water>(MaterialRegistryInfo {
            base_color: Color::rgba_u8(135,138,245,96),
            name: Water::NAME,
            flags: VoxelMaterialFlags::LIQUID,
        });

        registry.register_material::<Sandstone>(MaterialRegistryInfo {
            base_color: Color::rgb_u8(198, 192, 144),
            name: Sandstone::NAME,
            flags: VoxelMaterialFlags::SOLID,
        });

        registry.register_material::<Bedrock>(MaterialRegistryInfo {
            base_color: Color::DARK_GRAY,
            name: Bedrock::NAME,
            flags: VoxelMaterialFlags::UNBREAKABLE,
        });

        registry.register_material::<Cactus>(MaterialRegistryInfo {
            base_color: Color::rgb_u8(0, 96, 0),
            name: Cactus::NAME,
            flags: VoxelMaterialFlags::SOLID,
        });

        registry.register_material::<Wood>(MaterialRegistryInfo {
            base_color: Color::rgba_u8(245,231,171,96),
            name: Wood::NAME,
            flags: VoxelMaterialFlags::SOLID,
        });

        registry.register_material::<Leaves>(MaterialRegistryInfo {
            base_color: Color::rgba_u8(245,159,235,96),
            name: Leaves::NAME,
            flags: VoxelMaterialFlags::SOLID,
        });
    }
}

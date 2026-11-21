use bevy::prelude::*;
use bevy_voxel_world::prelude::*;
use std::sync::Arc;

#[derive(Resource, Clone, Default)]
struct WillaDadWorld;

impl VoxelWorldConfig for WillaDadWorld {
    type MaterialIndex = u8;
    type ChunkUserBundle = ();

    fn spawning_distance(&self) -> u32 {
        25
    }

    fn light_propagation(&self) -> Option<LightPropagation> {
        Some(LightPropagation::default())
    }

    fn texture_index_mapper(&self) -> Arc<dyn Fn(u8) -> [u32; 3] + Send + Sync> {
        Arc::new(|vox_mat: u8| match vox_mat {
            0 => [0, 1, 2],
            1 => [2, 2, 2],
            2 | _ => [3, 3, 3],
        })
    }

    fn emissive_materials(&self) -> Vec<(u8, Emission)> {
        vec![(3, Emission::level(15.0))]
    }

    /*
    fn voxel_texture(&self) -> Option<(String, u32)> {
        Some(("example_voxel_texture.png".into(), 4)) // Array texture with 4 indexes
    }
    */
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(VoxelWorldPlugin::with_config(WillaDadWorld))
        .add_systems(Startup, setup)
        .add_systems(Update, example_system)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        VoxelWorldCamera::<DefaultWorld>::default(),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // Ambient light
    commands.insert_resource(GlobalAmbientLight {
        color: Color::srgb(0.98, 0.95, 0.82),
        brightness: 1000.0,
        affects_lightmapped_meshes: true,
    });
}

fn example_system(mut world: VoxelWorld<WillaDadWorld>) {
    world.set_voxel(IVec3::new(0, -5, 0), WorldVoxel::Solid(0));
}

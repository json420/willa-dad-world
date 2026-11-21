use bevy::prelude::*;
use bevy_voxel_world::prelude::*;

#[derive(Resource, Clone, Default)]
struct WillaDadWorld;

impl VoxelWorldConfig for WillaDadWorld {
    type MaterialIndex = u8;
    type ChunkUserBundle = ();

    fn voxel_lookup_delegate(&self) -> VoxelLookupDelegate<Self::MaterialIndex> {
        // This is the Chunk closure, called once per chunk
        Box::new(move |_chunk_pos, _lod, _previous| {
            // And this is the Voxel closure, called for every voxel, 32^3, or 32768 times per chunk!
            Box::new(move |pos: IVec3, _previous| {
                let scale = 4.0;

                // Nice wavy sine hills!
                if (pos.y as f32)
                    < (pos.x as f32 / scale).sin() * scale + (pos.z as f32 / scale).sin() * scale
                {
                    // Every voxel below the sine threshold is solid
                    return WorldVoxel::Solid(0);
                }

                // Everything else is left unset
                WorldVoxel::Unset
            })
        })
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(VoxelWorldPlugin::with_config(WillaDadWorld))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, mut voxel_world: VoxelWorld<WillaDadWorld>) {
    // Camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-50.0, 50.0, -50.0).looking_at(Vec3::ZERO, Vec3::Y),
        // This tells bevy_voxel_world to use this cameras transform to calculate spawning area
        VoxelWorldCamera::<WillaDadWorld>::default(),
    ));

    // light
    commands.spawn((
        PointLight {
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    commands.insert_resource(GlobalAmbientLight {
        brightness: 500.0,
        ..default()
    });

    // Finally, a voxel!
    for z in -3..=3 {
        voxel_world.set_voxel(IVec3::new(0, 0, z), WorldVoxel::Solid(0));
    }
}

use bevy::prelude::*;
use bevy_voxel_world::prelude::*;

#[derive(Resource, Clone, Default)]
struct WillaDadWorld;

impl VoxelWorldConfig for WillaDadWorld {
    type MaterialIndex = u8;
    type ChunkUserBundle = ();
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
        Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
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

    // Finally, a voxel!
    for z in -3..=3 {
        voxel_world.set_voxel(IVec3::new(0, 0, z), WorldVoxel::Solid(0));
    }
}

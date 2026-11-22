use avian3d::prelude::*;
use bevy::{
    dev_tools::fps_overlay::FpsOverlayPlugin,
    input::mouse::AccumulatedMouseMotion,
    prelude::*,
    window::{CursorGrabMode, CursorOptions, PrimaryWindow, WindowMode},
};
use bevy_voxel_world::prelude::*;
use noise::{HybridMulti, NoiseFn, Perlin};
use std::collections::HashMap;
use std::f32::consts::FRAC_PI_2;
use std::sync::Arc;

const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.02;
const PLAYER_RADIUS: f32 = 0.35;
const PLAYER_HALF_HEIGHT: f32 = 1.0;

// Materials.
const GRASS: u8 = 0;
const DIRT: u8 = 1;
const STONE: u8 = 2;
const SNOWY_STONE: u8 = 3;
const COBBLESTONE: u8 = 4;
const DARK_PLANKS: u8 = 5;
const TIMBER: u8 = 6;
const ROOF: u8 = 7;
const LEAVES: u8 = 8;
const BARK: u8 = 9;
const LANTERN: u8 = 10;
const CRYSTAL: u8 = 11;
const GRAVEL: u8 = 12;
const FLOOR: u8 = 13;
const HEARTH: u8 = 14;

// Tiles of `assets/mountain_house_texture.png`, from the top.
const TILE_GRASS_TOP: u32 = 0;
const TILE_GRASS_SIDE: u32 = 1;
const TILE_DIRT: u32 = 2;
const TILE_STONE: u32 = 3;
const TILE_SNOW: u32 = 4;
const TILE_SNOWY_STONE_SIDE: u32 = 5;
const TILE_COBBLESTONE: u32 = 6;
const TILE_DARK_PLANKS: u32 = 7;
const TILE_TIMBER_SIDE: u32 = 8;
const TILE_TIMBER_TOP: u32 = 9;
const TILE_ROOF: u32 = 10;
const TILE_SPRUCE_LEAVES: u32 = 11;
const TILE_SPRUCE_BARK: u32 = 12;
const TILE_LANTERN: u32 = 13;
const TILE_CRYSTAL: u32 = 14;
const TILE_GRAVEL: u32 = 15;
const TILE_FLOOR_PLANKS: u32 = 16;
const TILE_HEARTH: u32 = 17;
const TILE_COUNT: u32 = 18;

fn texture_indices(material: u8) -> [u32; 3] {
    match material {
        GRASS => [TILE_GRASS_TOP, TILE_GRASS_SIDE, TILE_DIRT],
        DIRT => [TILE_DIRT; 3],
        STONE => [TILE_STONE; 3],
        SNOWY_STONE => [TILE_SNOW, TILE_SNOWY_STONE_SIDE, TILE_STONE],
        COBBLESTONE => [TILE_COBBLESTONE; 3],
        DARK_PLANKS => [TILE_DARK_PLANKS; 3],
        TIMBER => [TILE_TIMBER_TOP, TILE_TIMBER_SIDE, TILE_TIMBER_TOP],
        ROOF => [TILE_ROOF; 3],
        LEAVES => [TILE_SPRUCE_LEAVES; 3],
        BARK => [TILE_SPRUCE_BARK; 3],
        LANTERN => [TILE_LANTERN; 3],
        CRYSTAL => [TILE_CRYSTAL; 3],
        GRAVEL => [TILE_GRAVEL; 3],
        FLOOR => [TILE_FLOOR_PLANKS; 3],
        HEARTH => [TILE_HEARTH; 3],
        _ => [TILE_STONE; 3],
    }
}

struct NoiseMaker {
    pub a: HybridMulti<Perlin>,
    pub b: HybridMulti<Perlin>,
    map: HashMap<(i32, i32), (f64, f64)>,
}

impl Default for NoiseMaker {
    fn default() -> Self {
        let mut a = HybridMulti::<Perlin>::new(012345);
        a.octaves = 6;
        a.frequency = 1.1;
        a.lacunarity = 2.8;
        a.persistence = 0.4;

        let mut b = HybridMulti::<Perlin>::new(98765);
        b.octaves = 5;
        b.frequency = 1.5;
        b.lacunarity = 2.8;
        b.persistence = 0.4;

        Self {
            a,
            b,
            map: HashMap::with_capacity(1024),
        }
    }
}

impl NoiseMaker {
    pub fn get(&mut self, pos: &IVec3) -> (f64, f64) {
        let key = (pos.x, pos.z);
        if let Some(value) = self.map.get(&key) {
            *value
        } else {
            let [x, _y, z] = pos.as_dvec3().to_array();
            let a = self.a.get([x / 1000.0, z / 1000.0]) * 50.0;
            let b = self.b.get([x / 1000.0, z / 1000.0]) * 10.0;
            let value = (a, b);
            self.map.insert(key, value.clone());
            value
        }
    }
}

#[derive(Event)]
struct ToggleFullscreen;

#[derive(Event)]
struct UnFullscreen;

#[derive(Resource, Clone, Default)]
struct WillaDadWorld;

impl VoxelWorldConfig for WillaDadWorld {
    type MaterialIndex = u8;
    type ChunkUserBundle = ();

    fn spawning_distance(&self) -> u32 {
        8
    }

    fn chunk_spawn_strategy(&self) -> ChunkSpawnStrategy {
        ChunkSpawnStrategy::Close
    }

    fn chunk_despawn_strategy(&self) -> ChunkDespawnStrategy {
        ChunkDespawnStrategy::FarAway
    }

    fn voxel_lookup_delegate(&self) -> VoxelLookupDelegate<Self::MaterialIndex> {
        Box::new(move |_chunk_pos, _lod, _previous| {
            let mut noise = NoiseMaker::default();

            Box::new(move |pos: IVec3, _previous| {
                if pos.y < -10 {
                    return WorldVoxel::Solid(GRASS);
                }
                if pos.y > 50 {
                    return WorldVoxel::Unset;
                }
                let (a, b) = noise.get(&pos);
                let y = pos.y as f64;
                if y < a {
                    let s2 = y + b;
                    if s2 < 15.0 {
                        WorldVoxel::Solid(GRASS)
                    } else if s2 < 25.0 {
                        WorldVoxel::Solid(DIRT)
                    } else if s2 < 30.0 {
                        WorldVoxel::Solid(STONE)
                    } else {
                        WorldVoxel::Solid(SNOWY_STONE)
                    }
                } else {
                    WorldVoxel::Unset
                }
            })
        })
    }

    fn texture_index_mapper(&self) -> Arc<dyn Fn(Self::MaterialIndex) -> [u32; 3] + Send + Sync> {
        Arc::new(texture_indices)
    }

    fn voxel_texture(&self) -> Option<(String, u32)> {
        Some(("mountain_house_texture.png".into(), TILE_COUNT))
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(FpsOverlayPlugin::default())
        .add_plugins(VoxelWorldPlugin::with_config(WillaDadWorld::default()))
        .add_systems(Startup, setup)
        .add_systems(Update, (button_input, mouse_move))
        .add_observer(toggle_fullscreen)
        .add_observer(un_fullscreen)
        .run();
}

fn setup(mut commands: Commands, mut voxel_world: VoxelWorld<WillaDadWorld>) {
    // Camera
    commands.spawn((
        Camera3d::default(),
        VoxelWorldCamera::<WillaDadWorld>::default(),
        RigidBody::Static,
        Collider::capsule(PLAYER_RADIUS, PLAYER_HALF_HEIGHT),
        Transform::from_xyz(-40.0, 10.0, -40.0).looking_at(Vec3::ZERO, Vec3::Y),
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

fn button_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut cursor_options: Single<&mut CursorOptions>,
    mut commands: Commands,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        cursor_options.visible = true;
        cursor_options.grab_mode = CursorGrabMode::None;
        commands.trigger(UnFullscreen);
    }
    if keyboard.just_pressed(KeyCode::F11) {
        commands.trigger(ToggleFullscreen)
    }
    if mouse.any_just_pressed([MouseButton::Left, MouseButton::Right]) {
        if cursor_options.visible || cursor_options.grab_mode != CursorGrabMode::Locked {
            cursor_options.visible = false;
            cursor_options.grab_mode = CursorGrabMode::Locked;
        } else {
            println!("would do stuff, yo");
        }
    }
}

fn mouse_move(
    motion: Res<AccumulatedMouseMotion>,
    mut transform: Single<&mut Transform, With<Camera3d>>,
) {
    let delta_yaw = -motion.delta.x * 0.003;
    let delta_pitch = -motion.delta.y * 0.002;
    let (yaw, pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);
    let pitch = (pitch + delta_pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);
    let yaw = yaw + delta_yaw;
    transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
}

fn toggle_fullscreen(
    _on: On<ToggleFullscreen>,
    mut window: Single<&mut Window, With<PrimaryWindow>>,
    mut cursor_options: Single<&mut CursorOptions>,
) {
    window.mode = match window.mode {
        WindowMode::Windowed => {
            cursor_options.visible = false;
            cursor_options.grab_mode = CursorGrabMode::Locked;
            WindowMode::BorderlessFullscreen(MonitorSelection::Primary)
        }
        _ => {
            cursor_options.visible = true;
            cursor_options.grab_mode = CursorGrabMode::None;
            WindowMode::Windowed
        }
    };
}

fn un_fullscreen(_on: On<UnFullscreen>, mut window: Single<&mut Window, With<PrimaryWindow>>) {
    window.mode = WindowMode::Windowed;
}

use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use bevy::render::mesh::PrimitiveTopology;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy::tasks::{AsyncComputeTaskPool, Task};
use bevy::tasks::futures_lite::future;
use crate::components::tilemap::{Chunk, TileMap, TileType};

pub const CHUNK_SIZE: i32 = 64;
pub const LOAD_RADIUS: i64 = 7;
pub const SEED: u32 = 123123;
pub const PLAYER_SPEED: f32 = 1000.0;

pub const NOISE_SCALE: f64 = 0.02;
pub const CAMERA_SCALE: f32 = 1.0;


#[derive(Component)]
pub struct ChunkLoadingTask(Task<(i64, i64, Chunk)>);

pub(crate) fn chunk_loader_system(
    mut commands: Commands,
    mut tile_map: ResMut<TileMap>,
    camera_query: Query<&Transform, With<Camera>>,
) {
    // Get the camera position
    let camera_transform = camera_query.single();
    let camera_position = camera_transform.translation;

    // Calculate the chunk coordinates based on the camera's position
    let chunk_x = (camera_position.x / (CHUNK_SIZE as f32 * 2.0)).floor() as i64;
    let chunk_y = (camera_position.y / (CHUNK_SIZE as f32 * 2.0)).floor() as i64;


    // Load chunks within the radius of the camera position
    let task_pool = AsyncComputeTaskPool::get(); // Access the async compute task pool directly

    for x in (chunk_x - LOAD_RADIUS)..=(chunk_x + LOAD_RADIUS) {
        for y in (chunk_y - LOAD_RADIUS)..=(chunk_y + LOAD_RADIUS) {
            if !tile_map.chunks.contains_key(&(x, y)) {
                // Spawn a new async task to generate the chunk
                let task = task_pool.spawn(async move {
                    let chunk = Chunk::new((x, y));
                    (x, y, chunk)
                });

                commands.spawn(ChunkLoadingTask(task));
            }
        }
    }

    // Unload chunks that are outside the load radius
    let chunks_to_unload: Vec<(i64, i64)> = tile_map
        .chunks
        .keys()
        .filter(|&&(x, y)| (x - chunk_x).abs() > LOAD_RADIUS || (y - chunk_y).abs() > LOAD_RADIUS)
        .cloned()
        .collect();

    for chunk_coords in chunks_to_unload {
        if let Some(chunk) = tile_map.chunks.remove(&chunk_coords) {
            for entity in chunk.tile_entities {
                commands.entity(entity).despawn();
            }
        }
    }
}
pub(crate) fn poll_chunk_tasks(
    mut commands: Commands,
    mut tile_map: ResMut<TileMap>,
    mut tasks: Query<(Entity, &mut ChunkLoadingTask)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (entity, mut task) in tasks.iter_mut() {
        if let Some((x, y, chunk)) = future::block_on(future::poll_once(&mut task.0)) {
            // The chunk loading is complete, insert the chunk into the map
            tile_map.chunks.insert((x, y), chunk);

            // Now render the chunk
            if let Some(mut chunk) = tile_map.chunks.get_mut(&(x, y)) {
                render_chunk(&mut chunk, &mut commands, &mut meshes, &mut materials);
            }

            // Despawn the task entity
            commands.entity(entity).despawn();
        }
    }
}

fn render_chunk(
    chunk: &mut Chunk,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let tile_size = 2.0; // Each tile is 2x2 units in size
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default());

    // Define vertices and uvs
    let half_size = tile_size / 2.0;
    let positions = vec![
        [half_size, half_size, 0.0],
        [half_size, -half_size, 0.0],
        [-half_size, -half_size, 0.0],
        [-half_size, half_size, 0.0],
    ];

    let indices = vec![0, 1, 3, 1, 2, 3];

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_indices(bevy::render::mesh::Indices::U32(indices));

    let mesh_handle = meshes.add(mesh);

    // Define materials for each TileType
    let grass_material = materials.add(ColorMaterial::from(Color::rgb(0.1, 0.8, 0.1)));
    let dirt_material = materials.add(ColorMaterial::from(Color::rgb(0.5, 0.3, 0.1)));
    let water_material = materials.add(ColorMaterial::from(Color::rgb(0.1, 0.1, 0.8)));

    // Spawn each tile within the chunk
    for tile in &chunk.tiles {
        let material_handle = match tile.tile_type {
            TileType::Grass => grass_material.clone(),
            TileType::Dirt => dirt_material.clone(),
            TileType::Water => water_material.clone(),
        };

        let entity = commands.spawn(MaterialMesh2dBundle {
            mesh: Mesh2dHandle(mesh_handle.clone()),
            material: material_handle,
            transform: Transform::from_translation(Vec3::new(
                tile.position.x * tile_size + chunk.chunk_coords.0 as f32 * CHUNK_SIZE as f32 * tile_size,
                tile.position.y * tile_size + chunk.chunk_coords.1 as f32 * CHUNK_SIZE as f32 * tile_size,
                0.0,
            )),
            ..default()
        }).id();

        chunk.tile_entities.push(entity);
    }
}
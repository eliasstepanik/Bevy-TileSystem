use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use bevy::render::mesh::PrimitiveTopology;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use crate::components::tilemap::{Chunk, TileMap, TileType};

pub const CHUNK_SIZE: i32 = 64;

pub(crate) fn chunk_loader_system(
    mut commands: Commands,
    mut tile_map: ResMut<TileMap>,
    camera_query: Query<&Transform, With<Camera>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Get the camera position
    let camera_transform = camera_query.single();
    let camera_position = camera_transform.translation;

    // Calculate the chunk coordinates based on the camera's position
    let chunk_x = (camera_position.x / (CHUNK_SIZE as f32 * 2.0)).floor() as i64;
    let chunk_y = (camera_position.y / (CHUNK_SIZE as f32 * 2.0)).floor() as i64;

    let load_radius = 5; // Number of chunks to load around the camera

    // Load chunks within the radius of the camera position
    for x in (chunk_x - load_radius)..=(chunk_x + load_radius) {
        for y in (chunk_y - load_radius)..=(chunk_y + load_radius) {
            if !tile_map.chunks.contains_key(&(x, y)) {
                let mut new_chunk = Chunk::new((x, y));
                render_chunk(&mut new_chunk, &mut commands, &mut meshes, &mut materials);
                tile_map.chunks.insert((x, y), new_chunk);
            }
        }
    }

    // Unload chunks that are outside the load radius
    let chunks_to_unload: Vec<(i64, i64)> = tile_map
        .chunks
        .keys()
        .filter(|&&(x, y)| (x - chunk_x).abs() > load_radius || (y - chunk_y).abs() > load_radius)
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
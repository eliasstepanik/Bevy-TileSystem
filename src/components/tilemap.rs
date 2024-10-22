use std::collections::HashMap;
use bevy::prelude::*;
use noise::{NoiseFn, Perlin};
use crate::components::world::{CHUNK_SIZE, NOISE_SCALE, SEED};

pub struct Chunk {
    pub tiles: Vec<Tile>,
    pub chunk_coords: (i64, i64),
    pub tile_entities: Vec<Entity>, // List of entity IDs for the tiles in this chunk
}
impl Chunk {
    pub(crate) fn new(chunk_coords: (i64, i64)) -> Self {
        info!("Generating chunk at coordinates: {:?}", chunk_coords);

        let perlin = Perlin::new(SEED);
        let mut tiles = Vec::new();

        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                let world_x = chunk_coords.0 * CHUNK_SIZE as i64 + x as i64;
                let world_y = chunk_coords.1 * CHUNK_SIZE as i64 + y as i64;

                let noise_value = perlin.get([world_x as f64 * NOISE_SCALE, world_y as f64 * NOISE_SCALE]);

                let tile_type = if noise_value > 0.5 {
                    TileType::Grass
                } else if noise_value > 0.0 {
                    TileType::Dirt
                } else {
                    TileType::Water
                };

                tiles.push(Tile {
                    tile_type,
                    position: Vec2::new(x as f32, y as f32),
                });
            }
        }

        Chunk {
            tiles,
            chunk_coords,
            tile_entities: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileType {
    Grass,
    Dirt,
    Water,
}

pub struct Tile {
    pub(crate) tile_type: TileType,
    pub(crate) position: Vec2, // Position within the chunk
}

#[derive(Resource)]
pub struct TileMap {
    pub(crate) chunks: HashMap<(i64, i64), Chunk>,
}

impl TileMap {
    pub(crate) fn new() -> Self {
        TileMap {
            chunks: HashMap::new(),
        }
    }

    pub(crate) fn load_chunk(&mut self, chunk_coords: (i64, i64)) {
        // Load or generate the chunk at chunk_coords
        if !self.chunks.contains_key(&chunk_coords) {
            let new_chunk = Chunk::new(chunk_coords);
            self.chunks.insert(chunk_coords, new_chunk);
        }
    }

    fn unload_chunk(&mut self, chunk_coords: (i64, i64)) {
        self.chunks.remove(&chunk_coords);
    }
}

use bevy::prelude::*;
use noise::{MultiFractal, NoiseFn, OpenSimplex, RidgedMulti};
use vinox_common::world::chunks::storage::{BlockData, RawChunk, CHUNK_SIZE};

// Just some interesting stuff to look at while testing
pub fn add_grass(raw_chunk: &mut RawChunk) {
    for x in 0..=CHUNK_SIZE - 1 {
        for z in 0..=CHUNK_SIZE - 1 {
            for y in 0..=CHUNK_SIZE - 2 {
                if raw_chunk.get_identifier(UVec3::new(x, y + 1, z)) == "vinox:air"
                    && raw_chunk.get_identifier(UVec3::new(x, y, z)) == "vinox:cobblestone"
                {
                    let grass = BlockData::new("vinox".to_string(), "grass".to_string());
                    raw_chunk.set_block(UVec3::new(x, y, z), &grass);
                }
            }
        }
    }
}

pub fn generate_chunk(pos: IVec3, seed: u32) -> RawChunk {
    //TODO: Switch to using ron files to determine biomes and what blocks they should use. For now hardcoding a simplex noise
    let ridged_noise: RidgedMulti<OpenSimplex> =
        RidgedMulti::new(seed).set_octaves(8).set_frequency(0.25);
    let mut raw_chunk = RawChunk::new();
    for x in 0..=CHUNK_SIZE - 1 {
        for z in 0..=CHUNK_SIZE - 1 {
            for y in 0..=CHUNK_SIZE - 1 {
                let full_x = x as i32 + ((CHUNK_SIZE as i32) * pos.x);
                let full_z = z as i32 + ((CHUNK_SIZE as i32) * pos.z);
                let full_y = y as i32 + ((CHUNK_SIZE as i32) * pos.y);
                let noise_val =
                    ridged_noise.get([(full_x as f64 / 100.0), (full_z as f64 / 100.0)]) * 100.0;
                if full_y as f64 <= noise_val {
                    raw_chunk.set_block(
                        UVec3::new(x, y, z),
                        &BlockData::new("vinox".to_string(), "cobblestone".to_string()),
                    );
                } else {
                    raw_chunk.set_block(
                        UVec3::new(x, y, z),
                        &BlockData::new("vinox".to_string(), "air".to_string()),
                    );
                }
            }
        }
    }
    add_grass(&mut raw_chunk);
    raw_chunk
}

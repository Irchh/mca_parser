use inbt::NbtTag;
use log::{trace, warn};
use crate::{Block, Position};
use crate::parser::section::Section;

#[derive(Debug, Clone)]
pub struct Chunk {
    data_version: i32,
    chunk_pos: Position,
    status: String,
    sections: Vec<Section>,
    chunk_data: NbtTag,
}

impl Chunk {
    pub fn new(data_version: i32, chunk_pos: Position, status: String, sections: Vec<Section>, chunk_data: NbtTag) -> Self {
        Self {
            data_version,
            chunk_pos,
            status,
            sections,
            chunk_data,
        }
    }
    /// Gets block relative to chunk origin
    pub fn get(&self, pos: Position) -> Option<&Block> {
        let section = pos.section_index_in_chunk();
        if section.is_none() {
            warn!("Warning: section index out of bounds (Original Y: {})", pos.y);
        }
        Some(self.sections[section? as usize].get(pos))
    }

    /// Returns a vector with chunk data that can be put directly into a chunk data packet
    pub fn network_data(&self, f: fn(&String) -> i32) -> Vec<u8> {
        trace!("{} sections", self.sections.len());
        self.sections.iter().flat_map(|s| s.network_data(f)).collect()
    }

    pub fn is_finished(&self) -> bool {
        &*self.status == "minecraft:full"
    }

    pub fn data_version(&self) -> i32 {
        self.data_version
    }

    pub fn chunk_pos(&self) -> &Position {
        &self.chunk_pos
    }

    pub fn status(&self) -> &String {
        &self.status
    }

    pub fn sections(&self) -> &Vec<Section> {
        &self.sections
    }

    pub fn chunk_data(&self) -> &NbtTag {
        &self.chunk_data
    }
}
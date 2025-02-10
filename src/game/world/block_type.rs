#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BlockType {
    #[default]
    Grass,
    Sand,
    Water,
}

impl BlockType {
    pub const fn is_collidable(&self) -> bool {
        match self {
            BlockType::Water => true,
            _ => false,
        }
    }

    pub const fn get_atlas_index(&self) -> usize {
        match self {
            BlockType::Grass => 0,
            BlockType::Sand => 1,
            BlockType::Water => 2,
        }
    }
}

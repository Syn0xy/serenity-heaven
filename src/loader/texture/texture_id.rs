pub use StaticId::*;
pub use TextureId::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TextureId {
    Static(StaticId),
    Tileset(TilesetId),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StaticId {
    Player(PlayerId),
    Slime(SlimeId),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TilesetId {
    Nature,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PlayerId {
    Idle,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SlimeId {
    Idle,
    Dead,
    Jump,
}

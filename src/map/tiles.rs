use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

struct TileSheetInfo {
    tile_size: u32,
    horizontal_count: u32,
    vertical_count: u32,
}

// Dungeon Tile Set
const DEFAULT_TILE_INFO: TileSheetInfo = TileSheetInfo {
    tile_size: 16,
    horizontal_count: 12,
    vertical_count: 12,
};

#[derive(Debug, PartialEq, Clone, Copy, Component)]
pub enum TileType {
    Wall,
    Floor,
}

impl TileType {
    pub fn get_bundle(self, position: TilePos, tilemap_entity: Entity) -> impl Bundle {
        (
            self,
            TileBundle {
                position,
                tilemap_id: TilemapId(tilemap_entity),
                texture_index: TileTextureIndex(self.texture_index()),
                ..Default::default()
            },
        )
    }
    fn texture_index(&self) -> u32 {
        match self {
            Self::Floor => 4 * DEFAULT_TILE_INFO.horizontal_count,
            Self::Wall => 3 * DEFAULT_TILE_INFO.horizontal_count,
        }
    }
}

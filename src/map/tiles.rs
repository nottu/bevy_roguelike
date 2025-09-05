use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

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
            Self::Floor => 4 * 12,
            Self::Wall => 3 * 12,
        }
    }
}

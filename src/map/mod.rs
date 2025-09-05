use bevy::{log, prelude::*};
use bevy_ecs_tilemap::prelude::*;
use bevy_rand::prelude::*;
use rand::Rng;

use crate::{DungeonAssets, GameState, map::tiles::TileType};

mod tiles;

#[derive(Debug)]
pub struct MapPlugin {
    x: u32,
    y: u32,
}

impl MapPlugin {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

impl Default for MapPlugin {
    /// Default size is 80x80
    fn default() -> Self {
        Self::new(20, 20)
    }
}

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        let (x, y) = (self.x, self.y);
        app.add_plugins(TilemapPlugin).add_systems(
            OnEnter(GameState::InDungeon),
            move |cmd: Commands, shared_atlas: Res<DungeonAssets>, rng: GlobalEntropy<WyRand>| {
                bevy::log::info!("Spawning TileMap");
                load_map(x, y, cmd, shared_atlas, rng);
            },
        );
    }
}

fn load_map(
    x: u32,
    y: u32,
    mut commands: Commands,
    shared_atlas: Res<DungeonAssets>,
    mut rng: GlobalEntropy<WyRand>,
) {
    let texture_handle: Handle<Image> = shared_atlas.sprite.clone_weak();

    let map_size = TilemapSize { x, y };

    // Create a tilemap entity a little early.
    // We want this entity early because we need to tell each tile which tilemap entity
    // it is associated with. This is done with the TilemapId component on each tile.
    // Eventually, we will insert the `TilemapBundle` bundle on the entity, which
    // will contain various necessary components, such as `TileStorage`.
    let tilemap_entity = commands.spawn_empty().id();

    // To begin creating the map we will need a `TileStorage` component.
    // This component is a grid of tile entities and is used to help keep track of individual
    // tiles in the world. If you have multiple layers of tiles you would have a tilemap entity
    // per layer, each with their own `TileStorage` component.
    let mut tile_storage = TileStorage::empty(map_size);

    // Spawn the elements of the tilemap.
    for x in 0..map_size.x {
        let tile_pos = TilePos { x, y: 0 };
        let tile_entity = commands
            .spawn(TileType::Wall.get_bundle(tile_pos, tilemap_entity))
            .id();
        tile_storage.set(&tile_pos, tile_entity);

        let tile_pos = TilePos { x, y: y - 1 };
        let tile_entity = commands
            .spawn(TileType::Wall.get_bundle(tile_pos, tilemap_entity))
            .id();
        tile_storage.set(&tile_pos, tile_entity);
    }
    for y in 0..map_size.y {
        let tile_pos = TilePos { x: 0, y };
        let tile_entity = commands
            .spawn(TileType::Wall.get_bundle(tile_pos, tilemap_entity))
            .id();
        tile_storage.set(&tile_pos, tile_entity);

        let tile_pos = TilePos {
            x: map_size.x - 1,
            y,
        };
        let tile_entity = commands
            .spawn(TileType::Wall.get_bundle(tile_pos, tilemap_entity))
            .id();
        tile_storage.set(&tile_pos, tile_entity);
    }

    let mut wall_cnt = 4;
    for x in 1..(map_size.x - 1) {
        for y in 1..(map_size.y - 1) {
            let rand_val = rng.random_range(0..map_size.x * map_size.y);
            let tile_pos = TilePos { x, y };
            let tile_type = if rand_val < wall_cnt {
                wall_cnt -= 1;
                TileType::Wall
            } else {
                TileType::Floor
            };
            let tile_entity = commands
                .spawn(tile_type.get_bundle(tile_pos, tilemap_entity))
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    // TODO: can we get this info from the shared_atlas.layout?
    let tile_size = TilemapTileSize { x: 16.0, y: 16.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size,
        anchor: TilemapAnchor::Center,
        ..Default::default()
    });
    log::info!("Loaded Map Tiles");
}

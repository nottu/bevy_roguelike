use bevy::{log, prelude::*};
use bevy_asset_loader::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::map::MapPlugin;
use crate::player::*;

mod map;
mod player;
pub struct GamePlugin;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, States)]
enum GameState {
    #[default]
    AssetLoading,
    InDungeon,
}

#[derive(AssetCollection, Resource)]
pub(crate) struct DungeonAssets {
    #[asset(texture_atlas_layout(tile_size_x = 16, tile_size_y = 16, columns = 12, rows = 11))]
    layout: Handle<TextureAtlasLayout>,
    #[asset(path = "dungeon/tiles.png")]
    sprite: Handle<Image>,
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MapPlugin::default())
            .add_plugins(PlayerPlugin)
            .init_state::<GameState>()
            .add_loading_state(
                LoadingState::new(GameState::AssetLoading)
                    .continue_to_state(GameState::InDungeon)
                    .load_collection::<DungeonAssets>(),
            )
            .add_systems(PreStartup, init_game)
            .add_systems(
                Update,
                apply_grid_move.run_if(in_state(GameState::InDungeon)),
            );
    }
}

pub fn init_game(mut commands: Commands) {
    bevy::log::info!("Initializing Game");
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scale: 0.5,
            ..OrthographicProjection::default_2d()
        }),
    ));
}

pub fn apply_grid_move(
    tilemap_query: Query<(
        &Transform,
        &TilemapGridSize,
        &TilemapType,
        &TilemapSize,
        &TilemapTileSize,
        &TilemapAnchor,
    )>,
    mut query: Query<(&TilePos, &mut Transform), Without<TilemapType>>,
) {
    log::info_once!("Applying Grid Move");
    let Ok((map_transform, grid_size, map_type, map_size, tile_size, anchor)) =
        tilemap_query.single()
    else {
        log::warn_once!(
            "tilemap_query is_empty, err:{:?}",
            tilemap_query.single().unwrap_err()
        );
        return;
    };
    for (tile_pos, mut player_transform) in &mut query {
        let tile_center =
            tile_pos.center_in_world(map_size, grid_size, tile_size, map_type, anchor);

        let world_pos = *map_transform * Transform::from_translation(tile_center.extend(1.0));
        // world_pos.translation.z = 1.0;
        player_transform.translation = world_pos.translation;
    }
}

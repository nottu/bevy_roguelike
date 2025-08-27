use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::map::MapPlugin;
use crate::player::*;

mod map;
mod player;
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MapPlugin::default())
            .add_plugins(PlayerPlugin)
            .add_systems(PreStartup, init_game)
            .add_systems(Update, apply_grid_move);
    }
}

#[derive(Resource)]
pub struct SharedAtlasHandles {
    pub texture: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
}

pub fn init_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    bevy::log::info!("Initializing Game");
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scale: 0.5,
            ..OrthographicProjection::default_2d()
        }),
    ));

    let texture: Handle<Image> = asset_server.load("dungeon/tiles.png");

    // Create a TextureAtlasLayout from the tilesheet grid
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 12, 11, None, None);
    let layout_handle = layouts.add(layout);

    // Insert our handles resource
    commands.insert_resource(SharedAtlasHandles {
        texture,
        layout: layout_handle,
    });
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
    let (map_transform, grid_size, map_type, map_size, tile_size, anchor) =
        tilemap_query.single().expect("expected single tilemap");
    for (tile_pos, mut player_transform) in &mut query {
        let tile_center =
            tile_pos.center_in_world(map_size, grid_size, tile_size, map_type, anchor);

        let world_pos = *map_transform * Transform::from_translation(tile_center.extend(1.0));
        // world_pos.translation.z = 1.0;
        player_transform.translation = world_pos.translation;
    }
}

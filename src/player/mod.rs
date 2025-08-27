use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::TilePos;

use crate::SharedAtlasHandles;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands, shared_atlas: Res<SharedAtlasHandles>) {
    let player_sprite_idx = 7 * 12;
    let atlas = TextureAtlas {
        layout: shared_atlas.layout.clone_weak(),
        index: player_sprite_idx,
    };
    commands.spawn((
        Player,
        Sprite {
            image: shared_atlas.texture.clone_weak(),
            texture_atlas: Some(atlas),
            ..Default::default()
        },
        Transform::from_xyz(0.0, 0.0, 1.0),
        TilePos { x: 10, y: 10 },
    ));
}

#[derive(Debug, Component)]
pub struct Player;

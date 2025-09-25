use bevy::{log, prelude::*};
use bevy_ecs_tilemap::tiles::TilePos;
use leafwing_input_manager::prelude::*;

use crate::{Collider, DungeonAssets, GameState, WantsToMove};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<Action>::default())
            .add_systems(OnEnter(GameState::InDungeon), spawn_player)
            .add_systems(Update, move_player.run_if(in_state(GameState::InDungeon)));
    }
}

#[derive(Debug, Component)]
pub struct Player;
fn spawn_player(mut commands: Commands, shared_atlas: Res<DungeonAssets>) {
    let player_sprite_idx = 7 * 12;
    let atlas = TextureAtlas {
        layout: shared_atlas.layout.clone_weak(),
        index: player_sprite_idx,
    };
    log::info!("Spawning Player");
    commands.spawn((
        Player,
        Sprite {
            image: shared_atlas.sprite.clone_weak(),
            texture_atlas: Some(atlas),
            ..Default::default()
        },
        Transform::from_xyz(0.0, 0.0, 1.0),
        TilePos { x: 10, y: 10 },
        Collider,
        Action::default_input_map(),
    ));
}

#[derive(Debug, Actionlike, Reflect, Clone, Copy, Hash, PartialEq, Eq)]
enum Action {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
}

impl Action {
    fn default_input_map() -> InputMap<Self> {
        InputMap::default()
            .with(Action::MoveUp, KeyCode::KeyW)
            .with(Action::MoveDown, KeyCode::KeyS)
            .with(Action::MoveLeft, KeyCode::KeyA)
            .with(Action::MoveRight, KeyCode::KeyD)
    }
}

fn move_player(
    mut commands: Commands,
    player_action_query: Query<(Entity, &ActionState<Action>), With<Player>>,
) {
    let (entity, action_state) = player_action_query
        .single()
        .expect("Player actions not found");

    let mut move_direction = IVec2::ZERO;
    if action_state.just_pressed(&Action::MoveUp) {
        move_direction.y += 1;
    } else if action_state.just_pressed(&Action::MoveDown) {
        move_direction.y -= 1;
    } else if action_state.just_pressed(&Action::MoveLeft) {
        move_direction.x -= 1;
    } else if action_state.just_pressed(&Action::MoveRight) {
        move_direction.x += 1;
    }

    if move_direction != IVec2::ZERO {
        commands.entity(entity).insert(WantsToMove(move_direction));
    }
}

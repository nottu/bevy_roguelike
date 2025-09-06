use bevy::{log, prelude::*};
use bevy_ecs_tilemap::tiles::TilePos;
use leafwing_input_manager::prelude::*;

use crate::{Collider, DungeonAssets, GameState, GameTick, WantsToMove};

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

#[derive(Debug, Actionlike, Reflect, Clone, Copy, Hash, PartialEq, Eq)]
enum Action {
    #[actionlike(DualAxis)]
    Move,
}

impl Action {
    fn default_input_map() -> InputMap<Self> {
        InputMap::default()
            .with_dual_axis(Action::Move, GamepadStick::LEFT)
            .with_dual_axis(Action::Move, VirtualDPad::wasd())
    }
}

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

fn move_player(
    mut commands: Commands,
    player_action_query: Query<(Entity, &ActionState<Action>), With<Player>>,
    mut game_tick_query: Query<&mut GameTick>,
    time: Res<Time>,
) {
    let (entity, action_state) = player_action_query
        .single()
        .expect("Player actions not found");

    let mut game_tick = game_tick_query.single_mut().expect("expected one timer");
    game_tick.timer.tick(time.delta());
    // no timer tick, nothing to do
    if !game_tick.timer.finished() {
        return;
    }

    let move_direction = action_state.axis_pair(&Action::Move).as_ivec2();

    if move_direction != IVec2::ZERO {
        commands.entity(entity).insert(WantsToMove(move_direction));
    }
}

use bevy::{log, prelude::*};
use bevy_ecs_tilemap::tiles::TilePos;
use bevy_enhanced_input::prelude::*;

use crate::{DungeonAssets, GameState};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_input_context::<Player>()
            .add_systems(OnEnter(GameState::InDungeon), setup)
            .add_observer(
                apply_movement, //.run_if(in_state(GameState::InDungeon)),
            );
    }
}

fn setup(mut commands: Commands, shared_atlas: Res<DungeonAssets>) {
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
        actions! {Player[
            (
                Action::<Move>::new(),
                DeadZone::default(),
                SmoothNudge::default(),
                Bindings::spawn((
                    Cardinal::wasd_keys(),
                    Axial::left_stick(),
                )),
            )
        ]},
    ));
}

#[derive(Debug, Component)]
pub struct Player;

#[derive(Debug, InputAction)]
#[action_output(Vec2)]
struct Move;

fn apply_movement(trigger: Trigger<Fired<Move>>, mut player: Query<&mut TilePos, With<Player>>) {
    log::debug_once!("MOVE!");
    let mut player_pos = player.single_mut().expect("Expected One Player");
    let movement = trigger.value;
    if movement.x.abs() > movement.y.abs() {
        player_pos.x += 1;
    } else {
        player_pos.y += 1;
    };
}

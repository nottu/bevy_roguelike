use bevy::prelude::*;
use bevy_rand::prelude::*;

use crate::map::MapPlugin;

mod map;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        name: Some("()".to_string()),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(bevy::log::LogPlugin {
                    filter: "error,bevy_roguelike=debug".into(),
                    level: bevy::log::Level::DEBUG,
                    ..Default::default()
                }),
        )
        .add_plugins(EntropyPlugin::<WyRand>::default())
        .add_plugins(MapPlugin::default())
        .add_systems(PreStartup, setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    bevy::log::info!("Initializing Game");
    commands.spawn(Camera2d);
}

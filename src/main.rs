use bevy::prelude::*;
use bevy_rand::prelude::*;
use bevy_roguelike::*;

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
        .add_plugins(GamePlugin)
        .run();
}

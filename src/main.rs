use bevy::prelude::*;
use grabfruit::{enemy::EnemyPlugin, player::PlayerPlugin, resource::GAME_TITLE, system::SystemPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: GAME_TITLE.to_string(),
                resolution: (600., 700.).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(SystemPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(EnemyPlugin)
        .run();
}

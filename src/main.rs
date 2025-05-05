use bevy::prelude::*;

#[derive(Resource)]
struct WindowSize {
    width: f32,
    height: f32,
}

#[derive(Resource)]
struct GameShapes {
    // Player
    player_body: Handle<Mesh>,

    // Fruit
    fruit_body: Handle<Mesh>,
}

#[derive(Resource)]
struct GameColors {
    // Player
    player_body: Handle<ColorMaterial>,

    // Fruit
    fruit_body: Handle<ColorMaterial>,
}

#[derive(Resource)]
pub struct FruitSpawnTimer(pub Timer);

impl Default for FruitSpawnTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(2., TimerMode::Repeating))
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Grabfruit".to_string(),
                resolution: (600., 700.).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))

        .run();
}

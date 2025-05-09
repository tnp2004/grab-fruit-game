use bevy::prelude::*;

pub const TIME_STEP: f32 = 1. / 60.;
pub const BASE_SPEED: f32 = 500.;
pub const GAME_TITLE: &'static str = "Grabfruit";

#[derive(Resource)]
pub struct WindowSize {
    pub width: f32,
    pub height: f32,
}

#[derive(Resource)]
pub struct GameShapes {
    // Player
    pub player_body: Handle<Mesh>,

    // Enemy
    pub enemy_body: Handle<Mesh>,
}

#[derive(Resource)]
pub struct GameMaterial {
    pub player: Handle<ColorMaterial>,

    pub apple: Handle<ColorMaterial>,
    pub orange: Handle<ColorMaterial>,
    pub durian: Handle<ColorMaterial>,
}

#[derive(Resource)]
pub struct GameAssets {
    pub player: Handle<Image>,

    pub apple: Handle<Image>,
    pub durian: Handle<Image>,
    pub orange: Handle<Image>,
}

#[derive(Resource)]
pub struct FruitSpawnTimer(pub Timer);

impl Default for FruitSpawnTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(2., TimerMode::Repeating))
    }
}

#[derive(Resource)]
pub struct EnemySpawnTimer(pub Timer);

impl Default for EnemySpawnTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(2., TimerMode::Repeating))
    }
}

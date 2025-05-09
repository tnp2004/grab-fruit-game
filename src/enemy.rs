use bevy::app::Plugin;
use bevy::prelude::*;
use rand::{Rng, rng, rngs::ThreadRng};

use crate::{
    components::{Enemy, Movable, Velocity},
    resource::{EnemySpawnTimer, GameMaterial, GameShapes, WindowSize},
};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_enemy_system);
    }
}

fn spawn_enemy_system(
    mut commands: Commands,
    window_size: Res<WindowSize>,
    time: Res<Time>,
    mut timer: ResMut<EnemySpawnTimer>,
    game_material: Res<GameMaterial>,
    game_shapes: Res<GameShapes>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let mut rng: ThreadRng = rng();
        let window_width_half = window_size.width / 2.;

        let enemy = commands
            .spawn((
                Mesh2d(game_shapes.enemy_body.clone()),
                MeshMaterial2d(random_enemy_mesh(game_material)),
                Transform::from_xyz(
                    rng.random_range(-window_width_half + 50.0..window_width_half - 50.),
                    window_size.height / 2.,
                    0.,
                ),
            ))
            .insert(Enemy)
            .insert(Velocity { x: 0., y: -0.2 })
            .insert(Movable { auto_despawn: true })
            .id();

        commands.entity(enemy);
    }
}

fn random_enemy_mesh(game_material: Res<GameMaterial>) -> Handle<ColorMaterial> {
    let mut rng: ThreadRng = rng();

    let enemy_mesh_rng = match rng.random_range(0..3) {
        0 => game_material.apple.clone(),
        1 => game_material.orange.clone(),
        2 => game_material.durian.clone(),
        _ => panic!("error random enemy"),
    };

    enemy_mesh_rng
}

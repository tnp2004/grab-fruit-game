use bevy::prelude::*;

use crate::{
    components::{Movable, Player, Velocity}, GameColors, GameShapes, WindowSize
};
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_player_system);
        app.add_systems(Update, player_keyboard_event_system);
    }
}

fn spawn_player_system(
    mut commands: Commands,
    window_size: Res<WindowSize>,
    game_colors: Res<GameColors>,
    game_shapes: Res<GameShapes>,
) {
    let player = commands
        .spawn((
            Mesh2d(game_shapes.player_body.clone()),
            MeshMaterial2d(game_colors.player_body.clone()),
        ))
        .insert(Player)
        .insert(Velocity { x: 0., y: 0. })
        .insert(Movable { auto_despawn: true })
        .id();

    commands
    .entity(player)
    .insert(Transform::from_xyz(0., -window_size.height/2. + 100., 0.));
}

fn player_keyboard_event_system(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Velocity, With<Player>>,
) {
    if let Ok(mut velocity) = query.single_mut() {
        velocity.x = if input.pressed(KeyCode::ArrowLeft) {
            -1.
        } else if input.pressed(KeyCode::ArrowRight) {
            1.
        } else {
            0.
        }
    }
}

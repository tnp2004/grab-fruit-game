use bevy::{
    app::{Plugin, Startup},
    asset::Assets,
    core_pipeline::{bloom::Bloom, tonemapping::Tonemapping},
    ecs::{
        query::With,
        system::{Commands, Query, ResMut},
    },
    math::primitives::{Circle, Rectangle},
    window::{MonitorSelection, PrimaryWindow, Window, WindowPosition},
};

use bevy::prelude::*;

use crate::{
    BASE_SPEED, GameColors, GameShapes, TIME_STEP, WindowSize,
    components::{Movable, Player, Velocity},
};
pub struct SystemPlugin;

impl Plugin for SystemPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, setup_system);
        app.add_systems(Update, movement_system);
    }
}

fn setup_system(
    mut commands: Commands,
    mut primary_window: Query<&mut Window, With<PrimaryWindow>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if let Ok(mut window) = primary_window.single_mut() {
        window.position = WindowPosition::Centered(MonitorSelection::Current);

        let window_size = WindowSize {
            width: window.width(),
            height: window.height(),
        };
        commands.insert_resource(window_size);
    }

    commands.spawn((
        Camera2d,
        Camera {
            hdr: true,
            ..default()
        },
        Tonemapping::AcesFitted,
        Bloom::default(),
    ));

    let game_shapes: GameShapes = GameShapes {
        player_body: meshes.add(Rectangle::new(20., 10.)),
        fruit_body: meshes.add(Circle::new(20.)),
    };

    commands.insert_resource(game_shapes);

    let game_colors = GameColors {
        player_body: materials.add(Color::srgb(1., 1., 1.)),
        fruit_body: materials.add(Color::srgb(1., 0., 0.)),
    };

    commands.insert_resource(game_colors);
}

fn movement_system(
    mut commands: Commands,
    window_size: Res<WindowSize>,
    mut query: Query<(Entity, &Velocity, &mut Transform, &Movable, Option<&Player>)>,
) {
    for (entity, velocity, mut transform, movable, player) in query.iter_mut() {
        let translation = &mut transform.translation;
        translation.x += velocity.x * TIME_STEP * BASE_SPEED;
        translation.y += velocity.y * TIME_STEP * BASE_SPEED;

        let window_width_half = window_size.width / 2.;
        let window_height_half = window_size.height / 2.;

        if let Some(_player) = player {
            let wall_left = -window_width_half + 50.;
            let wall_right = window_width_half - 50.;

            translation.x = if translation.x <= wall_left {
                wall_left
            } else if translation.x >= wall_right {
                wall_right
            } else {
                translation.x
            }
        }

        if movable.auto_despawn {
            const MARGIN: f32 = 300.;

            if translation.y < -window_height_half - MARGIN
                || translation.y > window_height_half + MARGIN
            {
                commands.entity(entity).despawn();
            }
        }
    }
}

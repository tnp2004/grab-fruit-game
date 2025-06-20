use std::path::Path;

use bevy::{
    app::{Plugin, Startup}, asset::{AssetPath, Assets}, core_pipeline::{bloom::Bloom, tonemapping::Tonemapping}, ecs::{
        query::With,
        system::{Commands, Query, ResMut},
    }, math::{bounding::{Aabb2d, IntersectsVolume}, primitives::{Circle, Rectangle}}, platform::collections::HashSet, window::{MonitorSelection, PrimaryWindow, Window, WindowPosition}
};

use bevy::prelude::*;

use crate::{
    components::{Enemy, Movable, Player, Velocity},
    resource::{
        EnemySpawnTimer, GameAssets, GameMaterial, GameShapes, WindowSize, BASE_SPEED, TIME_STEP
    },
};
pub struct SystemPlugin;

impl Plugin for SystemPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, (load_game_assets, setup_system).chain());
        app.add_systems(Update, (movement_system, enemy_grab));
    }
}

fn load_game_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let game_assets = GameAssets {
        basket: asset_server.load(AssetPath::from_path(Path::new("player/basket.png"))),
        apple: asset_server.load(AssetPath::from_path(Path::new("enemy/apple.png"))),
        durian: asset_server.load(AssetPath::from_path(Path::new("enemy/durian.png"))),
        orange: asset_server.load(AssetPath::from_path(Path::new("enemy/orange.png"))),
    };

    commands.insert_resource(game_assets);
}

fn setup_system(
    mut commands: Commands,
    mut primary_window: Query<&mut Window, With<PrimaryWindow>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    game_assets: Res<GameAssets>,
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

    let game_shapes = GameShapes {
        player_body: meshes.add(Rectangle::new(60., 60.)),
        enemy_body: meshes.add(Rectangle::new(50., 50.)),
    };

    commands.insert_resource(game_shapes);

    let game_material = GameMaterial {
        player: materials.add(ColorMaterial {
            texture: Some(game_assets.basket.clone()),
            ..default()
        }),
        apple: materials.add(ColorMaterial {
            texture: Some(game_assets.apple.clone()),
            ..default()
        }),
        orange: materials.add(ColorMaterial {
            texture: Some(game_assets.orange.clone()),
            ..default()
        }),
        durian: materials.add(ColorMaterial {
            texture: Some(game_assets.durian.clone()),
            ..default()
        }),
    };

    commands.insert_resource(game_material);

    commands.insert_resource(EnemySpawnTimer::default());
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

fn enemy_grab(mut commands: Commands, player_query: Query<(Entity, &Transform), With<Player>>, enemy_query: Query<(Entity, &Transform), With<Enemy>>) {
    let mut check_despawn: HashSet<Entity> = HashSet::new();
    
    for (enemy_entity, enemy_transform) in enemy_query.iter() {
        if check_despawn.contains(&enemy_entity) { continue; }
        
        for (player_entity, player_transform) in player_query.iter() {
            if check_despawn.contains(&player_entity) || check_despawn.contains(&enemy_entity) { continue; }

            let collision = Aabb2d::new(enemy_transform.translation.truncate(), Vec2 { x: 40., y: 40. } / 2.)
            .intersects(&Aabb2d::new(player_transform.translation.truncate(), Vec2 { x: 50., y: 30. } / 2.));

            if collision {
                commands.entity(enemy_entity).despawn();
                check_despawn.insert(enemy_entity);
            }
        }
    }
}
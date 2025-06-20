use bevy::ecs::component::Component;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32
}

#[derive(Component)]
pub struct Movable {
    pub auto_despawn: bool
}
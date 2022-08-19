use bevy::prelude::*;

#[derive(Component)]
pub struct Health(f32);

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Friendly;

#[derive(Component)]
pub struct Enemy;

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(pub Vec2);

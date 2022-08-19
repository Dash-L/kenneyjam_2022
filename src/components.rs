use bevy::prelude::*;

#[derive(Component, Deref, DerefMut)]
pub struct Health(pub f32);

#[derive(Component, Deref, DerefMut)]
pub struct AttackTimer(pub Timer);

#[derive(Component, Deref, DerefMut)]
pub struct AttackRange(pub f32);

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Ally;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Hit(pub Option<u32>);

#[derive(Component)]
pub enum AttackType {
    Melee,
    Ranged(f32),
}

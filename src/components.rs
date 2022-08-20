use std::marker::PhantomData;

use bevy::prelude::*;
use num_derive::FromPrimitive;

#[derive(Component, Deref, DerefMut, Default)]
pub struct Health(pub f32);

#[derive(Component, Deref, DerefMut, Default)]
pub struct AttackTimer(pub Timer);
#[derive(Component, Deref, DerefMut, Default)]
pub struct AnimationTimer(pub Timer);

#[derive(Component, Deref, DerefMut, Default)]
pub struct AttackRange(pub f32);
#[derive(Component, Clone, Copy, Deref, DerefMut, Default)]
pub struct Damage(pub f32);

#[derive(Component, Default)]
pub struct Player;

#[derive(Component, Clone, Copy, Default)]
pub struct Projectile<C: Component>(pub PhantomData<C>);

#[derive(Component, Deref, DerefMut, Default)]
pub struct Velocity(pub Vec2);

#[derive(Component, Deref, DerefMut, Default)]
pub struct Speed(pub f32);

#[derive(Component, Clone, Copy, PartialEq, Eq, FromPrimitive, Default)]
pub enum EnemyType {
    #[default]
    Bat,
    EvilWizard,
    Ghost,
    Lobster,
    Rat,
    Spider,
}

#[derive(Component, Clone, Copy, PartialEq, Eq, FromPrimitive, Default)]
pub enum AllyType {
    #[default]
    Alchemist,
    Archer,
    Cyclops,
    Dwarf,
    Knight,
    Wizard,
    Player,
}

#[derive(Bundle, Default)]
pub struct PlayerBundle {
    pub velocity: Velocity,
    pub speed: Speed,
    pub _p: Player,
    #[bundle]
    pub ally: AllyBundle,
}

#[derive(Bundle, Default)]
pub struct AllyBundle {
    pub ally_type: AllyType,
    pub health: Health,
    pub damage: Damage,
    pub attack_range: AttackRange,
    pub attack_timer: AttackTimer,
    #[bundle]
    pub sprite: SpriteSheetBundle,
}

#[derive(Bundle, Default)]
pub struct EnemyBundle {
    pub enemy_type: EnemyType,
    pub health: Health,
    pub damage: Damage,
    pub attack_range: AttackRange,
    pub attack_timer: AttackTimer,
    #[bundle]
    pub sprite: SpriteSheetBundle,
}

#[derive(Bundle)]
pub struct ProjectileBundle<C: Component> {
    pub speed: Speed,
    pub velocity: Velocity,
    pub damage: Damage,
    pub projectile: Projectile<C>,
    #[bundle]
    pub sprite: SpriteBundle,
}

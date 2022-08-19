use bevy::prelude::*;

#[derive(Component, Deref, DerefMut, Default)]
pub struct Health(pub f32);

#[derive(Component, Deref, DerefMut, Default)]
pub struct AttackTimer(pub Timer);

#[derive(Component, Deref, DerefMut, Default)]
pub struct AttackRange(pub f32);

#[derive(Component, Clone, Default)]
pub enum AttackType {
    #[default]
    Melee,
    Ranged(f32, Projectile),
}

#[derive(Component, Clone, Copy, Deref, DerefMut, Default)]
pub struct Damage(pub f32);

#[derive(Component, Default)]
pub struct Player;

#[derive(Component, Default)]
pub struct Ally;

#[derive(Component, Default)]
pub struct Enemy;

#[derive(Component, Clone, Copy)]
pub enum Projectile {
    Ally,
    Enemy,
}

#[derive(Component, Deref, DerefMut, Default)]
pub struct Velocity(pub Vec2);

#[derive(Component, Deref, DerefMut, Default)]
pub struct Speed(pub f32);

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
    pub health: Health,
    pub damage: Damage,
    pub attack_type: AttackType,
    pub attack_range: AttackRange,
    pub attack_timer: AttackTimer,
    pub _a: Ally,
    #[bundle]
    pub sprite: SpriteBundle,
}

#[derive(Bundle, Default)]
pub struct EnemyBundle {
    pub health: Health,
    pub damage: Damage,
    pub attack_type: AttackType,
    pub attack_range: AttackRange,
    pub attack_timer: AttackTimer,
    pub _e: Enemy,
    #[bundle]
    pub sprite: SpriteBundle,
}

#[derive(Bundle)]
pub struct ProjectileBundle {
    pub speed: Speed,
    pub velocity: Velocity,
    pub damage: Damage,
    pub projectile: Projectile,
    #[bundle]
    pub sprite: SpriteBundle,
}

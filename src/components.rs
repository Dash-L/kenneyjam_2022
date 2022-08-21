use bevy_rapier2d::prelude::*;
use std::marker::PhantomData;

use bevy::{ecs::component, prelude::*};
use num_derive::FromPrimitive;

#[derive(Component, Default)]
pub struct Health(pub f32, pub f32);

#[derive(Component)]
pub struct HasHealthBar;

#[derive(Component)]
pub struct MainHealthBar;

#[derive(Component)]
pub struct InParty;

#[derive(Component, Default)]
pub struct PartyRadius(pub f32);

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
pub struct Projectile<C: Component>(pub bool, pub PhantomData<C>);

#[derive(Component, Deref, DerefMut, Default)]
pub struct Speed(pub f32);

#[derive(Component)]
pub struct Sound(pub Handle<AudioSource>);

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
    pub party_radius: PartyRadius,
    pub _p: Player,
    #[bundle]
    pub ally: AllyBundle,
}

#[derive(Bundle, Default)]
pub struct AllyBundle {
    pub rigid_body: RigidBody,
    pub velocity: Velocity,
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
    pub rigid_body: RigidBody,
    pub velocity: Velocity,
    pub enemy_type: EnemyType,
    pub health: Health,
    pub damage: Damage,
    pub attack_range: AttackRange,
    pub attack_timer: AttackTimer,
    #[bundle]
    pub sprite: SpriteSheetBundle,
}

#[derive(Bundle, Default)]
pub struct ProjectileBundle<C: Component> {
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub velocity: Velocity,
    pub damage: Damage,
    pub projectile: Projectile<C>,
    #[bundle]
    pub sprite: SpriteSheetBundle,
}

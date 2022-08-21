use bevy_rapier2d::prelude::*;
use std::marker::PhantomData;
use rand::{prelude::*, distributions::Standard};

use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Health(pub f32, pub f32);

#[derive(Component)]
pub struct IsDead;

#[derive(Component)]
pub struct HasHealthBar;

#[derive(Component)]
pub struct MainHealthBar;

#[derive(Component)]
pub struct InParty;

#[derive(Component, Default, Deref, DerefMut)]
pub struct IndicatorEntity(pub Option<Entity>);

#[derive(Component)]
pub struct Indicator;

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

#[derive(Component, Clone, Copy, PartialEq, Eq, Default)]
pub enum EnemyType {
    #[default]
    Bat,
    EvilWizard,
    Ghost,
    Lobster,
    Rat,
    Spider,
}

impl Distribution<EnemyType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> EnemyType {
        let val = rng.gen_range(0..16);
        match val {
            0..=3 => EnemyType::Bat,
            4..=7 => EnemyType::Rat,
            8..=10 => EnemyType::Spider,
            11..=13 => EnemyType::Lobster,
            14 => EnemyType::Ghost,
            15 => EnemyType::EvilWizard,
            _ => unreachable!()
        }
    }
}

#[derive(Component, Clone, Copy, PartialEq, Eq, Default)]
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

impl Distribution<AllyType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> AllyType {
        let val = rng.gen_range(0..17);
        match val {
            0..=4 => AllyType::Alchemist,
            5..=7 => AllyType::Dwarf,
            8..=9 => AllyType::Knight,
            10..=12 => AllyType::Cyclops,
            13..=15 => AllyType::Archer,
            16 => AllyType::Wizard,
            _ => unreachable!()
        }
    }
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
    pub indicator_entity: IndicatorEntity,
    #[bundle]
    pub sprite: SpriteSheetBundle,
}

#[derive(Bundle, Default)]
pub struct EnemyBundle {
    pub speed: Speed,
    pub rigid_body: RigidBody,
    pub velocity: Velocity,
    pub enemy_type: EnemyType,
    pub health: Health,
    pub damage: Damage,
    pub attack_range: AttackRange,
    pub attack_timer: AttackTimer,
    pub indicator_entity: IndicatorEntity,
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

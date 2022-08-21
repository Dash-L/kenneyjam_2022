use std::marker::PhantomData;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use iyes_loopless::prelude::*;

use crate::{
    components::{
        AllyType, AnimationTimer, AttackRange, AttackTimer, Damage, EnemyType, Health, Projectile,
        ProjectileBundle, Sound,
    },
    consts::PROJECTILE_SPEED,
    resources::{Sounds, Sprites},
    GameState,
};

struct AttackEvent<C>(C, Entity, Entity);

pub struct AutoBattlePlugin;

impl Plugin for AutoBattlePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AttackEvent<AllyType>>()
            .add_event::<AttackEvent<EnemyType>>()
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(GameState::InGame)
                    .with_system(auto_battle::<AllyType, EnemyType>)
                    .with_system(auto_battle::<EnemyType, AllyType>)
                    .with_system(collide_projectiles::<AllyType, EnemyType>)
                    .with_system(collide_projectiles::<EnemyType, AllyType>)
                    .with_system(handle_ally_attacks)
                    .with_system(handle_enemy_attacks)
                    .into(),
            );
    }
}

fn auto_battle<A, T>(
    time: Res<Time>,
    mut attack_events: EventWriter<AttackEvent<A>>,
    mut attackers: Query<(Entity, &Transform, &AttackRange, &mut AttackTimer, &A), Without<T>>,
    targets: Query<(Entity, &Transform), (With<T>, Without<A>)>,
) where
    A: Component + Clone,
    T: Component,
{
    for (attacker_entity, attacker_transform, range, mut timer, ty) in &mut attackers {
        let mut closest = (f32::MAX, Entity::from_raw(0), Entity::from_raw(0));
        for (target_entity, target_transform) in &targets {
            let dist = attacker_transform
                .translation
                .truncate()
                .distance(target_transform.translation.truncate());
            if dist < closest.0 {
                closest = (dist, attacker_entity, target_entity);
            }
        }

        timer.tick(time.delta());

        if timer.just_finished() && closest.0 <= range.0 {
            attack_events.send(AttackEvent(ty.clone(), closest.1, closest.2));
        }
    }
}

fn collide_projectiles<A, T>(
    mut commands: Commands,
    audio: Res<Audio>,
    mut projectiles: Query<(
        &Damage,
        &mut Projectile<A>,
        Option<&AnimationTimer>,
        &Sound,
        Option<&mut Velocity>,
    )>,
    mut targets: Query<&mut Health, With<T>>,
    mut collision_events: EventReader<CollisionEvent>,
) where
    A: Component,
    T: Component,
{
    let mut already_processed = Vec::new();
    for event in collision_events.iter() {
        if let CollisionEvent::Started(e1, e2, _) = event {
            if !already_processed.contains(e1) && !already_processed.contains(e2) {
                if let Ok((damage, mut projectile, animation_timer, sound, vel)) =
                    projectiles.get_mut(*e1)
                {
                    if projectile.0 {
                        if let Ok(mut health) = targets.get_mut(*e2) {
                            already_processed.push(*e1);
                            projectile.0 = false;
                            health.0 -= damage.0;
                            if let Some(mut vel) = vel {
                                vel.linvel = Vec2::ZERO;
                            }
                            audio.play_with_settings(sound.0.clone(), PlaybackSettings::ONCE.with_volume(0.3));
                            if animation_timer.is_none() {
                                commands
                                    .entity(*e1)
                                    .insert(AnimationTimer(Timer::from_seconds(0.1, true)));
                            }
                        }
                    }
                } else if let Ok((damage, mut projectile, animation_timer, sound, vel)) =
                    projectiles.get_mut(*e2)
                {
                    if projectile.0 {
                        if let Ok(mut health) = targets.get_mut(*e1) {
                            already_processed.push(*e2);
                            projectile.0 = false;
                            health.0 -= damage.0;
                            if let Some(mut vel) = vel {
                                vel.linvel = Vec2::ZERO;
                            }
                            audio.play_with_settings(sound.0.clone(), PlaybackSettings::ONCE.with_volume(0.3));
                            if animation_timer.is_none() {
                                commands
                                    .entity(*e2)
                                    .insert(AnimationTimer(Timer::from_seconds(0.1, true)));
                            }
                        }
                    }
                }
            }
        }
    }
}

fn handle_ally_attacks(
    mut commands: Commands,
    sprites: Res<Sprites>,
    sounds: Res<Sounds>,
    mut attack_events: EventReader<AttackEvent<AllyType>>,
    allies: Query<(&Transform, &Damage, &AllyType)>,
    enemies: Query<&Transform, With<EnemyType>>,
) {
    for AttackEvent(_, ally_entity, enemy_entity) in attack_events.iter() {
        if let Ok((ally_transform, damage, ally_type)) = allies.get(*ally_entity) {
            if let Ok(enemy_transform) = enemies.get(*enemy_entity) {
                match ally_type {
                    AllyType::Archer => {
                        let dir = (enemy_transform.translation.truncate()
                            - ally_transform.translation.truncate())
                        .normalize();

                        commands
                            .spawn_bundle(ProjectileBundle {
                                velocity: Velocity {
                                    linvel: dir * PROJECTILE_SPEED,
                                    ..default()
                                },
                                damage: Damage(damage.0),
                                projectile: Projectile::<AllyType>(true, PhantomData),
                                sprite: SpriteSheetBundle {
                                    texture_atlas: sprites.arrow.clone(),
                                    transform: Transform::from_translation(
                                        ally_transform.translation,
                                    )
                                    .with_rotation(Quat::from_rotation_z(
                                        Vec2::Y.angle_between(dir),
                                    ))
                                    .with_scale(Vec3::splat(1.5)),
                                    ..default()
                                },
                                collider: Collider::cuboid(4.0, 8.0),
                                ..default()
                            })
                            .insert(Sensor)
                            .insert(ActiveEvents::COLLISION_EVENTS)
                            .insert(Sound(sounds.arrow.clone()));
                    }
                    AllyType::Wizard => {
                        commands
                            .spawn_bundle(ProjectileBundle {
                                velocity: Velocity {
                                    linvel: (enemy_transform.translation.truncate()
                                        - ally_transform.translation.truncate())
                                    .normalize()
                                        * PROJECTILE_SPEED,
                                    ..default()
                                },
                                damage: Damage(damage.0),
                                projectile: Projectile::<AllyType>(true, PhantomData),
                                sprite: SpriteSheetBundle {
                                    texture_atlas: sprites.fireball.clone(),
                                    transform: Transform::from_translation(
                                        ally_transform.translation,
                                    )
                                    .with_scale(Vec3::splat(2.5)),
                                    ..default()
                                },
                                collider: Collider::cuboid(4.0, 4.0),
                                ..default()
                            })
                            .insert(Sensor)
                            .insert(ActiveEvents::COLLISION_EVENTS)
                            .insert(Sound(sounds.fireball.clone()));
                    }
                    _ => {
                        let dir = (enemy_transform.translation.truncate()
                            - ally_transform.translation.truncate())
                        .normalize();
                        commands
                            .spawn_bundle(SpriteSheetBundle {
                                texture_atlas: sprites.slash.clone(),
                                transform: Transform::from_translation(
                                    enemy_transform.translation - dir.extend(0.),
                                )
                                .with_scale(Vec3::splat(4.))
                                .with_rotation(
                                    Quat::from_rotation_z(Vec2::NEG_Y.angle_between(dir)),
                                ),
                                ..default()
                            })
                            .insert(AnimationTimer(Timer::from_seconds(0.03, true)))
                            .insert(Collider::cuboid(5., 2.))
                            .insert(Sensor)
                            .insert(ActiveEvents::COLLISION_EVENTS)
                            .insert(Projectile::<AllyType>(true, PhantomData))
                            .insert(Damage(damage.0))
                            .insert(Sound(sounds.slash.clone()));
                    }
                }
            }
        }
    }
}

fn handle_enemy_attacks(
    mut commands: Commands,
    sprites: Res<Sprites>,
    sounds: Res<Sounds>,
    mut attack_events: EventReader<AttackEvent<EnemyType>>,
    enemies: Query<(&Transform, &Damage, &EnemyType)>,
    allies: Query<&Transform, With<AllyType>>,
) {
    for AttackEvent(_, enemy_entity, ally_entity) in attack_events.iter() {
        if let Ok((enemy_transform, damage, enemy_type)) = enemies.get(*enemy_entity) {
            if let Ok(ally_transform) = allies.get(*ally_entity) {
                match enemy_type {
                    EnemyType::EvilWizard => {
                        commands
                            .spawn_bundle(ProjectileBundle {
                                velocity: Velocity {
                                    linvel: (ally_transform.translation.truncate()
                                        - enemy_transform.translation.truncate())
                                    .normalize()
                                        * PROJECTILE_SPEED,
                                    ..default()
                                },
                                damage: Damage(damage.0),
                                projectile: Projectile::<EnemyType>(true, PhantomData),
                                sprite: SpriteSheetBundle {
                                    texture_atlas: sprites.fireball.clone(),
                                    transform: Transform::from_translation(
                                        enemy_transform.translation,
                                    )
                                    .with_scale(Vec3::splat(2.5)),
                                    ..default()
                                },
                                collider: Collider::cuboid(4.0, 4.0),
                                ..default()
                            })
                            .insert(Sensor)
                            .insert(ActiveEvents::COLLISION_EVENTS)
                            .insert(Sound(sounds.fireball.clone()));
                    }
                    _ => {
                        commands
                            .spawn_bundle(SpriteSheetBundle {
                                texture_atlas: sprites.slash.clone(),
                                transform: *ally_transform,
                                visibility: Visibility { is_visible: false },
                                ..default()
                            })
                            .insert(AnimationTimer(Timer::from_seconds(0.001, true)))
                            .insert(Collider::cuboid(5., 2.))
                            .insert(Sensor)
                            .insert(ActiveEvents::COLLISION_EVENTS)
                            .insert(Projectile::<EnemyType>(true, PhantomData))
                            .insert(Damage(damage.0))
                            .insert(Sound(sounds.enemy_hit.clone()));
                    }
                }
            }
        }
    }
}

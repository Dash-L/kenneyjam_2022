use bevy::{prelude::*, sprite::collide_aabb::collide};
use iyes_loopless::prelude::*;

use crate::{
    components::{
        AllyType, AttackRange, AttackTimer, Collider, Damage, EnemyType, Health, Projectile,
        ProjectileBundle, Speed, Velocity,
    },
    resources::Sprites,
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
                    .with_system(move_projectiles)
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
    projectiles: Query<(Entity, &Transform, &Damage, &Collider), With<Projectile<A>>>,
    mut targets: Query<(&Transform, &mut Health, &Collider), With<T>>,
) where
    A: Component,
    T: Component,
{
    for (entity, projectile_transform, damage, projectile_collider) in &projectiles {
        for (target_transform, mut health, target_collider) in &mut targets {
            if collide(
                projectile_transform.translation,
                projectile_collider.0,
                target_transform.translation,
                target_collider.0,
            )
            .is_some()
            {
                health.0 -= **damage;
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}

fn handle_ally_attacks(
    mut commands: Commands,
    sprites: Res<Sprites>,
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
                                speed: Speed(20.0),
                                velocity: Velocity(dir),
                                damage: Damage(damage.0),
                                projectile: Projectile::<AllyType>::default(),
                                sprite: SpriteBundle {
                                    texture: sprites.arrow.clone(),
                                    transform: Transform::from_translation(
                                        ally_transform.translation,
                                    )
                                    .with_rotation(Quat::from_rotation_z(
                                        Vec2::Y.angle_between(dir),
                                    ))
                                    .with_scale(Vec3::splat(1.5)),
                                    ..default()
                                },
                            })
                            .insert(Collider(Vec2::new(8., 16.) * 1.5));
                    }
                    AllyType::Wizard => {
                        commands.spawn_bundle(ProjectileBundle {
                            speed: Speed(20.0),
                            velocity: Velocity(
                                (enemy_transform.translation.truncate()
                                    - ally_transform.translation.truncate())
                                .normalize(),
                            ),
                            damage: Damage(damage.0),
                            projectile: Projectile::<AllyType>::default(),
                            sprite: SpriteBundle {
                                texture: sprites.fireball.clone(),
                                transform: Transform::from_translation(ally_transform.translation),
                                ..default()
                            },
                        });
                    }
                    _ => {}
                }
            }
        }
    }
}

fn handle_enemy_attacks(
    mut commands: Commands,
    sprites: Res<Sprites>,
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
                                speed: Speed(20.0),
                                velocity: Velocity(
                                    (ally_transform.translation.truncate()
                                        - enemy_transform.translation.truncate())
                                    .normalize(),
                                ),
                                damage: Damage(damage.0),
                                projectile: Projectile::<EnemyType>::default(),
                                sprite: SpriteBundle {
                                    texture: sprites.fireball.clone(),
                                    transform: Transform::from_translation(
                                        enemy_transform.translation,
                                    )
                                    .with_scale(Vec3::splat(1.5)),
                                    ..default()
                                },
                            })
                            .insert(Collider(Vec2::new(8., 8.) * 1.5));
                    }
                    _ => {}
                }
            }
        }
    }
}

// fn auto_battle(
//     mut attack_ev: EventWriter<AttackEvent>,
//     time: Res<Time>,
//     mut allies: Query<
//         (
//             Entity,
//             &Transform,
//             &Damage,
//             &mut AttackTimer,
//             &AttackRange,
//             &AttackType,
//         ),
//         (With<Ally>, Without<Enemy>),
//     >,
//     mut enemies: Query<
//         (
//             Entity,
//             &Transform,
//             &Damage,
//             &mut AttackTimer,
//             &AttackRange,
//             &AttackType,
//         ),
//         (With<Enemy>, Without<Ally>),
//     >,
// ) {
//     // TODO: target closest instead of random
//     for (ally_entity, ally_transform, ally_damage, mut ally_timer, ally_range, ally_attack) in
//         &mut allies
//     {
//         for (
//             enemy_entity,
//             enemy_transform,
//             enemy_damage,
//             mut enemy_timer,
//             enemy_range,
//             enemy_attack,
//         ) in &mut enemies
//         {
//             do_attack(
//                 &mut attack_ev,
//                 &time,
//                 ally_transform,
//                 ally_damage,
//                 &mut ally_timer,
//                 ally_range,
//                 ally_attack,
//                 enemy_entity,
//                 enemy_transform,
//             );
//             do_attack(
//                 &mut attack_ev,
//                 &time,
//                 enemy_transform,
//                 enemy_damage,
//                 &mut enemy_timer,
//                 enemy_range,
//                 enemy_attack,
//                 ally_entity,
//                 ally_transform,
//             );
//         }
//     }
// }

fn move_projectiles(
    mut projectiles: Query<
        (&mut Transform, &Velocity, &Speed),
        Or<(With<Projectile<AllyType>>, With<Projectile<EnemyType>>)>,
    >,
) {
    for (mut transform, velocity, speed) in &mut projectiles {
        transform.translation += (velocity.0 * speed.0).extend(0.0);
    }
}

// fn handle_attacks(
//     mut commands: Commands,
//     sprites: Res<Sprites>,
//     mut attack_ev: EventReader<AttackEvent>,
//     mut entities: Query<&mut Health>,
// ) {
//     for AttackEvent(entity, attack_type, damage, from_transform, to_transform) in attack_ev.iter() {
//         match attack_type {
//             AttackType::Melee => {
//                 if let Ok(mut health) = entities.get_mut(*entity) {
//                     health.0 -= damage;
//                     break;
//                 }
//             }
//             AttackType::Ranged(speed, projectile) => {
//                 commands.spawn_bundle(ProjectileBundle {
//                     speed: Speed(*speed),
//                     velocity: Velocity(
//                         (to_transform.translation.truncate()
//                             - from_transform.translation.truncate())
//                         .normalize(),
//                     ),
//                     damage: Damage(*damage),
//                     projectile: *projectile,
//                     sprite: SpriteBundle {
//                         texture: sprites.cactus.clone(),
//                         transform: Transform::from_translation(from_transform.translation),
//                         ..default()
//                     },
//                 });
//             }
//         }
//     }
// }

// fn do_attack(
//     attack_ev: &mut EventWriter<AttackEvent>,
//     time: &Time,
//     attacker_transform: &Transform,
//     damage: &Damage,
//     timer: &mut AttackTimer,
//     range: &AttackRange,
//     attack_type: &AttackType,
//     target_entity: Entity,
//     target_transform: &Transform,
// ) {
//     let dist = attacker_transform
//         .translation
//         .distance(target_transform.translation);
//
//     timer.tick(time.delta());
//     if dist <= **range {
//         if timer.just_finished() {
//             attack_ev.send(AttackEvent(
//                 target_entity,
//                 attack_type.clone(),
//                 **damage,
//                 *attacker_transform,
//                 *target_transform,
//             ));
//         }
//     }
// }

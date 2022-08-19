use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{
    components::{
        Ally, AttackRange, AttackTimer, AttackType, Damage, Enemy, Health, Projectile,
        ProjectileBundle, Speed, Velocity,
    },
    resources::Sprites,
    GameState, InGameState,
};

struct AttackEvent(Entity, AttackType, f32, Transform, Transform);

pub struct AutoBattlePlugin;

impl Plugin for AutoBattlePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AttackEvent>()
            .add_exit_system(GameState::InGame(InGameState::Wave), despawn_projectiles)
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(GameState::InGame(InGameState::DownTime))
                    .with_system(auto_battle::<Ally, Enemy>)
                    .with_system(auto_battle::<Enemy, Ally>)
                    .with_system(move_projectiles)
                    .with_system(handle_attacks)
                    .into(),
            );
    }
}

fn auto_battle<A, T>(
    attackers: Query<(Entity, &Transform, &AttackRange), (With<A>, Without<T>)>,
    targets: Query<(Entity, &Transform), (With<T>, Without<A>)>,
) where
    A: Component,
    T: Component,
{
    for (attacker_entity, attacker_transform, range) in &attackers {
        let mut closest = (
            f32::MAX,
            Entity::from_raw(0),
            Transform::default(),
            Entity::from_raw(0),
            Transform::default(),
        );
        for (target_entity, target_transform) in &targets {
            let dist = attacker_transform
                .translation
                .truncate()
                .distance(target_transform.translation.truncate());
            if dist < closest.0 {
                closest = (
                    dist,
                    attacker_entity,
                    *attacker_transform,
                    target_entity,
                    *target_transform,
                );
            }
        }
        if closest.0 <= range.0 {
            // spawn attack event of some kind
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

fn move_projectiles(mut projectiles: Query<(&mut Transform, &Velocity, &Speed), With<Projectile>>) {
    for (mut transform, velocity, speed) in &mut projectiles {
        transform.translation += (velocity.0 * speed.0).extend(0.0);
    }
}

fn collide_projectiles(
    mut commands: Commands,
    mut entites: Query<(&Transform, &mut Health, Option<&Ally>, Option<&Enemy>)>,
    projectiles: Query<(&Transform, &Projectile)>,
) {
}

fn handle_attacks(
    mut commands: Commands,
    sprites: Res<Sprites>,
    mut attack_ev: EventReader<AttackEvent>,
    mut entities: Query<&mut Health>,
) {
    for AttackEvent(entity, attack_type, damage, from_transform, to_transform) in attack_ev.iter() {
        match attack_type {
            AttackType::Melee => {
                if let Ok(mut health) = entities.get_mut(*entity) {
                    health.0 -= damage;
                    break;
                }
            }
            AttackType::Ranged(speed, projectile) => {
                commands.spawn_bundle(ProjectileBundle {
                    speed: Speed(*speed),
                    velocity: Velocity(
                        (to_transform.translation.truncate()
                            - from_transform.translation.truncate())
                        .normalize(),
                    ),
                    damage: Damage(*damage),
                    projectile: *projectile,
                    sprite: SpriteBundle {
                        texture: sprites.cactus.clone(),
                        transform: Transform::from_translation(from_transform.translation),
                        ..default()
                    },
                });
            }
        }
    }
}

fn despawn_projectiles(mut commands: Commands, projectiles: Query<Entity, With<Projectile>>) {
    for entity in &projectiles {
        commands.entity(entity).despawn();
    }
}

fn do_attack(
    attack_ev: &mut EventWriter<AttackEvent>,
    time: &Time,
    attacker_transform: &Transform,
    damage: &Damage,
    timer: &mut AttackTimer,
    range: &AttackRange,
    attack_type: &AttackType,
    target_entity: Entity,
    target_transform: &Transform,
) {
    let dist = attacker_transform
        .translation
        .distance(target_transform.translation);

    timer.tick(time.delta());
    if dist <= **range {
        if timer.just_finished() {
            attack_ev.send(AttackEvent(
                target_entity,
                attack_type.clone(),
                **damage,
                *attacker_transform,
                *target_transform,
            ));
        }
    }
}

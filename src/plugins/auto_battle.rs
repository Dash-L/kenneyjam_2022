use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{
    components::{
        AllyType, AttackRange, AttackTimer, Damage, EnemyType, Health, Projectile,
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
                    .with_system(handle_archer_attack)
                    .with_system(handle_cyclops_attack)
                    .with_system(handle_dwarf_attack)
                    .with_system(handle_knight_attack)
                    .with_system(handle_player_attack)
                    .with_system(handle_wizard_attack)
                    .with_system(handle_bat_attack)
                    .with_system(handle_evil_wizard_attack)
                    .with_system(handle_ghost_attack)
                    .with_system(handle_lobster_attack)
                    .with_system(handle_rat_attack)
                    .with_system(handle_spider_attack)
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
    projectiles: Query<(Entity, &Transform), With<Projectile<A>>>,
    mut targets: Query<(&Transform, &mut Health), With<T>>,
) where
    A: Component,
    T: Component,
{
}

fn handle_cyclops_attack(
    mut attack_events: EventReader<AttackEvent<AllyType>>,
    cyclops_q: Query<(&Transform, &Damage), With<AllyType>>,
    enemy_q: Query<&Transform, With<EnemyType>>,
) {
    for AttackEvent(_, cyclops_entity, enemy_entity) in
        attack_events.iter().filter(|ev| ev.0 == AllyType::Cyclops)
    {
        let cyclops = cyclops_q.get(*cyclops_entity);
        let enemy = enemy_q.get(*enemy_entity);
    }
}

fn handle_archer_attack(
    mut commands: Commands,
    sprites: Res<Sprites>,
    mut attack_events: EventReader<AttackEvent<AllyType>>,
    archer_q: Query<(&Transform, &Damage), With<AllyType>>,
    enemy_q: Query<&Transform, With<EnemyType>>,
) {
    for AttackEvent(_, archer_entity, enemy_entity) in
        attack_events.iter().filter(|ev| ev.0 == AllyType::Archer)
    {
        let (transform, damage) = archer_q.get(*archer_entity).unwrap();
        let other_transform = enemy_q.get(*enemy_entity).unwrap();

        let dir =
            (other_transform.translation.truncate() - transform.translation.truncate()).normalize();

        commands.spawn_bundle(ProjectileBundle {
            speed: Speed(20.0),
            velocity: Velocity(dir),
            damage: Damage(damage.0),
            projectile: Projectile::<AllyType>::default(),
            sprite: SpriteBundle {
                texture: sprites.arrow.clone(),
                transform: Transform::from_translation(transform.translation)
                    .with_rotation(Quat::from_rotation_z(Vec2::Y.angle_between(dir)))
                    .with_scale(Vec3::splat(1.5)),
                ..default()
            },
        });
    }
}

fn handle_dwarf_attack(
    mut attack_events: EventReader<AttackEvent<AllyType>>,
    dwarf_q: Query<(&Transform, &Damage), With<AllyType>>,
    enemy_q: Query<&Transform, With<EnemyType>>,
) {
    for AttackEvent(_, dwarf_entity, enemy_entity) in
        attack_events.iter().filter(|ev| ev.0 == AllyType::Dwarf)
    {
        let dwarf = dwarf_q.get(*dwarf_entity);
        let enemy = enemy_q.get(*enemy_entity);
    }
}

fn handle_knight_attack(
    mut attack_events: EventReader<AttackEvent<AllyType>>,
    knight_q: Query<(&Transform, &Damage), With<AllyType>>,
    enemy_q: Query<&Transform, With<EnemyType>>,
) {
    for AttackEvent(_, knight_entity, enemy_entity) in
        attack_events.iter().filter(|ev| ev.0 == AllyType::Knight)
    {
        let knight = knight_q.get(*knight_entity);
        let enemy = enemy_q.get(*enemy_entity);
    }
}

fn handle_player_attack(
    mut attack_events: EventReader<AttackEvent<AllyType>>,
    player_q: Query<(&Transform, &Damage), With<AllyType>>,
    enemy_q: Query<&Transform, With<EnemyType>>,
) {
    for AttackEvent(_, player_entity, enemy_entity) in
        attack_events.iter().filter(|ev| ev.0 == AllyType::Player)
    {
        let player = player_q.get(*player_entity);
        let enemy = enemy_q.get(*enemy_entity);
    }
}

fn handle_wizard_attack(
    mut commands: Commands,
    sprites: Res<Sprites>,
    mut attack_events: EventReader<AttackEvent<AllyType>>,
    wizard_q: Query<(&Transform, &Damage), With<AllyType>>,
    enemy_q: Query<&Transform, With<EnemyType>>,
) {
    for AttackEvent(_, wizard_entity, enemy_entity) in
        attack_events.iter().filter(|ev| ev.0 == AllyType::Wizard)
    {
        let (transform, damage) = wizard_q.get(*wizard_entity).unwrap();
        let other_transform = enemy_q.get(*enemy_entity).unwrap();

        commands.spawn_bundle(ProjectileBundle {
            speed: Speed(20.0),
            velocity: Velocity(
                (other_transform.translation.truncate() - transform.translation.truncate())
                    .normalize(),
            ),
            damage: Damage(damage.0),
            projectile: Projectile::<AllyType>::default(),
            sprite: SpriteBundle {
                texture: sprites.fireball.clone(),
                transform: Transform::from_translation(transform.translation),
                ..default()
            },
        });
    }
}

fn handle_bat_attack(
    mut attack_events: EventReader<AttackEvent<EnemyType>>,
    bat_q: Query<(&Transform, &Damage), With<EnemyType>>,
    ally_q: Query<&Transform, With<AllyType>>,
) {
    for AttackEvent(_, bat_entity, ally_entity) in
        attack_events.iter().filter(|ev| ev.0 == EnemyType::Bat)
    {
        let bat = bat_q.get(*bat_entity);
        let ally = ally_q.get(*ally_entity);
    }
}

fn handle_evil_wizard_attack(
    mut commands: Commands,
    sprites: Res<Sprites>,
    mut attack_events: EventReader<AttackEvent<EnemyType>>,
    evil_wizard_q: Query<(&Transform, &Damage), With<EnemyType>>,
    ally_q: Query<&Transform, With<AllyType>>,
) {
    for AttackEvent(_, evil_wizard_entity, ally_entity) in attack_events
        .iter()
        .filter(|ev| ev.0 == EnemyType::EvilWizard)
    {
        let (transform, damage) = evil_wizard_q.get(*evil_wizard_entity).unwrap();
        let other_transform = ally_q.get(*ally_entity).unwrap();

        commands.spawn_bundle(ProjectileBundle {
            speed: Speed(20.0),
            velocity: Velocity(
                (other_transform.translation.truncate() - transform.translation.truncate())
                    .normalize(),
            ),
            damage: Damage(damage.0),
            projectile: Projectile::<EnemyType>::default(),
            sprite: SpriteBundle {
                texture: sprites.fireball.clone(),
                transform: Transform::from_translation(transform.translation),
                ..default()
            },
        });
    }
}

fn handle_ghost_attack(
    mut attack_events: EventReader<AttackEvent<EnemyType>>,
    ghost_q: Query<(&Transform, &Damage), With<EnemyType>>,
    ally_q: Query<&Transform, With<AllyType>>,
) {
    for AttackEvent(_, ghost_entity, ally_entity) in
        attack_events.iter().filter(|ev| ev.0 == EnemyType::Ghost)
    {
        let ghost = ghost_q.get(*ghost_entity);
        let ally = ally_q.get(*ally_entity);
    }
}

fn handle_lobster_attack(
    mut attack_events: EventReader<AttackEvent<EnemyType>>,
    lobster_q: Query<(&Transform, &Damage), With<EnemyType>>,
    ally_q: Query<&Transform, With<AllyType>>,
) {
    for AttackEvent(_, lobster_entity, ally_entity) in
        attack_events.iter().filter(|ev| ev.0 == EnemyType::Lobster)
    {
        let lobster = lobster_q.get(*lobster_entity);
        let ally = ally_q.get(*ally_entity);
    }
}

fn handle_rat_attack(
    mut attack_events: EventReader<AttackEvent<EnemyType>>,
    rat_q: Query<(&Transform, &Damage), With<EnemyType>>,
    ally_q: Query<&Transform, With<AllyType>>,
) {
    for AttackEvent(_, rat_entity, ally_entity) in
        attack_events.iter().filter(|ev| ev.0 == EnemyType::Rat)
    {
        let rat = rat_q.get(*rat_entity);
        let ally = ally_q.get(*ally_entity);
    }
}

fn handle_spider_attack(
    mut attack_events: EventReader<AttackEvent<EnemyType>>,
    spider_q: Query<(&Transform, &Damage), With<EnemyType>>,
    ally_q: Query<&Transform, With<AllyType>>,
) {
    for AttackEvent(_, spider_entity, ally_entity) in
        attack_events.iter().filter(|ev| ev.0 == EnemyType::Spider)
    {
        let spider = spider_q.get(*spider_entity);
        let ally = ally_q.get(*ally_entity);
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

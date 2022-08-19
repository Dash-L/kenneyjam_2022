use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{
    components::{
        Ally, Attack, AttackRange, AttackTimer, AttackType, Damage, Enemy, Health, Projectile,
        ProjectileBundle, Speed, Velocity,
    },
    resources::Sprites,
    GameState, InGameState,
};

pub struct AutoBattlePlugin;

impl Plugin for AutoBattlePlugin {
    fn build(&self, app: &mut App) {
        app.add_exit_system(GameState::InGame(InGameState::Wave), despawn_projectiles)
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(GameState::InGame(InGameState::Wave))
                    .with_system(auto_battle)
                    .with_system(move_projectiles)
                    .with_system(handle_attacks)
                    .into(),
            );
    }
}

fn auto_battle(
    mut commands: Commands,
    sprites: Res<Sprites>,
    time: Res<Time>,
    mut allies: Query<
        (
            Entity,
            &Transform,
            &Damage,
            &mut AttackTimer,
            &AttackRange,
            &AttackType,
        ),
        (With<Ally>, Without<Enemy>),
    >,
    mut enemies: Query<
        (
            Entity,
            &Transform,
            &Damage,
            &mut AttackTimer,
            &AttackRange,
            &AttackType,
        ),
        (With<Enemy>, Without<Ally>),
    >,
) {
    // TODO: target closest instead of random
    for (ally_entity, ally_transform, ally_damage, mut ally_timer, ally_range, ally_attack) in
        &mut allies
    {
        for (
            enemy_entity,
            enemy_transform,
            enemy_damage,
            mut enemy_timer,
            enemy_range,
            enemy_attack,
        ) in &mut enemies
        {
            do_attack(
                &mut commands,
                sprites.clone(),
                &time,
                ally_transform,
                ally_damage,
                &mut ally_timer,
                ally_range,
                ally_attack,
                enemy_entity,
                enemy_transform,
            );
            do_attack(
                &mut commands,
                sprites.clone(),
                &time,
                enemy_transform,
                enemy_damage,
                &mut enemy_timer,
                enemy_range,
                enemy_attack,
                ally_entity,
                ally_transform,
            );
        }
    }
}

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
    mut entities: Query<&mut Health>,
    attacks: Query<(Entity, &Attack, &Damage)>,
) {
    for (entity, attack, damage) in &attacks {
        if let Ok(mut health) = entities.get_mut(attack.0) {
            health.0 -= damage.0;
            commands.entity(entity).despawn();
            break;
        }
    }
}

fn despawn_projectiles(mut commands: Commands, projectiles: Query<Entity, With<Projectile>>) {
    for entity in &projectiles {
        commands.entity(entity).despawn();
    }
}

fn do_attack(
    commands: &mut Commands,
    sprites: Sprites,
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

    if dist <= **range {
        timer.tick(time.delta());
        if timer.just_finished() {
            match attack_type {
                AttackType::Melee => {
                    commands
                        .spawn()
                        .insert(Attack(target_entity))
                        .insert(*damage);
                }
                AttackType::Ranged(speed, projectile) => {
                    commands.spawn_bundle(ProjectileBundle {
                        speed: Speed(*speed),
                        velocity: Velocity(
                            (target_transform.translation.truncate()
                                - attacker_transform.translation.truncate())
                            .normalize(),
                        ),
                        damage: damage.clone(),
                        projectile: *projectile,
                        sprite: SpriteBundle {
                            texture: sprites.cactus.clone(),
                            transform: Transform::from_translation(attacker_transform.translation),
                            ..default()
                        },
                    });
                }
            }
        }
    }
}

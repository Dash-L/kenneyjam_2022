use crate::{
    components::{AllyBundle, AttackRange, AttackTimer, Damage, EnemyBundle, Health},
    consts::{HEIGHT, SPRITE_SCALE, WIDTH},
    resources::{AllyCount, AllySpawnTimer, EnemiesCount, EnemySpawnTimer, Sprites},
    AllyType, EnemyType, GameState, InGameState,
};
use bevy::prelude::*;
use iyes_loopless::prelude::*;
use num_traits::FromPrimitive;
use rand::prelude::*;
pub struct SpawnPlugin;

impl Plugin for SpawnPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EnemySpawnTimer(Timer::from_seconds(1.0, true)))
            .insert_resource(AllySpawnTimer(Timer::from_seconds(1.0, true)))
            .insert_resource(EnemiesCount(0))
            .insert_resource(AllyCount(0))
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(GameState::InGame(InGameState::DownTime))
                    .with_system(spawn_wave)
                    .with_system(spawn_allies)
                    .into(),
            );
    }
}
fn spawn_allies(
    mut commands: Commands,
    sprites: Res<Sprites>,
    time: Res<Time>,
    mut spawn_timer: ResMut<AllySpawnTimer>,
    mut ally_count: ResMut<AllyCount>,
) {
    spawn_timer.tick(time.delta());

    if spawn_timer.just_finished() && **ally_count < 1 {
        let mut rng = rand::thread_rng();

        let transform =
            Transform::from_scale(Vec3::splat(SPRITE_SCALE)).with_translation(Vec3::new(
                rng.gen_range((-WIDTH as i32 / 2)..(WIDTH as i32 / 2)) as f32,
                rng.gen_range((-HEIGHT as i32 / 2)..(HEIGHT as i32 / 2)) as f32,
                850.,
            ));

        println!("Ally Spawned!");
        let ally_type = AllyType::from_u32(rng.gen_range(0..6)).unwrap();

        match ally_type {
            AllyType::Alchemist => commands.spawn_bundle(AllyBundle {
                ally_type,
                health: Health(100.0),
                damage: Damage(25.0),
                attack_range: AttackRange(1000.),
                attack_timer: AttackTimer(Timer::from_seconds(1.0, true)),
                sprite: SpriteSheetBundle {
                    texture_atlas: sprites.alchemist.clone(),
                    transform,
                    ..default()
                },
                ..default()
            }),
            AllyType::Archer => commands.spawn_bundle(AllyBundle {
                ally_type,
                health: Health(100.0),
                damage: Damage(25.0),
                attack_range: AttackRange(1000.),
                attack_timer: AttackTimer(Timer::from_seconds(1.0, true)),
                sprite: SpriteSheetBundle {
                    texture_atlas: sprites.archer.clone(),
                    transform,
                    ..default()
                },
                ..default()
            }),
            AllyType::Cyclops => commands.spawn_bundle(AllyBundle {
                ally_type,
                health: Health(100.0),
                damage: Damage(25.0),
                attack_range: AttackRange(1000.),
                attack_timer: AttackTimer(Timer::from_seconds(1.0, true)),
                sprite: SpriteSheetBundle {
                    texture_atlas: sprites.cyclops.clone(),
                    transform,
                    ..default()
                },
                ..default()
            }),
            AllyType::Dwarf => commands.spawn_bundle(AllyBundle {
                ally_type,
                health: Health(100.0),
                damage: Damage(25.0),
                attack_range: AttackRange(1000.),
                attack_timer: AttackTimer(Timer::from_seconds(1.0, true)),
                sprite: SpriteSheetBundle {
                    texture_atlas: sprites.dwarf.clone(),
                    transform,
                    ..default()
                },
                ..default()
            }),
            AllyType::Knight => commands.spawn_bundle(AllyBundle {
                ally_type,
                health: Health(100.0),
                damage: Damage(25.0),
                attack_range: AttackRange(1000.),
                attack_timer: AttackTimer(Timer::from_seconds(1.0, true)),
                sprite: SpriteSheetBundle {
                    texture_atlas: sprites.knight.clone(),
                    transform,
                    ..default()
                },
                ..default()
            }),
            AllyType::Wizard => commands.spawn_bundle(AllyBundle {
                ally_type,
                health: Health(100.0),
                damage: Damage(25.0),
                attack_range: AttackRange(1000.),
                attack_timer: AttackTimer(Timer::from_seconds(1.0, true)),
                sprite: SpriteSheetBundle {
                    texture_atlas: sprites.wizard.clone(),
                    transform,
                    ..default()
                },
                ..default()
            }),
            AllyType::Player => unreachable!(),
        };
        **ally_count += 1;
    }
}

fn spawn_wave(
    mut commands: Commands,
    sprites: Res<Sprites>,
    time: Res<Time>,
    mut spawn_timer: ResMut<EnemySpawnTimer>,
    mut enemy_count: ResMut<EnemiesCount>,
) {
    spawn_timer.tick(time.delta());

    if spawn_timer.just_finished() && **enemy_count < 5 {
        let mut rng = rand::thread_rng();
        let rng_chance: f32 = rng.gen();
        let transform =
            Transform::from_scale(Vec3::splat(SPRITE_SCALE)).with_translation(Vec3::new(
                rng.gen_range((-WIDTH as i32 / 2)..(WIDTH as i32 / 2)) as f32,
                rng.gen_range((-HEIGHT as i32 / 2)..(HEIGHT as i32 / 2)) as f32,
                850.,
            ));

        if rng_chance >= 0.5 {
            println!("enemy spawned!");
            let enemy_type = EnemyType::from_u32(rng.gen_range(0..6)).unwrap();
            match enemy_type {
                EnemyType::Bat => commands.spawn_bundle(EnemyBundle {
                    enemy_type,
                    health: Health(100.),
                    damage: Damage(10.),
                    attack_range: AttackRange(1000.),
                    attack_timer: AttackTimer(Timer::from_seconds(1.0, true)),
                    sprite: SpriteSheetBundle {
                        texture_atlas: sprites.bat.clone(),
                        transform,
                        ..default()
                    },
                    ..default()
                }),
                EnemyType::EvilWizard => commands.spawn_bundle(EnemyBundle {
                    enemy_type,
                    health: Health(100.),
                    damage: Damage(10.),
                    attack_range: AttackRange(1000.),
                    attack_timer: AttackTimer(Timer::from_seconds(1.0, true)),
                    sprite: SpriteSheetBundle {
                        texture_atlas: sprites.evil_wizard.clone(),
                        transform,
                        ..default()
                    },
                    ..default()
                }),
                EnemyType::Ghost => commands.spawn_bundle(EnemyBundle {
                    enemy_type,
                    health: Health(100.),
                    damage: Damage(10.),
                    attack_range: AttackRange(1000.),
                    attack_timer: AttackTimer(Timer::from_seconds(1.0, true)),
                    sprite: SpriteSheetBundle {
                        texture_atlas: sprites.ghost.clone(),
                        transform,
                        ..default()
                    },
                    ..default()
                }),
                EnemyType::Lobster => commands.spawn_bundle(EnemyBundle {
                    enemy_type,
                    health: Health(100.),
                    damage: Damage(10.),
                    attack_range: AttackRange(1000.),
                    attack_timer: AttackTimer(Timer::from_seconds(1.0, true)),
                    sprite: SpriteSheetBundle {
                        texture_atlas: sprites.lobster.clone(),
                        transform,
                        ..default()
                    },
                    ..default()
                }),
                EnemyType::Rat => commands.spawn_bundle(EnemyBundle {
                    enemy_type,
                    health: Health(100.),
                    damage: Damage(10.),
                    attack_range: AttackRange(1000.),
                    attack_timer: AttackTimer(Timer::from_seconds(1.0, true)),
                    sprite: SpriteSheetBundle {
                        texture_atlas: sprites.rat1.clone(),
                        transform,
                        ..default()
                    },
                    ..default()
                }),
                EnemyType::Spider => commands.spawn_bundle(EnemyBundle {
                    enemy_type,
                    health: Health(100.),
                    damage: Damage(10.),
                    attack_range: AttackRange(1000.),
                    attack_timer: AttackTimer(Timer::from_seconds(1.0, true)),
                    sprite: SpriteSheetBundle {
                        texture_atlas: sprites.spider.clone(),
                        transform,
                        ..default()
                    },
                    ..default()
                }),
            };
            **enemy_count += 1;
        }
    }
}

use crate::{
    components::{
        AllyBundle, AnimationTimer, AttackRange, AttackTimer, Damage, EnemyBundle, Health,
        PartyRadius, Player, Speed,
    },
    consts::{SPRITE_SCALE, XEXTENT, YEXTENT},
    resources::{
        AllySpawnTimer, DifficultyScaleTimer, EnemyScale, EnemySpawnChance, EnemySpawnTimer,
        Sprites,
    },
    AllyType, EnemyType, GameState,
};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use iyes_loopless::prelude::*;
use rand::prelude::*;
pub struct SpawnPlugin;

impl Plugin for SpawnPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EnemySpawnTimer(Timer::from_seconds(0.75, true)))
            .insert_resource(AllySpawnTimer(Timer::from_seconds(1.0, true)))
            .insert_resource(DifficultyScaleTimer(Timer::from_seconds(1.5, true)))
            .insert_resource(EnemySpawnChance(0.8))
            .insert_resource(EnemyScale(1.0))
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(GameState::InGame)
                    .with_system(spawn_wave)
                    .with_system(spawn_allies)
                    .with_system(scale_difficulty)
                    .into(),
            );
    }
}

fn scale_difficulty(
    time: Res<Time>,
    mut difficulty_timer: ResMut<DifficultyScaleTimer>,
    mut spawn_chance: ResMut<EnemySpawnChance>,
    mut enemy_scale: ResMut<EnemyScale>,
) {
    difficulty_timer.tick(time.delta());
    if difficulty_timer.just_finished() {
        spawn_chance.0 /= 1.03;
        enemy_scale.0 *= 1.02;
    }
}

fn spawn_allies(
    mut commands: Commands,
    sprites: Res<Sprites>,
    time: Res<Time>,
    player: Query<(&Transform, &PartyRadius), With<Player>>,
    mut spawn_timer: ResMut<AllySpawnTimer>,
) {
    spawn_timer.tick(time.delta());

    if spawn_timer.just_finished() {
        let mut rng = rand::thread_rng();
        let roll: f32 = rng.gen();
        if roll >= 0.8 {
            let (player_transform, party_radius) = player.single();
            const MAX_TRIES: u32 = 100;
            let mut i = 0;
            let transform = loop {
                let transform =
                    Transform::from_scale(Vec3::splat(SPRITE_SCALE)).with_translation(Vec3::new(
                        rng.gen_range(XEXTENT.0 as i32..XEXTENT.1 as i32) as f32,
                        rng.gen_range(YEXTENT.0 as i32..YEXTENT.1 as i32) as f32,
                        1.0,
                    ));

                if transform
                    .translation
                    .truncate()
                    .distance(player_transform.translation.truncate())
                    > party_radius.0 * SPRITE_SCALE
                {
                    break transform;
                }

                i += 1;
                if i > MAX_TRIES {
                    return;
                }
            };

            let mut timer = Timer::from_seconds(0.115, true);
            timer.pause();

            let ally_type: AllyType = rng.gen();

            match ally_type {
                AllyType::Alchemist => commands.spawn_bundle(AllyBundle {
                    ally_type,
                    health: Health(100.0, 100.0),
                    damage: Damage(15.0),
                    attack_range: AttackRange(80.),
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
                    health: Health(100.0, 100.0),
                    damage: Damage(15.0),
                    attack_range: AttackRange(250.),
                    attack_timer: AttackTimer(Timer::from_seconds(0.5, true)),
                    sprite: SpriteSheetBundle {
                        texture_atlas: sprites.archer.clone(),
                        transform,
                        ..default()
                    },
                    ..default()
                }),
                AllyType::Cyclops => commands.spawn_bundle(AllyBundle {
                    ally_type,
                    health: Health(150.0, 150.0),
                    damage: Damage(35.0),
                    attack_range: AttackRange(60.),
                    attack_timer: AttackTimer(Timer::from_seconds(1.5, true)),
                    sprite: SpriteSheetBundle {
                        texture_atlas: sprites.cyclops.clone(),
                        transform,
                        ..default()
                    },
                    ..default()
                }),
                AllyType::Dwarf => commands.spawn_bundle(AllyBundle {
                    ally_type,
                    health: Health(90.0, 90.0),
                    damage: Damage(25.0),
                    attack_range: AttackRange(90.),
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
                    health: Health(115.0, 115.0),
                    damage: Damage(25.0),
                    attack_range: AttackRange(80.),
                    attack_timer: AttackTimer(Timer::from_seconds(0.75, true)),
                    sprite: SpriteSheetBundle {
                        texture_atlas: sprites.knight.clone(),
                        transform,
                        ..default()
                    },
                    ..default()
                }),
                AllyType::Wizard => commands.spawn_bundle(AllyBundle {
                    ally_type,
                    health: Health(75.0, 75.0),
                    damage: Damage(30.0),
                    attack_range: AttackRange(200.),
                    attack_timer: AttackTimer(Timer::from_seconds(1.25, true)),
                    sprite: SpriteSheetBundle {
                        texture_atlas: sprites.wizard.clone(),
                        transform,
                        ..default()
                    },
                    ..default()
                }),
                AllyType::Player => unreachable!(),
            }
            .insert(AnimationTimer(timer))
            .insert(Collider::cuboid(8.0, 8.0))
            .insert(LockedAxes::ROTATION_LOCKED);
        }
    }
}

fn spawn_wave(
    mut commands: Commands,
    time: Res<Time>,
    sprites: Res<Sprites>,
    enemy_spawn_chance: Res<EnemySpawnChance>,
    enemy_scale: Res<EnemyScale>,
    mut spawn_timer: ResMut<EnemySpawnTimer>,
    player: Query<(&Transform, &PartyRadius), With<Player>>,
) {
    spawn_timer.tick(time.delta());

    if spawn_timer.just_finished() {
        let mut rng = rand::thread_rng();
        let roll: f32 = rng.gen();
        if roll >= enemy_spawn_chance.0 {
            let (player_transform, party_radius) = player.single();
            const MAX_TRIES: u32 = 100;
            let mut i = 0;
            let transform = loop {
                let transform =
                    Transform::from_scale(Vec3::splat(SPRITE_SCALE)).with_translation(Vec3::new(
                        rng.gen_range(XEXTENT.0 as i32..XEXTENT.1 as i32) as f32,
                        rng.gen_range(YEXTENT.0 as i32..YEXTENT.1 as i32) as f32,
                        1.0,
                    ));

                if transform
                    .translation
                    .truncate()
                    .distance(player_transform.translation.truncate())
                    > party_radius.0 * SPRITE_SCALE
                {
                    break transform;
                }

                i += 1;
                if i > MAX_TRIES {
                    return;
                }
            };

            let enemy_type: EnemyType = rng.gen();
            match enemy_type {
                EnemyType::Bat => commands.spawn_bundle(EnemyBundle {
                    enemy_type,
                    speed: Speed(90.0),
                    health: Health(60.0 * enemy_scale.0, 60.0 * enemy_scale.0),
                    damage: Damage(8.0 * enemy_scale.0),
                    attack_range: AttackRange(45.),
                    attack_timer: AttackTimer(Timer::from_seconds(0.75, true)),
                    sprite: SpriteSheetBundle {
                        texture_atlas: sprites.bat.clone(),
                        transform,
                        ..default()
                    },
                    ..default()
                }),
                EnemyType::EvilWizard => commands.spawn_bundle(EnemyBundle {
                    enemy_type,
                    speed: Speed(70.0),
                    health: Health(75.0 * enemy_scale.0, 75.0 * enemy_scale.0),
                    damage: Damage(15. * enemy_scale.0),
                    attack_range: AttackRange(200.),
                    attack_timer: AttackTimer(Timer::from_seconds(1.25, true)),
                    sprite: SpriteSheetBundle {
                        texture_atlas: sprites.evil_wizard.clone(),
                        transform,
                        ..default()
                    },
                    ..default()
                }),
                EnemyType::Ghost => commands.spawn_bundle(EnemyBundle {
                    enemy_type,
                    speed: Speed(65.0),
                    health: Health(100.0 * enemy_scale.0, 100.0 * enemy_scale.0),
                    damage: Damage(20. * enemy_scale.0),
                    attack_range: AttackRange(60.),
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
                    speed: Speed(75.0),
                    health: Health(80.0 * enemy_scale.0, 80.0 * enemy_scale.0),
                    damage: Damage(15. * enemy_scale.0),
                    attack_range: AttackRange(40.),
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
                    speed: Speed(110.0),
                    health: Health(50.0 * enemy_scale.0, 50.0 * enemy_scale.0),
                    damage: Damage(5. * enemy_scale.0),
                    attack_range: AttackRange(50.),
                    attack_timer: AttackTimer(Timer::from_seconds(0.5, true)),
                    sprite: SpriteSheetBundle {
                        texture_atlas: sprites.rat.clone(),
                        transform,
                        ..default()
                    },
                    ..default()
                }),
                EnemyType::Spider => commands.spawn_bundle(EnemyBundle {
                    enemy_type,
                    speed: Speed(150.0),
                    health: Health(65.0 * enemy_scale.0, 65.0 * enemy_scale.0),
                    damage: Damage(10. * enemy_scale.0),
                    attack_range: AttackRange(40.),
                    attack_timer: AttackTimer(Timer::from_seconds(0.75, true)),
                    sprite: SpriteSheetBundle {
                        texture_atlas: sprites.spider.clone(),
                        transform,
                        ..default()
                    },
                    ..default()
                }),
            }
            .insert(AnimationTimer(Timer::from_seconds(0.115, true)))
            .insert(Collider::cuboid(8.0, 8.0))
            .insert(LockedAxes::ROTATION_LOCKED);
        }
    }
}

use crate::{
    components::{
        AllyBundle, AttackRange, AttackTimer, AttackType, Damage, EnemyBundle, Health, Projectile,
    },
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

        println!("Ally Spawned!");
        let ally_type = AllyType::from_u32(rng.gen_range(0..6)).unwrap();
        commands.spawn_bundle(AllyBundle {
            health: Health(100.0),
            attack_range: AttackRange(1000.0),
            attack_timer: AttackTimer(Timer::from_seconds(1.0, true)),
            damage: Damage(10.0),
            attack_type: AttackType::Ranged(5.0, Projectile::Ally),
            sprite: SpriteBundle {
                texture: match ally_type {
                    AllyType::Alchemist => sprites.alchemist.clone(),
                    AllyType::Archer => sprites.archer.clone(),
                    AllyType::Cyclops => sprites.cyclops.clone(),
                    AllyType::Dwarf => sprites.dwarf.clone(),
                    AllyType::Knight => sprites.knight.clone(),
                    AllyType::Wizard => sprites.wizard.clone(),
                },
                transform: Transform::from_scale(Vec3::splat(SPRITE_SCALE)).with_translation(
                    Vec3::new(
                        rng.gen_range((-WIDTH as i32 / 2)..(WIDTH as i32 / 2)) as f32,
                        rng.gen_range((-HEIGHT as i32 / 2)..(HEIGHT as i32 / 2)) as f32,
                        850.,
                    ),
                ),
                ..default()
            },
            ..default()
        });
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

    if spawn_timer.just_finished() && **enemy_count <= 5 {
        let mut rng = rand::thread_rng();
        let rng_chance: f32 = rng.gen();

        if rng_chance >= 0.5 {
            println!("enemy spawned!");
            let enemy_type = EnemyType::from_u32(rng.gen_range(0..7)).unwrap();
            commands.spawn_bundle(EnemyBundle {
                health: Health(100.0),
                attack_range: AttackRange(10000.0),
                attack_timer: AttackTimer(Timer::from_seconds(1.0, true)),
                damage: Damage(10.0),
                attack_type: AttackType::Ranged(5.0, Projectile::Enemy),
                sprite: SpriteBundle {
                    texture: match enemy_type {
                        EnemyType::Bat => sprites.bat.clone(),
                        EnemyType::Cactus => sprites.cactus.clone(),
                        EnemyType::EvilWizard => sprites.evil_wizard.clone(),
                        EnemyType::Ghost => sprites.ghost.clone(),
                        EnemyType::Lobster => sprites.lobster.clone(),
                        EnemyType::Rat => sprites.rat1.clone(),
                        EnemyType::Spider => sprites.spider.clone(),
                    },
                    transform: Transform::from_scale(Vec3::splat(SPRITE_SCALE)).with_translation(
                        Vec3::new(
                            rng.gen_range((-WIDTH as i32 / 2)..(WIDTH as i32 / 2)) as f32,
                            rng.gen_range((-HEIGHT as i32 / 2)..(HEIGHT as i32 / 2)) as f32,
                            800.,
                        ),
                    ),
                    ..default()
                },
                ..default()
            });
            **enemy_count += 1;
        }
    }
}

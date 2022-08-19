use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{
    components::{Ally, AttackRange, AttackTimer, AttackType, Enemy},
    GameState, InGameState,
};

pub struct AutoBattlePlugin;

impl Plugin for AutoBattlePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::InGame(InGameState::Wave))
                .with_system(auto_battle)
                .into(),
        );
    }
}

fn auto_battle(
    time: Res<Time>,
    mut friendlies_query: Query<
        (&Transform, &mut AttackTimer, &AttackRange, &AttackType),
        (With<Ally>, Without<Enemy>),
    >,
    mut enemies_query: Query<
        (&Transform, &mut AttackTimer, &AttackRange, &AttackType),
        (With<Enemy>, Without<Ally>),
    >,
) {
    for (ally_transform, mut ally_timer, ally_range, ally_attack) in &mut friendlies_query {
        for (enemy_transform, mut enemy_timer, enemy_range, enemy_attack) in &mut enemies_query {
            do_attack(
                &time,
                ally_transform,
                enemy_transform,
                &mut ally_timer,
                ally_range,
                ally_attack,
            );
            do_attack(
                &time,
                enemy_transform,
                ally_transform,
                &mut enemy_timer,
                enemy_range,
                enemy_attack,
            );
        }
    }
}

fn do_attack(
    time: &Time,
    attacker_transform: &Transform,
    target_transform: &Transform,
    timer: &mut AttackTimer,
    range: &AttackRange,
    attack_type: &AttackType,
) {
    let dist = attacker_transform
        .translation
        .distance(target_transform.translation);

    if dist <= **range {
        timer.tick(time.delta());
        if timer.just_finished() {
            // attack
        }
    }
}

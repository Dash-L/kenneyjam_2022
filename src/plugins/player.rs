use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{
    components::{AnimationTimer, Player, Speed, Velocity},
    GameState, InGameState,
};
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::InGame(InGameState::DownTime))
                .with_system(handle_inputs)
                .with_system(move_player)
                .into(),
        )
        .add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::InGame(InGameState::Wave))
                .with_system(handle_inputs)
                .with_system(move_player)
                .into(),
        );
    }
}

fn handle_inputs(
    mut player: Query<(&mut Velocity, &mut AnimationTimer), With<Player>>,
    keyboard: Res<Input<KeyCode>>,
) {
    let (mut velocity, mut animation_timer) = player.single_mut();
    *velocity = Velocity(Vec2::ZERO);
    if keyboard.pressed(KeyCode::W) {
        velocity.y += 1.;
    }
    if keyboard.pressed(KeyCode::S) {
        velocity.y -= 1.;
    }
    if keyboard.pressed(KeyCode::D) {
        velocity.x += 1.;
    }
    if keyboard.pressed(KeyCode::A) {
        velocity.x -= 1.;
    }
    if velocity.0 == Vec2::ZERO {
        animation_timer.pause();
    } else {
        animation_timer.unpause();
    }

    *velocity = Velocity(velocity.normalize_or_zero());
}

fn move_player(mut player: Query<(&mut Transform, &Velocity, &Speed), With<Player>>) {
    let (mut transform, velocity, speed) = player.single_mut();
    transform.translation += (velocity.0 * speed.0).extend(0.);
}

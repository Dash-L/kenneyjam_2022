use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{
    components::{Player, Velocity},
    consts::{HEIGHT, PLAYERSPEED, WIDTH},
    GameState, InGameState,
};
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            ConditionSet::new()
                //.run_in_state(GameState::InGame(InGameState::Wave))
                .run_in_state(GameState::InGame(InGameState::DownTime))
                .with_system(calc_velocity)
                .with_system(move_player)
                .into(),
        );
    }
}

fn calc_velocity(
    mut player_query: Query<&mut Velocity, With<Player>>,
    keyboard: Res<Input<KeyCode>>,
) {
    let mut velocity = player_query.single_mut();
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

    *velocity = Velocity(velocity.normalize_or_zero());
}

fn move_player(mut player_query: Query<(&mut Transform, &Velocity), With<Player>>) {
    let (mut player_transform, player_velocity) = player_query.single_mut();
    player_transform.translation += (player_velocity.0 * PLAYERSPEED).extend(0.);
}

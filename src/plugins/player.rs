use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{
    components::{AnimationTimer, Player, Speed, Velocity},
    consts::{XEXTENT, YEXTENT},
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
    mut player: Query<(&mut Velocity, &mut AnimationTimer, &mut TextureAtlasSprite), With<Player>>,
    keyboard: Res<Input<KeyCode>>,
) {
    let (mut velocity, mut animation_timer, mut texture_atlas_sprite) = player.single_mut();
    *velocity = Velocity(Vec2::ZERO);
    if keyboard.pressed(KeyCode::W) {
        velocity.y += 1.;
    }
    if keyboard.pressed(KeyCode::S) {
        velocity.y -= 1.;
    }
    if keyboard.pressed(KeyCode::D) {
        velocity.x += 1.;
        texture_atlas_sprite.flip_x = false;
    }
    if keyboard.pressed(KeyCode::A) {
        velocity.x -= 1.;
        texture_atlas_sprite.flip_x = true;
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
    if transform.translation.x >= XEXTENT.0 && transform.translation.x <= XEXTENT.1 {
        transform.translation.x += velocity.0.x * speed.0;
        if transform.translation.x < XEXTENT.0 {
            transform.translation.x = XEXTENT.0;
        } else if transform.translation.x > XEXTENT.1 {
            transform.translation.x = XEXTENT.1;
        }
    }
    if transform.translation.y >= YEXTENT.0 && transform.translation.y <= YEXTENT.1 {
        transform.translation.y += velocity.0.y * speed.0;
        if transform.translation.y < YEXTENT.0 {
            transform.translation.y = YEXTENT.0;
        } else if transform.translation.y > YEXTENT.1 {
            transform.translation.y = YEXTENT.1;
        }
    }
}

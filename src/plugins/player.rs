use std::iter;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use iyes_loopless::prelude::*;

use crate::{
    components::{AllyType, AnimationTimer, InParty, PartyRadius, Player, Speed, Velocity},
    consts::{SPRITE_SCALE, XEXTENT, YEXTENT},
    GameState,
};
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::InGame)
                .with_system(handle_inputs)
                .with_system(move_party)
                .with_system(update_circle)
                .with_system(add_to_party)
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

    *velocity = Velocity(velocity.normalize_or_zero());
}

fn move_party(
    mut player: Query<(&mut Transform, &Velocity, &Speed, &mut AnimationTimer), With<Player>>,
    mut party_members: Query<
        (&mut Transform, &mut AnimationTimer),
        (With<InParty>, With<AllyType>, Without<Player>),
    >,
) {
    let (player_transform, velocity, speed, player_animation_timer) = player.single_mut();
    for (mut transform, mut animation_timer) in
        iter::once((player_transform, player_animation_timer)).chain(party_members.iter_mut())
    {
        if velocity.0 == Vec2::ZERO {
            animation_timer.pause();
        } else {
            animation_timer.unpause();
        }
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
}

fn update_circle(player: Query<&PartyRadius, With<Player>>, mut path: Query<&mut Path>) {
    let party_radius = player.single();
    let mut path = path.single_mut();

    let circle = shapes::Circle {
        radius: party_radius.0,
        ..default()
    };

    *path = ShapePath::build_as(&circle);
}

fn add_to_party(
    mut commands: Commands,
    player: Query<(&Transform, &PartyRadius), With<Player>>,
    entities: Query<(Entity, &Transform), Without<InParty>>,
) {
    let (player_transform, party_radius) = player.single();
    for (entity, transform) in &entities {
        if player_transform
            .translation
            .truncate()
            .distance(transform.translation.truncate())
            < party_radius.0 * SPRITE_SCALE
        {
            commands.entity(entity).insert(InParty);
        }
    }
}

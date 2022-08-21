use std::iter;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;
use iyes_loopless::prelude::*;

use crate::{
    components::{AllyType, AnimationTimer, EnemyType, InParty, IsDead, PartyRadius, Player},
    consts::SPRITE_SCALE,
    GameState, helpers::{check_player_death, player_death_animation},
};
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::InGame)
                .label("first")
                .with_system(handle_inputs)
                .with_system(update_circle)
                .with_system(add_to_party)
                .with_system(move_enemies_towards_closest_ally)
                .with_system(check_player_death)
                .with_system(player_death_animation)
                .into(),
        )
        .add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::InGame)
                .after("first")
                .with_system(keep_allies_in_circle)
                .into(),
        );
    }
}

fn handle_inputs(
    mut player: Query<
        (&mut Velocity, &mut AnimationTimer, &mut TextureAtlasSprite),
        (With<Player>, Without<IsDead>),
    >,
    mut party_members: Query<
        (&mut Velocity, &mut AnimationTimer, &mut TextureAtlasSprite),
        (With<InParty>, Without<Player>),
    >,
    keyboard: Res<Input<KeyCode>>,
) {
    if let Ok((velocity, animation_timer, texture_atlas_sprite)) = player.get_single_mut() {
        for (mut velocity, mut animation_timer, mut texture_atlas_sprite) in
            iter::once((velocity, animation_timer, texture_atlas_sprite))
                .chain(party_members.iter_mut())
        {
            velocity.linvel = Vec2::ZERO;
            if keyboard.pressed(KeyCode::W) {
                velocity.linvel.y += 1.;
            }
            if keyboard.pressed(KeyCode::S) {
                velocity.linvel.y -= 1.;
            }
            if keyboard.pressed(KeyCode::D) {
                velocity.linvel.x += 1.;
                texture_atlas_sprite.flip_x = false;
            }
            if keyboard.pressed(KeyCode::A) {
                velocity.linvel.x -= 1.;
                texture_atlas_sprite.flip_x = true;
            }

            velocity.linvel = velocity.linvel.normalize_or_zero() * 200.0;

            if velocity.linvel == Vec2::ZERO {
                animation_timer.pause();
            } else {
                animation_timer.unpause();
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
    entities: Query<(Entity, &Transform), (Without<InParty>, Without<Player>, With<AllyType>)>,
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

fn keep_allies_in_circle(
    player: Query<(&Transform, &PartyRadius), With<Player>>,
    mut party_members: Query<(&Transform, &mut Velocity), With<InParty>>,
) {
    let (player_transform, party_radius) = player.single();
    for (transform, mut vel) in &mut party_members {
        if player_transform
            .translation
            .truncate()
            .distance(transform.translation.truncate())
            > party_radius.0 * SPRITE_SCALE
        {
            vel.linvel = (player_transform.translation.truncate()
                - transform.translation.truncate())
            .normalize()
                * 400.0;
        }
    }
}

fn move_enemies_towards_closest_ally(
    allies: Query<&Transform, With<AllyType>>,
    mut enemies: Query<(&Transform, &mut Velocity), With<EnemyType>>,
) {
    for (enemy_transform, mut velocity) in &mut enemies {
        let mut closest = (f32::MAX, Transform::default());
        for ally_transform in &allies {
            let dist = enemy_transform
                .translation
                .truncate()
                .distance(ally_transform.translation.truncate());
            if dist < closest.0 {
                closest = (dist, *ally_transform);
            }
        }
        velocity.linvel =
            (closest.1.translation.truncate() - enemy_transform.translation.truncate()).normalize()
                * 80.0;
    }
}

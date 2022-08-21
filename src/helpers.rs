use bevy::prelude::*;

use crate::{
    components::{
        AllyType, AnimationTimer, EnemyType, HasHealthBar, Health, MainHealthBar, Player,
        Projectile,
    },
    consts::HEALTH_BAR_LEN,
};

pub fn animate_sprites(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<
        (
            &mut AnimationTimer,
            &mut TextureAtlasSprite,
            &Handle<TextureAtlas>,
        ),
        (
            Without<Projectile<AllyType>>,
            Without<Projectile<EnemyType>>,
        ),
    >,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in &mut query {
        timer.tick(time.delta());
        if timer.paused() {
            sprite.index = 0;
        } else if timer.just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % (texture_atlas.textures.len());
            if sprite.index == 0 && texture_atlas.textures.len() > 1 {
                sprite.index = 1;
            }
        }
    }
}

pub fn animate_attacks(
    mut commands: Commands,
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<
        (
            Entity,
            &mut AnimationTimer,
            &mut TextureAtlasSprite,
            &Handle<TextureAtlas>,
        ),
        Or<(With<Projectile<AllyType>>, With<Projectile<EnemyType>>)>,
    >,
) {
    for (entity, mut timer, mut sprite, texture_atlas_handle) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % (texture_atlas.textures.len());
            if sprite.index == 0 {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}

pub fn regen(mut entities: Query<&mut Health>) {
    for mut health in &mut entities {
        health.0 += 0.01;
        if health.0 > health.1 {
            health.0 = health.1;
        }
    }
}

pub fn spawn_health_bars(
    mut commands: Commands,
    entities: Query<Entity, (With<Health>, Without<HasHealthBar>)>,
) {
    for entity in &entities {
        commands
            .entity(entity)
            .with_children(|parent| {
                parent.spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: Color::RED,
                        custom_size: Some(Vec2::new(HEALTH_BAR_LEN, 1.5)),
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::new(0.0, 10.0, 5.0)),
                    ..default()
                });
                parent
                    .spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            color: Color::GREEN,
                            custom_size: Some(Vec2::new(HEALTH_BAR_LEN, 1.5)),
                            ..default()
                        },
                        transform: Transform::from_translation(Vec3::new(0.0, 10.0, 10.0)),
                        ..default()
                    })
                    .insert(MainHealthBar);
            })
            .insert(HasHealthBar);
    }
}

pub fn update_health_bars(
    entities: Query<(&Health, &Children), (Changed<Health>, With<HasHealthBar>)>,
    mut health_bars: Query<(&mut Transform, &mut Sprite), With<MainHealthBar>>,
) {
    for (health, children) in &entities {
        for &child in children {
            if let Ok((mut transform, mut sprite)) = health_bars.get_mut(child) {
                let ratio = health.0 / health.1;
                transform.translation.x = -HEALTH_BAR_LEN * (1.0 - ratio) / 2.0;
                sprite.custom_size = Some(Vec2::new(HEALTH_BAR_LEN * ratio, 1.5));
            }
        }
    }
}

pub fn despawn_zero_health(
    mut commands: Commands,
    entities: Query<(Entity, &Health, Option<&Player>)>,
) {
    for (entity, health, maybe_player) in &entities {
        if health.0 <= 0.0 {
            commands.entity(entity).despawn_recursive();
            if maybe_player.is_some() {
                // you died screen or smth
            }
        }
    }
}

use bevy::prelude::*;

use crate::{
    components::{AnimationTimer, EnemyType, HasHealthBar, Health, MainHealthBar},
    consts::HEALTH_BAR_LEN,
    resources::EnemiesCount,
};

pub fn animate_sprites(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in &mut query {
        timer.tick(time.delta());
        if timer.paused() {
            sprite.index = 0;
        } else if timer.just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % (texture_atlas.textures.len());
            if sprite.index == 0 {
                sprite.index = 1;
            }
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
    mut enemy_count: ResMut<EnemiesCount>,
    entities: Query<(Entity, &Health, Option<&EnemyType>)>,
) {
    for (entity, health, maybe_enemy) in &entities {
        if health.0 <= 0.0 {
            commands.entity(entity).despawn_recursive();
            if maybe_enemy.is_some() {
                **enemy_count -= 1;
            }
        }
    }
}

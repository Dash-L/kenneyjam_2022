use std::{cell::RefCell, rc::Rc};

use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};

use crate::{
    components::{
        AllyType, AnimationTimer, Collider, EnemyType, HasHealthBar, Health, MainHealthBar,
        PrevPosition,
    },
    consts::HEALTH_BAR_LEN,
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
    entities: Query<(&Health, &Children), With<HasHealthBar>>,
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
pub fn handle_collision(
    mut entities: Query<
        (Entity, &mut Transform, &Collider, &PrevPosition),
        Or<(With<AllyType>, With<EnemyType>)>,
    >,
) {
    let entities_arr = entities
        .iter_mut()
        .map(|e| Rc::new(RefCell::new(e)))
        .collect::<Vec<Rc<RefCell<_>>>>();
    for entity in entities_arr.clone() {
        for other_entity in &entities_arr {
            if entity.borrow().0 != other_entity.borrow().0 {
                while let Some(_) = collide(
                    entity.borrow().1.translation,
                    entity.borrow().2 .0,
                    other_entity.borrow().1.translation,
                    other_entity.borrow().2 .0,
                ) {
                    let vectorthing =
                        (entity.borrow().3 .0 - entity.borrow().1.translation).normalize() * 0.01;
                    entity.borrow_mut().1.translation += vectorthing;
                }
            }
        }
    }
}

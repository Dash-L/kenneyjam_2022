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
    consts::{HEALTH_BAR_LEN, XEXTENT, YEXTENT},
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

pub fn update_prev_position(mut entities: Query<(&Transform, &mut PrevPosition)>) {
    for (transform, mut prev_position) in &mut entities {
        prev_position.0 = transform.translation;
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
    for entity in &entities_arr {
        for other_entity in &entities_arr {
            if entity.borrow().0 != other_entity.borrow().0 {
                let mut entity = entity.borrow_mut();
                let other_entity = other_entity.borrow();
                const MAX_ITERATIONS: u32 = 1000;
                let mut i = 0;
                if entity.3 .0 != entity.1.translation {
                    while let Some(_) = collide(
                        entity.1.translation,
                        entity.2 .0,
                        other_entity.1.translation,
                        other_entity.2 .0,
                    ) {
                        if i >= MAX_ITERATIONS {
                            break;
                        }
                        let vector = (entity.3 .0 - entity.1.translation).normalize() * 0.05;
                        entity.1.translation += vector;
                        i += 1;
                    }
                    if let Some(collision) = collide(
                        entity.1.translation,
                        entity.2 .0,
                        other_entity.1.translation,
                        other_entity.2 .0,
                    ) {
                        println!(
                            "{:?} stuck! with {:?} ({:?})",
                            entity.0, other_entity.0, collision
                        );
                        const RESOLUTION: f32 = 0.5;
                        match collision {
                            Collision::Left => entity.1.translation += Vec3::NEG_X * RESOLUTION,
                            Collision::Right => entity.1.translation += Vec3::X * RESOLUTION,
                            Collision::Top => entity.1.translation += Vec3::Y * RESOLUTION,
                            Collision::Bottom => entity.1.translation += Vec3::NEG_Y * RESOLUTION,
                            _ => {}
                        }
                    }
                }
            }
        }
    }
}

pub fn keep_in_map(mut entities: Query<&mut Transform, Or<(With<EnemyType>, With<AllyType>)>>) {
    for mut transform in &mut entities {
        if transform.translation.x < XEXTENT.0 {
            transform.translation.x = XEXTENT.0;
        } else if transform.translation.x > XEXTENT.1 {
            transform.translation.x = XEXTENT.1;
        }

        if transform.translation.y < YEXTENT.0 {
            transform.translation.y = YEXTENT.0;
        } else if transform.translation.y > YEXTENT.1 {
            transform.translation.y = YEXTENT.1;
        }
    }
}

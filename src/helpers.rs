use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use iyes_loopless::prelude::*;

use crate::{
    components::{
        AllyType, AnimationTimer, EnemyType, HasHealthBar, Health, Indicator, IndicatorEntity,
        IsDead, MainHealthBar, PartyRadius, Player, Projectile,
    },
    consts::{BUTTON_CLICKED, BUTTON_DEFAULT, BUTTON_HOVERED, HEALTH_BAR_LEN},
    resources::Sprites,
    GameState,
};

pub fn despawn_with<C: Component>(mut commands: Commands, q: Query<Entity, With<C>>) {
    for e in &q {
        commands.entity(e).despawn_recursive();
    }
}

pub fn button_pressed<B: Component>(
    q: Query<&Interaction, (Changed<Interaction>, With<Button>, With<B>)>,
) -> bool {
    for interaction in &q {
        if *interaction == Interaction::Clicked {
            return true;
        }
    }

    false
}

pub fn update_buttons(
    mut q: Query<(&Interaction, &mut UiColor), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, mut color) in &mut q {
        match interaction {
            Interaction::Clicked => {
                *color = BUTTON_CLICKED;
            }
            Interaction::Hovered => {
                *color = BUTTON_HOVERED;
            }
            Interaction::None => {
                *color = BUTTON_DEFAULT;
            }
        }
    }
}

pub fn go_to_state(state: GameState) -> impl Fn(Commands) {
    move |mut commands: Commands| {
        commands.insert_resource(NextState(state));
    }
}

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
            Without<IsDead>,
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

pub fn regen(mut entities: Query<&mut Health, Without<EnemyType>>) {
    for mut health in &mut entities {
        health.0 += 0.075;
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
    mut player: Query<&mut PartyRadius, With<Player>>,
    entities: Query<
        (
            Entity,
            &Health,
            Option<&EnemyType>,
            Option<&IndicatorEntity>,
        ),
        Without<Player>,
    >,
) {
    for (entity, health, maybe_enemy, maybe_indicator) in &entities {
        if health.0 <= 0.0 {
            if maybe_enemy.is_some() {
                let mut radius = player.single_mut();
                radius.0 += 0.1;
            }
            if let Some(indicator) = maybe_indicator {
                if let Some(entity) = indicator.0 {
                    commands.entity(entity).despawn_recursive();
                }
            }
            commands.entity(entity).despawn_recursive();
        }
    }
}
pub fn check_player_death(
    mut commands: Commands,
    sprites: Res<Sprites>,
    mut player: Query<
        (Entity, &Health, &mut Handle<TextureAtlas>),
        (With<Player>, Without<IsDead>),
    >,
) {
    for (entity, health, mut animation) in &mut player {
        if health.0 <= 0.0 {
            *animation = sprites.player_death.clone();
            commands.entity(entity).insert(IsDead);
        }
    }
}

pub fn player_death_animation(
    mut commands: Commands,
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut player: Query<
        (
            Entity,
            &mut Health,
            &mut Velocity,
            &mut AnimationTimer,
            &mut TextureAtlasSprite,
            &Handle<TextureAtlas>,
        ),
        (With<Player>, With<IsDead>),
    >,
    entities: Query<
        Entity,
        (
            Without<Player>,
            Or<(
                With<AllyType>,
                With<EnemyType>,
                With<Indicator>,
                With<Projectile<AllyType>>,
                With<Projectile<EnemyType>>,
            )>,
        ),
    >,
) {
    for (
        entity,
        mut health,
        mut vel,
        mut animation_timer,
        mut texture_atlas_sprite,
        texture_atlas_handle,
    ) in &mut player
    {
        animation_timer.set_duration(Duration::from_secs_f32(0.05));
        health.0 = 0.0;
        vel.linvel = Vec2::ZERO;
        if animation_timer.paused() {
            animation_timer.unpause();
        }

        animation_timer.tick(time.delta());
        if animation_timer.just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            texture_atlas_sprite.index =
                (texture_atlas_sprite.index + 1) % texture_atlas.textures.len();

            if texture_atlas_sprite.index == 0 {
                commands.entity(entity).despawn_recursive();
                for entity in &entities {
                    commands.entity(entity).despawn_recursive();
                }
                commands.insert_resource(NextState(GameState::MainMenu));
            }
        }
    }
}

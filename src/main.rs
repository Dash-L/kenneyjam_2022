use bevy::{prelude::*, render::{texture::ImageSettings, render_resource::PolygonMode}};
use bevy_asset_loader::prelude::*;
use iyes_loopless::prelude::*;

mod components;
use components::*;

mod consts;
use consts::*;

mod helpers;
use helpers::*;

mod plugins;
use plugins::*;

mod resources;
use resources::*;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum InGameState {
    DownTime,
    Wave,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum GameState {
    Load,
    Setup,
    InGame(InGameState),
}

fn main() {
    App::new()
        .insert_resource(ImageSettings::default_nearest())
        .insert_resource(PolygonMode::Line)
        .insert_resource(WindowDescriptor {
            title: "Kenney Jam".to_string(),
            width: WIDTH,
            height: HEIGHT,
            resizable: false,
            ..default()
        })
        .add_loopless_state(GameState::Load)
        .add_loading_state(
            LoadingState::new(GameState::Load)
                .continue_to_state(GameState::Setup)
                .with_collection::<Sprites>(),
        )
        .add_plugins(DefaultPlugins)
        .add_plugin(SpawnPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(AutoBattlePlugin)
        .add_enter_system(GameState::Setup, setup)
        .add_system(animate_sprites)
        .add_system(spawn_health_bars)
        .add_system(update_health_bars)
        .run();
}

fn setup(mut commands: Commands, sprites: Res<Sprites>) {
    commands.spawn_bundle(SpriteBundle {
        texture: sprites.background.clone(),
        transform: Transform::from_scale(Vec3::splat(5.0)),
        ..default()
    });
    commands
        .spawn_bundle(PlayerBundle {
            speed: Speed(5.0),
            ally: AllyBundle {
                ally_type: AllyType::Player,
                attack_range: AttackRange(5.0),
                attack_timer: AttackTimer(Timer::from_seconds(0.5, true)),
                damage: Damage(5.0),
                health: Health(75.0, 100.0),
                sprite: SpriteSheetBundle {
                    texture_atlas: sprites.player.clone(),
                    transform: Transform::from_scale(Vec3::splat(SPRITE_SCALE))
                        .with_translation(Vec3::new(0., 0., 900.)),
                    ..default()
                },
                ..default()
            },
            ..default()
        })
        .insert(AnimationTimer(Timer::from_seconds(0.115, true)))
        .with_children(|parent| {
            parent.spawn_bundle(Camera2dBundle {
                transform: Transform::from_scale(Vec2::splat(0.35).extend(1.))
                    .with_translation(Vec3::Z * 99.9),
                ..default()
            });
        });

    commands.insert_resource(NextState(GameState::InGame(InGameState::DownTime)));
}

fn animate_sprites(
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

fn spawn_health_bars(
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
                    transform: Transform::from_translation(Vec3::Y * 10.0),
                    ..default()
                });
                parent
                    .spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            color: Color::GREEN,
                            custom_size: Some(Vec2::new(HEALTH_BAR_LEN, 1.5)),
                            ..default()
                        },
                        transform: Transform::from_translation(Vec3::new(0.0, 10.0, 2.0)),
                        ..default()
                    })
                    .insert(MainHealthBar);
            })
            .insert(HasHealthBar);
    }
}

fn update_health_bars(
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

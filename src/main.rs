use bevy::{prelude::*, render::texture::ImageSettings};
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
        .run();
}

fn setup(mut commands: Commands, sprites: Res<Sprites>) {
    commands.spawn_bundle(Camera2dBundle::default());
    commands.spawn_bundle(SpriteBundle {
        texture: sprites.background.clone(),
        transform: Transform::from_scale(Vec3::splat(5.0)).with_translation(Vec3::Z * 10.0),
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
                health: Health(100.0),
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
        .insert(AnimationTimer(Timer::from_seconds(0.115, true)));

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

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
use num_derive::FromPrimitive;
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
#[derive(Clone, Copy, PartialEq, Eq, FromPrimitive)]
pub enum EnemyType {
    Bat,
    Cactus,
    EvilWizard,
    Ghost,
    Lobster,
    Rat,
    Spider,
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
        .run();
}

fn setup(mut commands: Commands, sprites: Res<Sprites>) {
    commands.spawn_bundle(Camera2dBundle::default());
    commands.spawn_bundle(SpriteBundle {
        texture: sprites.background.clone(),
        transform: Transform::from_scale(Vec3::splat(5.0)),
        ..default()
    });
    commands.spawn_bundle(PlayerBundle {
        health: Health(100.0),
        speed: Speed(10.0),
        sprite: SpriteBundle {
            texture: sprites.player.clone(),
            transform: Transform::from_scale(Vec3::splat(SPRITE_SCALE))
                .with_translation(Vec3::new(0., 0., 900.)),
            ..default()
        },
        ..default()
    });

    commands.insert_resource(NextState(GameState::InGame(InGameState::DownTime)));
}

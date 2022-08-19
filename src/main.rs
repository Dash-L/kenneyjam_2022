use bevy::{prelude::*, render::texture::ImageSettings};
use bevy_asset_loader::prelude::*;
use iyes_loopless::prelude::*;

mod consts;
use consts::*;

mod helpers;
use helpers::*;

mod plugins;
use plugins::*;

mod resources;
use resources::*;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum GameState {
    Load,
    Setup,
    InGame,
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
}

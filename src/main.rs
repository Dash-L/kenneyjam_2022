use bevy::{prelude::*, render::texture::ImageSettings};
use bevy_asset_loader::prelude::*;
use bevy_prototype_lyon::prelude::*;
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
        .add_plugin(ShapePlugin)
        .add_plugin(SpawnPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(AutoBattlePlugin)
        .add_enter_system(GameState::Setup, setup)
        .add_system(animate_sprites)
        .add_system(spawn_health_bars)
        .add_system(update_health_bars)
        .add_system_to_stage(CoreStage::PostUpdate, handle_collision)
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
            speed: Speed(2.0),
            party_radius: PartyRadius(20.0),
            ally: AllyBundle {
                ally_type: AllyType::Player,
                attack_range: AttackRange(5.0),
                attack_timer: AttackTimer(Timer::from_seconds(0.5, true)),
                damage: Damage(5.0),
                health: Health(75.0, 100.0),
                sprite: SpriteSheetBundle {
                    texture_atlas: sprites.player.clone(),
                    transform: Transform::from_scale(Vec3::splat(SPRITE_SCALE))
                        .with_translation(Vec3::new(0., 0., 2.)),
                    ..default()
                },
                ..default()
            },
            ..default()
        })
        .insert(AnimationTimer(Timer::from_seconds(0.115, true)))
        .insert(Collider(Vec2::splat(16.) * SPRITE_SCALE))
        .with_children(|parent| {
            let shape = shapes::Circle { ..default() };
            parent.spawn_bundle(GeometryBuilder::build_as(
                &shape,
                DrawMode::Stroke(StrokeMode {
                    color: Color::PURPLE,
                    options: StrokeOptions::default().with_line_width(1.0),
                }),
                Transform::default(),
            ));
            parent.spawn_bundle(Camera2dBundle {
                transform: Transform::from_scale(Vec2::splat(0.25).extend(1.))
                    .with_translation(Vec3::Z * 997.9),
                ..default()
            });
        });

    commands.insert_resource(NextState(GameState::InGame));
}

use bevy::{prelude::*, render::texture::ImageSettings};
use bevy_asset_loader::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;
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
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..default()
        })
        .add_loopless_state(GameState::Load)
        .add_loading_state(
            LoadingState::new(GameState::Load)
                .continue_to_state(GameState::Setup)
                .with_collection::<Sprites>()
                .with_collection::<Sounds>(),
        )
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(SpawnPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(AutoBattlePlugin)
        .add_plugin(DragAndDropPlugin)
        .add_enter_system(GameState::Setup, setup)
        .add_system(animate_sprites)
        .add_system(animate_attacks)
        .add_system(spawn_health_bars)
        .add_system(update_health_bars)
        .add_system(despawn_zero_health)
        .add_system(regen)
        .run();
}

fn setup(
    mut commands: Commands,
    sprites: Res<Sprites>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    // Background
    commands.spawn_bundle(SpriteBundle {
        texture: sprites.background.clone(),
        transform: Transform::from_scale(Vec3::splat(5.0)),
        ..default()
    });

    // Wall colliders
    commands
        .spawn_bundle(TransformBundle::from_transform(
            Transform::from_translation(Vec3::new(
                XEXTENT.1 + 25.0,
                0.5 * (YEXTENT.0 + YEXTENT.1),
                0.0,
            )),
        ))
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(10.0, 0.5 * (YEXTENT.1 - YEXTENT.0)));
    commands
        .spawn_bundle(TransformBundle::from_transform(
            Transform::from_translation(Vec3::new(
                XEXTENT.0 - 25.0,
                0.5 * (YEXTENT.0 + YEXTENT.1),
                0.0,
            )),
        ))
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(10.0, 0.5 * (YEXTENT.1 - YEXTENT.0)));
    commands
        .spawn_bundle(TransformBundle::from_transform(
            Transform::from_translation(Vec3::new(
                0.5 * (XEXTENT.0 + XEXTENT.1),
                YEXTENT.1 + 30.0,
                0.0,
            )),
        ))
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(0.5 * (XEXTENT.1 - XEXTENT.0), 10.0));
    commands
        .spawn_bundle(TransformBundle::from_transform(
            Transform::from_translation(Vec3::new(
                0.5 * (XEXTENT.0 + XEXTENT.1),
                YEXTENT.0 - 25.0,
                0.0,
            )),
        ))
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(0.5 * (XEXTENT.1 - XEXTENT.0), 10.0));

    // Player
    commands
        .spawn_bundle(PlayerBundle {
            party_radius: PartyRadius(40.0),
            ally: AllyBundle {
                ally_type: AllyType::Player,
                attack_range: AttackRange(60.0),
                attack_timer: AttackTimer(Timer::from_seconds(0.5, true)),
                damage: Damage(5.0),
                health: Health(100.0, 100.0),
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
        .insert(Collider::cuboid(8.0, 8.0))
        .insert(LockedAxes::ROTATION_LOCKED)
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

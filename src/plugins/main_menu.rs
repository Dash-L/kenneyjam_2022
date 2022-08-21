use bevy::{app::AppExit, audio::AudioSink, prelude::*};
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;
use iyes_loopless::prelude::*;

use crate::{
    components::{
        AllyBundle, AllyType, AnimationTimer, AttackRange, AttackTimer, Damage, Health,
        PartyRadius, Player, PlayerBundle,
    },
    consts::{SPRITE_SCALE, TRANSPARENT},
    helpers::{button_pressed, despawn_with, go_to_state, update_buttons},
    resources::{EnemyScale, EnemySpawnChance, Fonts, MusicController, Sounds, Sprites},
    GameState,
};

#[derive(Component)]
struct MainMenu;

#[derive(Component)]
struct PlayButton;

#[derive(Component)]
struct ExitButton;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MusicController>()
            .add_enter_system(GameState::MainMenu, setup_menu)
            .add_enter_system(GameState::MainMenu, spawn_player)
            .add_enter_system(GameState::MainMenu, start_menu_music)
            .add_exit_system(GameState::MainMenu, despawn_with::<MainMenu>)
            .add_exit_system(GameState::MainMenu, show_player)
            .add_exit_system(GameState::MainMenu, start_game_music)
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(GameState::MainMenu)
                    .with_system(
                        go_to_state(GameState::InGame).run_if(button_pressed::<PlayButton>),
                    )
                    .with_system(exit.run_if(button_pressed::<ExitButton>))
                    .with_system(update_buttons)
                    .into(),
            );
    }
}

fn setup_menu(mut commands: Commands, fonts: Res<Fonts>) {
    commands
        .spawn_bundle(NodeBundle {
            color: TRANSPARENT,
            style: Style {
                size: Size::new(Val::Auto, Val::Auto),
                margin: UiRect::all(Val::Auto),
                align_self: AlignSelf::Center,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::ColumnReverse,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            let button_style = Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                margin: UiRect::all(Val::Px(6.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            };

            let text_style = TextStyle {
                font: fonts.main.clone(),
                font_size: 40.0,
                color: Color::WHITE,
            };
            parent
                .spawn_bundle(ButtonBundle {
                    style: button_style.clone(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle::from_section("Play", text_style.clone()));
                })
                .insert(PlayButton);

            parent
                .spawn_bundle(ButtonBundle {
                    style: button_style.clone(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle::from_section("Exit", text_style.clone()));
                })
                .insert(ExitButton);
        })
        .insert(MainMenu);
}

fn spawn_player(
    mut commands: Commands,
    sprites: Res<Sprites>,
    mut spawn_chance: ResMut<EnemySpawnChance>,
    mut enemy_scale: ResMut<EnemyScale>,
) {
    spawn_chance.0 = 0.8;
    enemy_scale.0 = 1.0;
    commands
        .spawn_bundle(PlayerBundle {
            party_radius: PartyRadius(40.0),
            ally: AllyBundle {
                ally_type: AllyType::Player,
                attack_range: AttackRange(60.0),
                attack_timer: AttackTimer(Timer::from_seconds(0.5, true)),
                damage: Damage(10.0),
                health: Health(100.0, 100.0),
                sprite: SpriteSheetBundle {
                    texture_atlas: sprites.player.clone(),
                    transform: Transform::from_scale(Vec3::splat(SPRITE_SCALE))
                        .with_translation(Vec3::new(0., 0., 2.)),
                    visibility: Visibility { is_visible: false },
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
                transform: Transform::from_scale(Vec2::splat(0.5).extend(1.))
                    .with_translation(Vec3::Z * 997.9),
                ..default()
            });
        });
}

fn show_player(
    mut player: Query<&mut Visibility, With<Player>>,
    mut camera: Query<&mut Transform, With<Camera2d>>,
) {
    let mut visibility = player.single_mut();
    let mut transform = camera.single_mut();
    visibility.is_visible = true;
    transform.scale = Vec2::splat(0.25).extend(1.);
}

fn exit(mut ev: EventWriter<AppExit>) {
    ev.send(AppExit);
}

fn start_menu_music(
    audio_sinks: Res<Assets<AudioSink>>,
    audio: Res<Audio>,
    mut music_controller: ResMut<MusicController>,
    sound: Res<Sounds>,
) {
    if let Some(current) = audio_sinks.get(&music_controller.0) {
        current.stop();
    }
    let music = sound.menu.clone();
    let handle = audio_sinks
        .get_handle(audio.play_with_settings(music, PlaybackSettings::LOOP.with_volume(0.07)));
    music_controller.0 = handle;
}
fn start_game_music(
    audio_sinks: Res<Assets<AudioSink>>,
    audio: Res<Audio>,
    mut music_controller: ResMut<MusicController>,
    sound: Res<Sounds>,
) {
    if let Some(current) = audio_sinks.get(&music_controller.0) {
        current.stop();
    }
    let music = sound.game.clone();
    let handle = audio_sinks
        .get_handle(audio.play_with_settings(music, PlaybackSettings::LOOP.with_volume(0.07)));
    music_controller.0 = handle;
}

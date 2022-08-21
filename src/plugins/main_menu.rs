use bevy::{app::AppExit, audio::AudioSink, prelude::*};
use iyes_loopless::prelude::*;

use crate::{
    consts::TRANSPARENT,
    helpers::{button_pressed, despawn_with, go_to_state, update_buttons},
    resources::{Fonts, MusicController, Sounds, Sprites},
    GameState,
};

#[derive(Component)]
struct MainMenu;

#[derive(Component)]
struct PlayButton;

#[derive(Component)]
struct InfoButton;

#[derive(Component)]
struct ExitButton;

#[derive(Component)]
struct Info;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MusicController>()
            .add_enter_system(GameState::MainMenu, setup_menu)
            .add_enter_system(GameState::MainMenu, start_menu_music)
            .add_exit_system(GameState::MainMenu, despawn_with::<MainMenu>)
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(GameState::MainMenu)
                    .with_system(
                        go_to_state(GameState::InGame).run_if(button_pressed::<PlayButton>),
                    )
                    .with_system(go_to_state(GameState::Info).run_if(button_pressed::<InfoButton>))
                    .with_system(exit.run_if(button_pressed::<ExitButton>))
                    .with_system(update_buttons)
                    .into(),
            )
            .add_enter_system(GameState::Info, show_info)
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(GameState::Info)
                    .with_system(back_on_esc)
                    .into(),
            )
            .add_exit_system(GameState::Info, despawn_with::<Info>);
    }
}

fn show_info(mut commands: Commands, sprites: Res<Sprites>) {
    commands
        .spawn_bundle(Camera2dBundle {
            transform: Transform::from_scale(Vec2::splat(0.615).extend(1.0))
                .with_translation(Vec3::Z * 999.9),
            ..default()
        })
        .insert(Info);
    commands
        .spawn_bundle(SpriteBundle {
            texture: sprites.info.clone(),
            transform: Transform::from_translation(Vec3::Z * 10.0),
            ..default()
        })
        .insert(Info);
}

fn back_on_esc(mut commands: Commands, kbd: Res<Input<KeyCode>>) {
    if kbd.just_pressed(KeyCode::Escape) {
        commands.insert_resource(NextState(GameState::MainMenu));
    }
}

fn setup_menu(mut commands: Commands, fonts: Res<Fonts>) {
    commands
        .spawn_bundle(Camera2dBundle::default())
        .insert(MainMenu);
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
                    parent.spawn_bundle(TextBundle::from_section("Info", text_style.clone()));
                })
                .insert(InfoButton);

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

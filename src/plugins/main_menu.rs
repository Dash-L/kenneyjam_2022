use bevy::{app::AppExit, prelude::*};
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
    resources::{Fonts, Sprites},
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
        app.add_enter_system(GameState::MainMenu, setup_menu)
            .add_enter_system(GameState::MainMenu, spawn_player)
            .add_exit_system(GameState::MainMenu, despawn_with::<MainMenu>)
            .add_exit_system(GameState::MainMenu, show_player)
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

fn spawn_player(mut commands: Commands, sprites: Res<Sprites>) {
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
                transform: Transform::from_scale(Vec2::splat(0.25).extend(1.))
                    .with_translation(Vec3::Z * 997.9),
                ..default()
            });
        });
}

fn show_player(mut player: Query<&mut Visibility, With<Player>>) {
    let mut visibility = player.single_mut();
    visibility.is_visible = true;
}

fn exit(mut ev: EventWriter<AppExit>) {
    ev.send(AppExit);
}

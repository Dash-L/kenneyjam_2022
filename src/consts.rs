use bevy::prelude::*;

pub const WIDTH: f32 = 1280.0;
pub const HEIGHT: f32 = 720.0;
pub const SPRITE_SCALE: f32 = 2.0;
pub const XEXTENT: (f32, f32) = (-545.0, 545.0);
pub const YEXTENT: (f32, f32) = (-225.0, 250.0);
pub const HEALTH_BAR_LEN: f32 = 12.0;
pub const PROJECTILE_SPEED: f32 = 750.0;
pub const BUTTON_CLICKED: UiColor = UiColor(Color::BLUE);
pub const BUTTON_HOVERED: UiColor = UiColor(Color::GRAY);
pub const BUTTON_DEFAULT: UiColor = UiColor(Color::BLACK);
pub const TRANSPARENT: UiColor = UiColor(Color::rgba(0.0, 0.0, 0.0, 0.0));

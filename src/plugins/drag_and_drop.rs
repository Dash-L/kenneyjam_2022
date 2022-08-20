use bevy::prelude::*;

use crate::{
    components::{Collider, InParty},
    resources::DraggingEntity,
};

pub struct DragAndDropPlugin;

impl Plugin for DragAndDropPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DraggingEntity(None));
    }
}

fn set_dragging(
    mut commands: Commands,
    dragging_entity: Res<DraggingEntity>,
    mouse: Res<Input<MouseButton>>,
    cursor_motion_ev: EventReader<CursorMoved>,
    draggables: Query<(Entity, &Transform, &Collider), With<InParty>>,
) {
    if dragging_entity.is_none() {
        if mouse.just_pressed(MouseButton::Left) {}
    }
}

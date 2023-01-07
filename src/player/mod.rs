pub(crate) mod input;

use bevy::prelude::{Component, In, Input, KeyCode, Res, Vec2};
use bevy_ggrs::ggrs::PlayerHandle;
use crate::camera::CursorWorldPos;
use crate::player::input::PlayerControls;


#[derive(Component)]
pub struct Player{
    pub handle: usize
}


pub fn input(
    _: In<PlayerHandle>,
    keys: Res<Input<KeyCode>>,
    mouse_pos: Res<CursorWorldPos>,
) -> PlayerControls {
    let mut action_vars = 0u32;

    let mut direction = Vec2::ZERO;
    if keys.any_pressed([KeyCode::Up, KeyCode::W]) {
        direction.y += 1.;
    }
    if keys.any_pressed([KeyCode::Down, KeyCode::S]) {
        direction.y -= 1.;
    }
    if keys.any_pressed([KeyCode::Right, KeyCode::D]) {
        direction.x += 1.;
    }
    if keys.any_pressed([KeyCode::Left, KeyCode::A]) {
        direction.x -= 1.;
    }

    if keys.pressed(KeyCode::Space) {
        //other_actions |= TEST;
    }

    PlayerControls {
        move_direction: direction,
        action_vars,
        mouse_position: mouse_pos.cursor_world_pos,
    }
}
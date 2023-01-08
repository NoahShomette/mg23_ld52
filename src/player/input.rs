use bevy::prelude::{In, Input, KeyCode, Reflect, Res, Resource, Vec2};
use bevy_ggrs::ggrs::PlayerHandle;
use bytemuck::{Pod, Zeroable};
use crate::camera::CursorWorldPos;
use crate::player::PlayerId;

// What actions do we need

/*
    A variable amount of spells - 1-4
            a spell needs two things, the type of spell, and then a location, target, direction, or something else
    move left, right, up, down
    dash
            Direction. Maybe just do it in the move direction?
    shield
            a shield needs the direction its cast in. The mouse position?
 */


// the player has autoattacked
const AUTOATTACK: u32 = 1 << 0;
// the player has dashed in their move direction
const DASH: u32 = 1 << 1;
// the player is shielding in the direction of their mouse
const SHIELD: u32 = 1 << 2;
// the player has cast a spell, using the information in their mouse_position
const CAST_SPELL: u32 = 1 << 3;


pub enum SpellType{
    SelfCast,
    Directional(Vec2),
    Location(Vec2),
    Targeted(PlayerId),
}

#[derive(Pod, Zeroable, Copy, Clone, PartialEq, Reflect, Resource)]
#[repr(C)]
pub struct PlayerControls {
    // the direction the player is requesting to move in
    pub move_direction: Vec2,
    // The separate and different action_vars the player has, and might be doing
    pub action_vars: u32,
    // The spell_type that the player has cast, if its not 0
    pub cast_spell_type: u32,
    // the mouse position - used for relevant info
    pub mouse_position: Vec2,
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
        cast_spell_type: 0,
        mouse_position: mouse_pos.cursor_world_pos,
    }
}
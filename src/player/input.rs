use bevy::input::mouse::MouseButtonInput;
use crate::camera::CursorWorldPos;
use crate::player::{LocalPlayer, MovementState, PlayerCombatState, PlayerId, PlayerMovementState, PlayerSpells, SpellCastState};
use bevy::prelude::{In, Input, KeyCode, MouseButton, Query, Reflect, Res, ResMut, Resource, Vec2};
use bevy_ggrs::ggrs::PlayerHandle;
use bytemuck::{Pod, Zeroable};

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
pub const AUTOATTACK: u32 = 1 << 0;
// the player has dashed in their move direction
pub const DASH: u32 = 1 << 1;
// the player is shielding in the direction of their mouse
pub const SHIELD: u32 = 1 << 2;
// the player has cast a spell, using the information in their mouse_position
pub const CAST_SPELL: u32 = 1 << 3;

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
    player_handle: In<PlayerHandle>,
    keys: Res<Input<KeyCode>>,
    mouse: Res<Input<MouseButton>>,
    mouse_pos: Res<CursorWorldPos>,
    mut player_movement_query: Query<(
        &PlayerId,
        &PlayerMovementState,
        &mut PlayerCombatState,
        &PlayerSpells,
    )>,
    mut local_handle: ResMut<LocalPlayer>
) -> PlayerControls {
    let mut action_vars = 0u32;
    let mut cast_spell = 0u32;
    let mut direction = Vec2::ZERO;

    for (id, state, mut combat_state, spells) in player_movement_query.iter_mut() {
        if id.handle == player_handle.0 {
            local_handle.handle_id = player_handle.0;
            match state.movement_state {
                MovementState::Dashing {
                    duration,
                    direction,
                } => {}
                MovementState::Walking => {
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

                    if keys.just_pressed(KeyCode::Space) && state.can_dash {
                        action_vars |= DASH;
                    }
                }
                MovementState::Idle => {
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
                }
            }

            if keys.pressed(KeyCode::Key1) {
                combat_state.spell_cast_state = SpellCastState::Precast {
                    spell_id: spells.spells[0].clone(),
                }
            }
            if let SpellCastState::Precast {spell_id} = &combat_state.spell_cast_state{
                if mouse.pressed(MouseButton::Left)  {
                    action_vars |= CAST_SPELL;
                    cast_spell = spell_id.spell_id.id;
                } else if mouse.pressed(MouseButton::Right){
                    combat_state.spell_cast_state = SpellCastState::None;
                }
            }

        }
    }
    PlayerControls {
        move_direction: direction,
        action_vars,
        cast_spell_type: cast_spell,
        mouse_position: mouse_pos.cursor_world_pos,
    }
}

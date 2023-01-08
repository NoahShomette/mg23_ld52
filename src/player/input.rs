use bevy::prelude::{Reflect, Resource, Vec2};
use bytemuck::{Pod, Zeroable};
use crate::player::Player;

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
    Targeted(Player),
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
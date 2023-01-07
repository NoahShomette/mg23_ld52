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

//
const AUTOATTACK: u32 = 1 << 0;
const DASH: u32 = 1 << 1;
const SHIELD: u32 = 1 << 2;
const SPELL_ONE: u32 = 1 << 3;
const SPELL_TWO: u32 = 1 << 4;
const SPELL_THREE: u32 = 1 << 5;
const SPELL_FOUR: u32 = 1 << 6;


pub enum SpellType{
    SelfCast,
    Directional(Vec2),
    Location(Vec2),
    Targeted(Player),
}

#[derive(Pod, Zeroable, Copy, Clone, PartialEq, Reflect, Resource)]
#[repr(C)]
pub struct PlayerControls {
    pub move_direction: Vec2,
    pub action_vars: u32,
    pub mouse_position: Vec2,
}
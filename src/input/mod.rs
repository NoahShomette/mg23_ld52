use bevy::prelude::{Reflect, Resource, Vec2};
use bytemuck::{Pod, Zeroable};
use crate::player::Player;

// 
const SPELL_ONE: u32 = 1 << 0;
const SPELL_TWO: u32 = 1 << 1;
const SPELL_THREE: u32 = 1 << 2;
const SPELL_FOUR: u32 = 1 << 3;
const DASH: u32 = 1 << 4;
const TEST: u32 = 1 << 5;

pub enum SpellType{
    SelfCast,
    Directional(Vec2),
    Location(Vec2),
    Targeted(Player),
}

#[derive(Pod, Zeroable, Copy, Clone, PartialEq, Reflect)]
#[repr(C)]
pub struct NetworkSpellType{
}

#[derive(Pod, Zeroable, Copy, Clone, PartialEq, Reflect)]
#[repr(C)]
pub struct SpellCast{
    spell: u32,
    spell_type: NetworkSpellType,
}

#[derive(Pod, Zeroable, Copy, Clone, PartialEq, Reflect, Resource)]
#[repr(C)]
pub struct PlayerControls {
    pub move_direction: Vec2,
    pub spells: SpellCast,
    pub other_actions: u32,
}
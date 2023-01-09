pub(crate) mod input;

use crate::physics::Movement;
use crate::spell::SpellCastInfo;
use bevy::math::Vec2;
use bevy::prelude::{Bundle, Component, FromReflect, Reflect, SpriteBundle};
use bevy::sprite::MaterialMesh2dBundle;
use bevy_ggrs::Rollback;
use bevy_sepax2d::prelude::{Movable, Sepax};
use bevy_simple_2d_outline::OutlineAndTextureMaterial;

#[derive(Bundle)]
pub struct PlayerBundle {
    // network stuff
    pub player_id: PlayerId,
    pub rollback_id: Rollback,
    //spell stuff
    pub player_spells: PlayerSpells,
    //player state
    pub player_movement: PlayerMovementStats,
    pub player_movement_state: PlayerMovementState,
    pub health: Health,
    pub team_id: TeamId,
    // assorted
    pub sepax: Sepax,
    pub movable: Movable,
    pub movement: Movement,

    pub mesh_bundle: MaterialMesh2dBundle<OutlineAndTextureMaterial>,
}

pub struct PlayerState {}

#[derive(Reflect, Default, Component, Debug, Copy, Clone, PartialEq)]
pub struct PlayerCombatState {
    spell_cast_state: SpellCastState,
}

#[derive(Reflect, Default, Debug, Copy, Clone, PartialEq)]
pub enum SpellCastState {
    #[default]
    None,
    Precast,
    Cast,
}

#[derive(Reflect, Default, Component, Debug, Copy, Clone, PartialEq)]
pub struct PlayerMovementState {
    pub can_dash: bool,
    pub dash_cooldown: f32,
    pub movement_state: MovementState,
}

#[derive(Reflect, Default, Component, Debug, Copy, Clone, PartialEq)]
pub struct PlayerMovementStats {
    pub speed: f32,
    pub dash_power: f32,
    pub dash_duration: f32,
    pub dash_cooldown_length: f32,
}

// not used currently
#[derive(Reflect, Default, Component, Debug, Copy, Clone, PartialEq)]
pub struct PlayerDashInfo {
    pub dash_duration: f32,
    pub dash_cooldown_length: f32,
    pub dash_cooldown: f32,
}

#[derive(Reflect, Component, Debug, Copy, Clone, PartialEq)]

pub enum MovementState {
    Dashing { duration: f32, direction: Vec2 },
    Walking,
    Idle,
}

impl Default for MovementState {
    fn default() -> Self {
        MovementState::Walking
    }
}

#[derive(Component)]
pub struct PlayerSpells {
    pub autoattack: SpellCastInfo,
    //pub shield: SpellCastInfo,
    pub spells: Vec<SpellCastInfo>,
}

#[derive(FromReflect, Reflect, Eq, Debug, PartialEq, PartialOrd, Ord, Copy, Clone, Component)]
pub struct PlayerId {
    pub handle: usize,
}

pub enum PlayerActions {
    Shield,
    Dash,
    CastSpell,
}

#[derive(FromReflect, Reflect, Default, Eq, PartialEq, Debug, PartialOrd, Ord, Copy, Clone, Component)]
pub struct Health {
    pub max_health: u32,
    pub current_health: u32,
}

impl Health {}

#[derive(FromReflect, Reflect, Default, Eq, PartialEq, Debug, PartialOrd, Ord, Copy, Clone, Component)]
pub struct TeamId{
    pub id: usize,
}
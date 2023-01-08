pub(crate) mod input;

use crate::physics::Movement;
use crate::spell::Spell;
use bevy::prelude::{Bundle, Component, Reflect, SpriteBundle};
use bevy_ggrs::Rollback;
use bevy_sepax2d::prelude::{Movable, Sepax};

#[derive(Bundle)]
pub struct PlayerBundle {
    pub(crate) player_id: PlayerId,
    pub(crate) rollback_id: Rollback,
    pub(crate) player_spells: PlayerSpells,
    pub(crate) player_movement: PlayerMovementStats,
    pub player_movement_state: PlayerMovementState,
    pub(crate) sepax: Sepax,
    pub movable: Movable,
    pub movement: Movement,
    pub sprite_bundle: SpriteBundle,
}

#[derive(Reflect, Default, Component, Debug, Copy, Clone, PartialEq)]
pub struct PlayerMovementStats {
    pub speed: f32,
    pub dash_power: f32,
    pub dash_duration: f32,
    pub dash_cooldown_length: f32,
    pub dash_cooldown: f32,
}

#[derive(Reflect, Default, Component, Debug, Copy, Clone, PartialEq)]
pub struct PlayerMovementState {
    pub can_dash: bool,
    pub movement_state: MovementState,
}

#[derive(Reflect, Default, Component, Debug, Copy, Clone, PartialEq)]

pub enum MovementState {
    Dashing { duration: f32 },
    #[default]
    Walking,
    Idle,
}

#[derive(Component)]
pub struct PlayerSpells {
    pub autoattack: Spell,
    pub shield: Spell,
    pub spells: Vec<Spell>,
}

#[derive(Eq, PartialEq, PartialOrd, Ord, Copy, Clone, Component)]
pub struct PlayerId {
    pub handle: usize,
}

pub enum PlayerActions {
    Shield,
    Dash,
    CastSpell,
}

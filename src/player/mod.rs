pub(crate) mod input;

use crate::physics::Movement;
use crate::spell::{SpellCastInfo};
use bevy::math::Vec2;
use bevy::prelude::{Bundle, Component, FromReflect, Query, Reflect, Resource, With};
use bevy_aseprite::anim::AsepriteAnimation;
use bevy_aseprite::AsepriteBundle;
use bevy_ggrs::Rollback;
use bevy_sepax2d::prelude::{Movable, Sepax};

pub fn update_animation_state(
    mut query: Query<
        (
            &PlayerMovementState,
            &mut AnimationState,
            &mut AsepriteAnimation,
        ),
        With<PlayerId>,
    >,
) {
    for (state, mut animation_state, mut aseprite_animation) in query.iter_mut() {
        match state.movement_state {
            MovementState::Dashing {
                duration,
                direction,
            } => {
                if AnimationState::Dash != *animation_state {
                    *aseprite_animation = AsepriteAnimation::from("Dash");
                    *animation_state = AnimationState::Dash;
                }
            }
            MovementState::Walking => {
                if AnimationState::Run != *animation_state {
                    *aseprite_animation = AsepriteAnimation::from("Run");
                    *animation_state = AnimationState::Run;
                }
            }
            MovementState::Idle => {
                if AnimationState::Idle != *animation_state {
                    *aseprite_animation = AsepriteAnimation::from("Idle");
                    *animation_state = AnimationState::Idle;
                }
            }
        }
    }
}


#[derive(Bundle)]
pub struct PlayerBundle {
    // network stuff
    pub player_id: PlayerId,
    pub rollback_id: Rollback,
    //spell stuff
    pub player_spells: PlayerSpells,
    pub combat_state: PlayerCombatState,
    //player state
    pub player_movement: PlayerMovementStats,
    pub player_movement_state: PlayerMovementState,
    pub health: Health,
    pub team_id: TeamId,
    // assorted
    pub sepax: Sepax,
    pub movable: Movable,
    pub movement: Movement,

    pub aseprite_bundle: AsepriteBundle,
    pub animation_state: AnimationState,
}

pub struct PlayerState {}

#[derive(Reflect, Default, Component, Debug, PartialEq)]
pub struct PlayerCombatState {
    pub spell_cast_state: SpellCastState,
}

#[derive(Reflect, Default, Resource, Debug, PartialEq)]
pub struct LocalPlayer {
    pub handle_id: usize,
}

#[derive(Reflect, Default, Debug, PartialEq)]
pub enum SpellCastState {
    #[default]
    None,
    Precast {
        spell_id: SpellCastInfo,
    },
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
        MovementState::Idle
    }
}

#[derive(Reflect, Component, Debug, Copy, Clone, PartialEq)]
pub enum AnimationState {
    Dash,
    Run,
    Idle,
}

impl Default for AnimationState {
    fn default() -> Self {
        AnimationState::Idle
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

#[derive(
    FromReflect, Reflect, Default, Eq, PartialEq, Debug, PartialOrd, Ord, Copy, Clone, Component,
)]
pub struct Health {
    pub max_health: u32,
    pub current_health: u32,
}

impl Health {}

#[derive(
    FromReflect, Reflect, Default, Eq, PartialEq, Debug, PartialOrd, Ord, Copy, Clone, Component,
)]
pub struct TeamId {
    pub id: usize,
}

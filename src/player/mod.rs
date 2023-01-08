pub(crate) mod input;

use bevy::prelude::{Component};
use crate::spell::Spell;


pub struct PlayerBundle{
    player_id: PlayerId,
    player_spells: PlayerSpells,
    player_movement: PlayerMovement,
}

#[derive(Component)]
pub struct PlayerMovement{
    pub speed: f32,
    pub dash_power: f32,
}

#[derive(Component)]
pub struct PlayerSpells{
    pub autoattack: Spell,
    pub shield: Spell,
    pub spells: Vec<Spell>,

}

#[derive(Component)]
pub struct PlayerId {
    pub handle: usize
}

pub enum PlayerActions{
    Shield,
    Dash,
    CastSpell,
}


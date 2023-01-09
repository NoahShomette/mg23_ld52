use bevy::log::info;
use crate::assets::SpellSprites;
use crate::player::{PlayerId, PlayerSpellBuffer};
use crate::spell::{DamageDealer, DamageSpellProjectileBundle, SpellCasterId, SpellLifetime};
use bevy::prelude::{Commands, default, Res, ResMut, Transform};
use bevy_aseprite::anim::AsepriteAnimation;
use bevy_aseprite::AsepriteBundle;
use bevy_ggrs::{Rollback, RollbackIdProvider};
use bevy_sepax2d::Convex;
use bevy_sepax2d::prelude::{Movable, Sepax};
use sepax2d::prelude::Circle;

pub fn handle_spell_buffer(
    mut commands: Commands,
    game_spells: Res<SpellSprites>,
    mut rip: ResMut<RollbackIdProvider>,
    mut spell_buffer: ResMut<PlayerSpellBuffer>,
) {
    // collect and sort for determinism
    //let mut spell_actions = spell_buffer.actions;
    //info!("Spell Buffer length: {:?}", spell_buffer.actions.len());

    for spell_action in spell_buffer.actions.iter().rev() {
        //info!("{:?}", spell_action.1.length());
       
    }
    spell_buffer.actions.clear();
}

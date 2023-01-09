use bevy::app::App;
use bevy::prelude::{Component, FromReflect, Handle, Image, Plugin, Reflect, Resource, Vec2};
use bevy::utils::HashMap;
use crate::player::PlayerId;


pub struct SpellPlugin;

impl Plugin for SpellPlugin {
    fn build(&self, app: &mut App) {
        
    }
}

// spells have two parts that are not connected perse
// the cast section of a spell, this controls all the cast timings, etc. This isnt set per a spell technically but basically would always be unique per a spell
// this controls the spell cooldown, how long it takes to cast, etc
// the second part is the spell entity and all its components. These control the actual spell qualities, its shapes, how it spawns, etc

#[derive(FromReflect, Reflect, PartialEq, Debug, Clone)]
pub enum SpellType {
    SelfCast,
    Directional(Vec2),
    Location(Vec2),
    Targeted(PlayerId),
}

#[derive(FromReflect, Reflect, Default, PartialEq, Debug, Clone, Resource)]
pub struct GameSpells{
    spell_cast_info: HashMap<u32, SpellCastInfo>,
}

#[derive(FromReflect, Reflect, PartialEq, Debug, Clone)]
pub struct SpellCastInfo {
    pub spell_type: SpellType,
    pub cooldown: f32,
    pub cast_spell: SpellProjectileInfo,
    pub spell_indicator: Handle<Image>,
}

#[derive(FromReflect, Reflect, Default, Eq, PartialEq, Debug, PartialOrd, Ord, Clone, Component)]
pub struct SpellProjectileInfo {
    pub id: usize,
}



pub fn show_spell_preview(){
    
}
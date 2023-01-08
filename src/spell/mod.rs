use bevy::prelude::Resource;
use bevy::utils::HashMap;

#[derive(Resource)]
pub struct GameSpells{
    spells: HashMap<u32, Spell>,
}

pub struct Spell{
    
}
use bevy::prelude::*;
use bevy_asset_loader::prelude::AssetCollection;


#[derive(AssetCollection, Resource)]
pub struct Sprites {
    #[asset(path = "magelings/Basic-Mageling_green.png")]
    pub mageling_green: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct SpellSprites {
    #[asset(path = "spells_art/Circle-Indicator.png")]
    pub circle_indicator: Handle<Image>,
}
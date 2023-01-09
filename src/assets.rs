use bevy::prelude::*;
use bevy_aseprite::Aseprite;
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
    #[asset(path = "spells_art/Explosion_indicator.aseprite")]
    pub explosion_spell: Handle<Aseprite>,
}

#[derive(AssetCollection, Resource)]
pub struct MenuSprites {
    #[asset(path = "menu/button.png")]
    pub button: Handle<Image>,
    #[asset(path = "menu/button_click.png")]
    pub button_click: Handle<Image>,
    #[asset(path = "menu/button_highlight.png")]
    pub button_hover: Handle<Image>,
    //#[asset(path = "spells_art/Explosion_indicator.aseprite")]
    //pub explosion_spell: Handle<Aseprite>,
}
use bevy::prelude::*;
use bevy_asset_loader::prelude::AssetCollection;


#[derive(AssetCollection, Resource)]
pub struct Sprites {
    #[asset(path = "magelings/Basic-Mageling_green.png")]
    pub mageling_green: Handle<Image>,
}
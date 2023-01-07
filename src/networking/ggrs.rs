use bevy_ggrs::ggrs::{Config};
use crate::player::input::PlayerControls;


#[derive(Debug)]
pub struct GGRSConfig;
impl Config for GGRSConfig {
    type Input = PlayerControls;
    type State = u8;
    type Address = String;
}
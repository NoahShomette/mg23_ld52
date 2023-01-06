use bevy::prelude::*;
use crate::camera::CamPlugin;

mod networking;
mod input;
mod assets;
mod player;
mod camera;

const FPS: usize = 60;

fn main() {
    let mut app = App::new();
    
    app.add_plugin(CamPlugin);
        
        
        
    app.run();
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    AssetLoading,
    Menu,
    WaitingForPlayers,
    BetweenRound,
    InRound,
}
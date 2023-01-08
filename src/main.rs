use crate::assets::Sprites;
use crate::camera::CamPlugin;
use crate::networking::ggrs::GGRSConfig;
use crate::networking::{move_players, NetworkPlugin, start_matchbox_socket, wait_for_players};
use crate::player::{input, Player};
use bevy::prelude::*;
use bevy::window::close_on_esc;
use bevy_asset_loader::prelude::{LoadingState, LoadingStateAppExt};
use bevy_ggrs::{GGRSPlugin, Rollback, RollbackIdProvider};
use bevy_tiled_camera::{TiledCameraBundle, TiledCameraPlugin, WorldSpace};
use iyes_loopless::prelude::{AppLooplessStateExt, IntoConditionalSystem, NextState};

mod assets;
mod camera;
mod networking;
mod player;
mod spell;

const FPS: usize = 60;

fn main() {
    let mut app = App::new();

    GGRSPlugin::<GGRSConfig>::new()
        // define frequency of rollback game logic update
        //.with_update_frequency(FPS)
        // define system that returns inputs given a player handle, so GGRS can send the inputs around
        .with_input_system(input)
        // register types of components AND resources you want to be rolled back
        .register_rollback_component::<Transform>()
        // these systems will be executed as part of the advance frame update
        .with_rollback_schedule(Schedule::default().with_stage(
            "ROLLBACK_STAGE",
            SystemStage::single_threaded().with_system(move_players),
        ))
        // make it happen in the bevy app
        .build(&mut app);

    app.add_loopless_state(GameState::AssetLoading)
        .add_loading_state(
            LoadingState::new(GameState::AssetLoading)
                .continue_to_state(GameState::WaitingForPlayers)
                .with_collection::<Sprites>(),
        )
        .add_state(GameState::AssetLoading)
        //base plugins
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        mode: WindowMode::Windowed,
                        fit_canvas_to_parent: true,
                        ..default()
                    },
                    ..default()
                }),
        )
        .add_plugin(TiledCameraPlugin)
        // base systems
        .add_enter_system(GameState::WaitingForPlayers, setup)
        .add_enter_system(GameState::WaitingForPlayers, start_matchbox_socket)
        .add_system(wait_for_players.run_in_state(GameState::WaitingForPlayers))
        .add_enter_system(GameState::BetweenRound, spawn_players);
    //

    // crate plugins
    app.add_plugin(CamPlugin).add_plugin(NetworkPlugin);

    app.add_system(close_on_esc);

    app.run();
}


/// The game state
/// 
/// - AssetLoading starts at the beginning of every time the app is launched, runs all the asset stuff, and then is never used again
/// - Menu is used for the main menu and associated places
/// - WaitingForPlayers is the pregame, menu, lobby. Eg, the player selects find match, it goes to 
///     waiting for players, the player can either quit the matchmaking, or wait to find a match
/// - BetweenRound is the period between fighting rounds. The players are spawned, the game countdowns till the round starts, the players see the map, etc
/// - InRound is the actual gameplay. It starts, the players are given control of their characters, and it plays until the round is ended
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    AssetLoading,
    Menu,
    WaitingForPlayers,
    BetweenRound,
    InRound,
    PostMatch,
}

fn spawn_players(
    sprites: Res<Sprites>,
    mut commands: Commands,
    mut rip: ResMut<RollbackIdProvider>,
) {
    commands
        .spawn(SpriteBundle {
            transform: Transform {
                translation: Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 1.0,
                },
                rotation: Default::default(),
                scale: Vec3 {
                    x: 1.0,
                    y: 1.0,
                    z: 1.0,
                },
            },
            texture: sprites.mageling_green.clone_weak(),
            ..default()
        })
        .insert(Player { handle: 0 })
        .insert(Rollback::new(rip.next_id()));

    commands
        .spawn(SpriteBundle {
            transform: Transform {
                translation: Vec3 {
                    x: 10.0,
                    y: 10.0,
                    z: 1.0,
                },
                rotation: Default::default(),
                scale: Vec3 {
                    x: 1.0,
                    y: 1.0,
                    z: 1.0,
                },
            },
            texture: sprites.mageling_green.clone_weak(),
            ..default()
        })
        .insert(Player { handle: 1 })
        .insert(Rollback::new(rip.next_id()));

    commands.insert_resource(NextState(GameState::InRound))
}

fn setup(mut commands: Commands) {
    commands.spawn(
        TiledCameraBundle::new()
            .with_pixels_per_tile([24, 24])
            .with_tile_count([26, 15])
            .with_world_space(WorldSpace::Pixels),
    );
}

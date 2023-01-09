use crate::assets::{SpellSprites, Sprites};
use crate::camera::{CamPlugin, CameraBundle};
use crate::networking::ggrs::GGRSConfig;
use crate::networking::rollback_systems::{move_players, update_dash_info, velocity_system};
use crate::networking::{
    start_matchbox_socket, wait_for_players, NetworkPlugin, RoomNetworkSettings,
};
use crate::physics::{clear_correction_system, collision_system, update_movable_system, Movement};
use crate::player::input::input;
use crate::player::{Health, MovementState, PlayerBundle, PlayerId, PlayerMovementState, PlayerMovementStats, PlayerSpells};
use crate::spell::{SpellCastInfo, SpellType, SpellProjectileInfo, GameSpells};
use bevy::prelude::*;
use bevy::window::close_on_esc;
use bevy_asset_loader::prelude::{LoadingState, LoadingStateAppExt};
use bevy_ecs_ldtk::{
    LdtkLevel, LdtkPlugin, LdtkSettings, LdtkWorldBundle, LevelSelection, LevelSpawnBehavior,
};
use bevy_ecs_ldtk::prelude::RegisterLdtkObjects;
use bevy_ggrs::{GGRSPlugin, Rollback, RollbackIdProvider};
use bevy_sepax2d::prelude::{Movable, Sepax};
use bevy_sepax2d::Convex;
use bevy_tiled_camera::TiledCameraPlugin;
use iyes_loopless::prelude::{AppLooplessStateExt, IntoConditionalSystem, NextState};
use sepax2d::prelude::AABB;
use crate::map::WallCollisions;

mod assets;
mod camera;
mod game_state;
mod networking;
mod physics;
mod player;
mod spell;
mod map;

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
        .register_rollback_component::<Movement>()
        .register_rollback_component::<Health>()
        .register_rollback_component::<PlayerMovementStats>()
        .register_rollback_component::<PlayerMovementState>()
        //resources
        .register_rollback_resource::<GameSpells>()
        // these systems will be executed as part of the advance frame update
        .with_rollback_schedule(
            Schedule::default().with_stage(
                "ROLLBACK_STAGE",
                SystemStage::single_threaded()
                    .with_system(move_players)
                    .with_system(velocity_system.after(move_players))
                    .with_system(update_dash_info.after(velocity_system))
                    // physics stuff - need to be at the end
                    .with_system(clear_correction_system.after(update_dash_info))
                    .with_system(update_movable_system.after(clear_correction_system))
                    .with_system(collision_system.after(update_movable_system)),
            ),
        )
        // make it happen in the bevy app
        .build(&mut app);

    app.add_loopless_state(GameState::AssetLoading)
        .add_loading_state(
            LoadingState::new(GameState::AssetLoading)
                .continue_to_state(GameState::WaitingForPlayers)
                .with_collection::<Sprites>()
                .with_collection::<SpellSprites>(),
        )
        .add_state(GameState::AssetLoading)
        //base plugins
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        title: "Magelings".to_string(),
                        position: WindowPosition::Automatic,
                        fit_canvas_to_parent: true,
                        canvas: Some("#bevy".to_string()),
                        ..default()
                    },
                    ..default()
                }),
        )
        .add_plugin(TiledCameraPlugin)
        .insert_resource(LdtkSettings {
            level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                load_level_neighbors: true,
            },
            ..default()
        })
        .add_plugin(LdtkPlugin)
        .register_ldtk_int_cell::<WallCollisions>(1)

        .insert_resource(LevelSelection::Index(0))
        // base systems
        .add_enter_system(GameState::WaitingForPlayers, setup)
        .add_enter_system(GameState::WaitingForPlayers, start_matchbox_socket)
        .add_system(wait_for_players.run_in_state(GameState::WaitingForPlayers))
        .add_enter_system(GameState::BetweenRound, spawn_players);
    //

    // crate plugins
    app.add_plugin(CamPlugin).add_plugin(NetworkPlugin);

    // Debug stuff
    //app.add_plugin(RapierDebugRenderPlugin::default());
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
    spell_sprites: Res<SpellSprites>,

    mut commands: Commands,
    mut rip: ResMut<RollbackIdProvider>,
    mut query: Query<&mut Transform, With<Handle<LdtkLevel>>>,
    asset_server: Res<AssetServer>,
    settings: Res<RoomNetworkSettings>,
) {
    for mut transform in query.iter_mut() {
        info!("This is called");
        transform.translation.y -= 180.0;
        //transform.translation.z -= 50.0;
    }

    for i in 0..settings.player_count {
        commands.spawn(PlayerBundle {
            player_id: PlayerId { handle: i as usize },
            rollback_id: Rollback::new(rip.next_id()),
            player_spells: PlayerSpells {
                autoattack: SpellCastInfo {
                    spell_type: SpellType::SelfCast,
                    cooldown: 0.0,
                    cast_spell: SpellProjectileInfo {
                        id: 0
                    },
                    spell_indicator: spell_sprites.circle_indicator.clone_weak(),
                },
                spells: vec![],
            },
            player_movement: PlayerMovementStats {
                speed: 130.0,
                dash_power: 3.0,
                dash_duration: 0.2,
                dash_cooldown_length: 5.0,
            },
            player_movement_state: PlayerMovementState {
                can_dash: true,
                dash_cooldown: 0.0,
                movement_state: MovementState::default(),
            },
            health: Health{
                max_health: 100,
                current_health: 100,
            },
            sepax: Sepax {
                convex: Convex::AABB(AABB::new((0.0, 0.0 + (i as f32 * 20.0)), 20.0, 20.0)),
            },
            movable: Movable { axes: vec![] },
            movement: Default::default(),
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: Vec3 {
                        x: 0.0,
                        y: 0.0 + (i as f32 * 20.0),
                        z: 30.0,
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
            },
        });
    }

    commands.spawn((
        Sepax {
            convex: Convex::AABB(AABB::new((0.0, 0.0 + (4 as f32 * 20.0)), 20.0, 20.0)),
        },
        SpriteBundle {
            transform: Transform {
                translation: Vec3 {
                    x: 0.0,
                    y: 0.0 + (4 as f32 * 20.0),
                    z: 30.0,
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
        },
    ));

    commands.insert_resource(NextState(GameState::InRound))
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("ldtk/map.ldtk"),
        level_set: Default::default(),
        ..default()
    });

    commands.spawn(CameraBundle::default());
}

use crate::assets::{MenuSprites, SpellSprites, Sprites};
use crate::camera::{CamPlugin, CameraBundle};
use crate::combat::handle_spell_buffer;
use crate::map::{SpawnPoint, Wall, WallCollisions};
use crate::networking::ggrs::GGRSConfig;
use crate::networking::rollback_systems::{
    handle_spell_casts, move_players, spell_collision_system, update_dash_info,
    update_spell_lifetimes, velocity_system,
};
use crate::networking::{
    start_matchbox_socket, wait_for_players, NetworkPlugin, RoomNetworkSettings,
};
use crate::physics::{
    clear_correction_system, collision_system, update_movable_system, update_walls_system, Movement,
};
use crate::player::input::input;
use crate::player::{
    update_animation_state, AnimationState, Health, LocalPlayer, MovementState, PlayerBundle,
    PlayerCombatState, PlayerId, PlayerMovementState, PlayerMovementStats, PlayerSpellBuffer,
    PlayerSpells, TeamId,
};
use crate::spell::{
    DamageDealer, DamageSpellProjectileBundle, GameSpells, SpellCastInfo, SpellCasterId,
    SpellLifetime, SpellPlugin, SpellType,
};
use crate::ui::UiPlugin;
use bevy::prelude::*;
use bevy::sprite::Material2dPlugin;
use bevy::window::close_on_esc;
use bevy_aseprite::anim::AsepriteAnimation;
use bevy_aseprite::{AsepriteBundle, AsepritePlugin};
use bevy_asset_loader::prelude::{LoadingState, LoadingStateAppExt};
use bevy_ecs_ldtk::prelude::RegisterLdtkObjects;
use bevy_ecs_ldtk::{
    LdtkLevel, LdtkPlugin, LdtkSettings, LdtkWorldBundle, LevelSelection, LevelSpawnBehavior,
};
use bevy_ggrs::{GGRSPlugin, Rollback, RollbackIdProvider};
use bevy_sepax2d::prelude::{Movable, Sepax};
use bevy_sepax2d::Convex;
use bevy_simple_2d_outline::OutlineAndTextureMaterial;
use bevy_tiled_camera::TiledCameraPlugin;
use iyes_loopless::prelude::{AppLooplessStateExt, ConditionSet, IntoConditionalSystem, NextState};
use sepax2d::prelude::{Circle, AABB};

mod assets;
mod camera;
mod combat;
mod game_state;
mod map;
mod networking;
mod physics;
mod player;
mod spell;
mod ui;

const FPS: usize = 60;

fn main() {
    let mut app = App::new();

    GGRSPlugin::<GGRSConfig>::new()
        // define frequency of rollback game logic update
        .with_update_frequency(FPS)
        // define system that returns inputs given a player handle, so GGRS can send the inputs around
        .with_input_system(input)
        // register types of components AND resources you want to be rolled back
        .register_rollback_component::<Transform>()
        .register_rollback_component::<Movement>()
        .register_rollback_component::<Health>()
        .register_rollback_component::<PlayerMovementStats>()
        .register_rollback_component::<PlayerMovementState>()
        .register_rollback_component::<AnimationState>()
        .register_rollback_component::<PlayerCombatState>()
        .register_rollback_component::<SpellLifetime>()
        //.register_rollback_component::<PlayerCombatState>()
        //resources
        .register_rollback_resource::<PlayerSpellBuffer>()
        //.register_rollback_resource::<NetworkIdProvider>()
        // these systems will be executed as part of the advance frame update
        .with_rollback_schedule(
            Schedule::default().with_stage(
                "ROLLBACK_STAGE",
                SystemStage::single_threaded()
                    .with_system(move_players)
                    .with_system(handle_spell_casts.after(move_players))
                    .with_system(velocity_system.after(handle_spell_casts))
                    .with_system(update_dash_info.after(velocity_system))
                    // physics stuff - need to be at the end
                    .with_system(clear_correction_system.after(update_dash_info))
                    .with_system(update_movable_system.after(clear_correction_system))
                    .with_system(update_walls_system.after(update_movable_system))
                    .with_system(collision_system.after(update_walls_system))
                    .with_system(spell_collision_system.after(collision_system)),
            ),
        )
        // make it happen in the bevy app
        .build(&mut app);

    app.add_loopless_state(GameState::AssetLoading)
        .add_loading_state(
            LoadingState::new(GameState::AssetLoading)
                .continue_to_state(GameState::Menu)
                .with_collection::<Sprites>()
                .with_collection::<SpellSprites>()
                .with_collection::<MenuSprites>(),
        )
        .add_state(GameState::AssetLoading)
        //base plugins
        //.insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        title: "Mageling".to_string(),
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
        .register_ldtk_entity::<WallCollisions>("Wall")
        .insert_resource(LevelSelection::Index(0))
        .add_plugin(Material2dPlugin::<OutlineAndTextureMaterial>::default())
        .add_plugin(AsepritePlugin)
        // base systems
        .add_system_set(
            ConditionSet::new()
                // all the conditions, and any labels/ordering
                // must be added before adding the systems
                // (helps avoid confusion and accidents)
                // (makes it clear they apply to all systems in the set)
                .run_in_state(GameState::InRound)
                .label("thing2")
                //.after("stuff")
                .with_system(handle_spell_buffer)
                .with_system(update_spell_lifetimes)
                .into(),
        )
        .add_enter_system(GameState::Menu, setup)
        .add_enter_system(GameState::BetweenRound, spawn_players)
        .add_enter_system(GameState::WaitingForPlayers, setup_map)
        .add_system(update_animation_state)
        .init_resource::<PlayerSpellBuffer>();
    // resources
    app.insert_resource(LocalPlayer { handle_id: 0 });

    // crate plugins
    app.add_plugin(CamPlugin)
        .add_plugin(NetworkPlugin)
        .add_plugin(SpellPlugin)
        .add_plugin(UiPlugin);

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
    mut query: Query<&mut Transform, (With<Handle<LdtkLevel>>, Without<Wall>)>,
    mut wall_query: Query<&mut Transform, With<Wall>>,
    asset_server: Res<AssetServer>,
    settings: Res<RoomNetworkSettings>,
    mut spawn_points: Query<(&mut Transform, &SpawnPoint)>,
) {
    for mut transform in query.iter_mut() {
        info!("This is called");
        transform.translation.y -= 180.0;
    }
    for mut transform in wall_query.iter_mut() {
        transform.translation.y -= 180.0;
        transform.translation.x -= 320.0;
    }

    // collect and sort for determinism
    let mut info = spawn_points.iter_mut().collect::<Vec<_>>();
    info.sort_by_key(|x| x.1);
    
    let mut spawn_points = vec![];
    
    for (mut transform, spawn_point) in info {
        transform.translation.y -= 180.0;
        transform.translation.x -= 320.0;
        spawn_points.push(transform.translation);
    }

    for i in 0..settings.player_count {
        //let next_spawn_point = info[i as usize].0;
        commands.spawn(PlayerBundle {
            player_id: PlayerId { handle: i as usize },
            rollback_id: Rollback::new(rip.next_id()),
            player_spells: PlayerSpells {
                autoattack: SpellCastInfo {
                    spell_type: SpellType::SelfCast,
                    cooldown: 0.0,
                    spell_indicator: spell_sprites.circle_indicator.clone_weak(),
                    spell_id: Default::default(),
                },
                spells: vec![SpellCastInfo {
                    spell_type: SpellType::SelfCast,
                    cooldown: 0.0,
                    spell_indicator: spell_sprites.circle_indicator.clone_weak(),
                    spell_id: Default::default(),
                }],
            },
            combat_state: Default::default(),
            player_movement: PlayerMovementStats {
                speed: 160.0,
                dash_power: 3.0,
                dash_duration: 0.15,
                dash_cooldown_length: 5.0,
            },
            player_movement_state: PlayerMovementState {
                can_dash: true,
                dash_cooldown: 0.0,
                movement_state: MovementState::default(),
            },
            health: Health {
                max_health: 100,
                current_health: 100,
            },
            team_id: TeamId {
                id: (i % 2) as usize,
            },
            sepax: Sepax {
                convex: Convex::AABB(AABB::new((0.0, 0.0 + (i as f32 * 20.0)), 5.0, 16.0)),
            },
            movable: Movable { axes: vec![] },
            movement: Default::default(),
            aseprite_bundle: AsepriteBundle {
                transform: Transform {
                    translation: spawn_points[i as usize],
                    rotation: Default::default(),
                    scale: Vec3 {
                        x: 1.0,
                        y: 1.0,
                        z: 1.0,
                    },
                },
                animation: AsepriteAnimation::from("Idle"),
                aseprite: asset_server.load("magelings/Red-Mageling-Run.aseprite"),
                ..default()
            },
            animation_state: AnimationState::Idle,
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

    commands.spawn((
        Sepax {
            convex: Convex::AABB(AABB::new((0.0, 0.0 - (4 as f32 * 20.0)), 20.0, 20.0)),
        },
        SpriteBundle {
            transform: Transform {
                translation: Vec3 {
                    x: 0.0,
                    y: 0.0 - (4 as f32 * 20.0),
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
            sprite: Sprite {
                color: Color::RED,
                ..default()
            },
            ..default()
        },
    ));

    commands.insert_resource(NextState(GameState::InRound))
}

fn setup(mut commands: Commands) {
    commands.spawn(CameraBundle::default());
}

fn setup_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("ldtk/map.ldtk"),
        level_set: Default::default(),
        ..default()
    });
}

#[derive(
    FromReflect, Reflect, Default, Eq, PartialEq, Debug, PartialOrd, Ord, Copy, Clone, Resource,
)]
pub struct NetworkIdProvider {
    pub current_highest_id: i64,
}

impl NetworkIdProvider {
    pub fn next_id(&mut self) -> i64 {
        self.current_highest_id + 1
    }
}

#[derive(
    FromReflect, Reflect, Default, Eq, PartialEq, Debug, PartialOrd, Ord, Copy, Clone, Component,
)]
pub struct NetworkID(i64);

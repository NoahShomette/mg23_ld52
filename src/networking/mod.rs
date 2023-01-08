use crate::networking::ggrs::GGRSConfig;
use crate::{GameState, FPS};
use bevy::prelude::{info, App, Commands, Plugin, Res, ResMut, Resource};
use bevy::tasks::IoTaskPool;
use bevy_ggrs::ggrs::SessionBuilder;
use bevy_ggrs::{Session};
use iyes_loopless::prelude::NextState;
use matchbox_socket::WebRtcSocket;

pub mod ggrs;
pub mod rollback_systems;

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        // insert basic default matchmaking resource for testing
        //app.insert_resource(RoomNetworkSettings::default_matchmake_room());
        
        // insert basic default local matchmaking resource for testing
        //app.insert_resource(RoomNetworkSettings::testing_local());

        //testing online
        app.insert_resource(RoomNetworkSettings::testing_ip());
        
    }
}

#[derive(Resource)]
pub struct RoomNetworkSettings {
    // Network type
    pub network_type: MatchmakeType,
    pub ip: String,
    pub port: String,
    pub player_count: u32,
}

impl RoomNetworkSettings {
    pub fn default_matchmake_room() -> Self {
        RoomNetworkSettings {
            network_type: MatchmakeType::Matchmake,
            ip: "match.gschup.dev".to_string(),
            port: "".to_string(),
            player_count: 2,
        }
    }

    pub fn custom_matchmake_room(ip: String, port: String, player_count: u32) -> Self {
        RoomNetworkSettings {
            network_type: MatchmakeType::Matchmake,
            ip,
            port,
            player_count,
        }
    }

    pub fn private_room(room_key: String, player_count: u32) -> Self {
        RoomNetworkSettings {
            network_type: MatchmakeType::PrivateRoom(room_key),
            ip: "match.gschup.dev".to_string(),
            port: "".to_string(),
            player_count,
        }
    }
    
    fn testing_ip() -> Self {
        RoomNetworkSettings {
            network_type: MatchmakeType::Matchmake,
            ip: "172.124.208.194".to_string(),
            port: "6500".to_string(),
            player_count: 2,
        }
    }

    fn testing_local() -> Self {
        RoomNetworkSettings {
            network_type: MatchmakeType::Matchmake,
            ip: "127.0.0.1".to_string(),
            port: "6500".to_string(),
            player_count: 2,
        }
    }
    
}

/// Default is a local room
impl Default for RoomNetworkSettings {
    fn default() -> Self {
        RoomNetworkSettings {
            network_type: MatchmakeType::Matchmake,
            ip: "127.0.0.1".to_string(),
            port: "3536".to_string(),
            player_count: 2,
        }
    }
}

pub enum MatchmakeType {
    Matchmake,
    PrivateRoom(String),
}

/* not needed currently but keeping in case we need it
pub enum RoomNetworkType {
    Local,
    Online(MatchmakeType),
}

 */

#[derive(Resource)]
pub struct WrtcSocket {
    socket: Option<WebRtcSocket>,
}

pub fn start_matchbox_socket(mut commands: Commands, settings: Res<RoomNetworkSettings>) {
    // local ip
    let room_url: String = match &settings.network_type {
        MatchmakeType::Matchmake => {
            format!(
                "ws://{}:{}/mg23?next={}",
                settings.ip,
                settings.port,
                settings.player_count.to_string()
            )
        }
        MatchmakeType::PrivateRoom(room_key) => {
            format!(
                "ws://{}:{}/{}?next={}",
                settings.ip,
                settings.port,
                room_key,
                settings.player_count.to_string()
            )
        }
    };
    //let room_url = "ws://172.124.208.194:6500/network_test?next=2";
    info!("connecting to matchbox server: {:?}", room_url);

    let (socket, message_loop) = WebRtcSocket::new(room_url);

    // The message loop needs to be awaited, or nothing will happen.
    // We do this here using bevy's task system.
    IoTaskPool::get().spawn(message_loop).detach();

    commands.insert_resource(WrtcSocket {
        socket: Some(socket),
    }); 
}

pub fn wait_for_players(
    mut commands: Commands,
    mut socket: ResMut<WrtcSocket>,
    settings: Res<RoomNetworkSettings>,
) {
    // If there is no socket we've already started the game
    if socket.socket.is_none() {
        return;
    }
    let socket_ref = socket.socket.as_mut().unwrap();

    // Check for new connections
    socket_ref.accept_new_connections();
    let players = socket_ref.players();

    //info!("{}", socket_ref.players().len());

    if socket_ref.players().len() < settings.player_count as usize {
        return;
    }
    // create a new ggrs session
    let mut session_builder = SessionBuilder::<GGRSConfig>::new()
        .with_num_players(settings.player_count as usize)
        .with_max_prediction_window(8) // (optional) set max prediction window
        .with_input_delay(1) // (optional) set input delay for the local player
        .with_fps(FPS)
        .expect("Invalid FPS");

    for (i, player) in players.into_iter().enumerate() {
        session_builder = session_builder
            .add_player(player, i)
            .expect("failed to add player");
    }

    let socket = socket.socket.take().unwrap();

    // move the socket out of the resource (required because GGRS takes ownership of it)
    // start the GGRS session
    let session = session_builder
        .start_p2p_session(socket)
        .expect("failed to start session");

    commands.insert_resource(Session::P2PSession(session));
    commands.insert_resource(NextState(GameState::BetweenRound))
}

use bevy::prelude::{App, Camera, Commands, GlobalTransform, Plugin, Query, Res, ResMut, Resource, Vec2, Windows};
use bevy::render::camera::RenderTarget;
use bevy_tiled_camera::{TiledCameraBundle, WorldSpace};
use iyes_loopless::prelude::IntoConditionalSystem;
use crate::GameState;

/// A plugin containing the systems and resources for the Bevy_GGF camera system to function
pub struct CamPlugin;

impl Plugin for CamPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<CursorWorldPos>()
            .add_system(update_cursor_world_pos.run_in_state(GameState::BetweenRound))
            .add_system(update_cursor_world_pos.run_in_state(GameState::InRound));

    }
}

#[derive(PartialEq, Clone, Copy, Debug, Default, Resource)]
pub struct CursorWorldPos {
    pub cursor_world_pos: Vec2,
}


fn setup(
    mut commands: Commands,
) {
    commands.spawn(
        TiledCameraBundle::new()
            .with_pixels_per_tile([24, 24])
            .with_tile_count([26, 15])
            .with_world_space(WorldSpace::Pixels),
    );
}

fn update_cursor_world_pos(
    mut query: Query<(&GlobalTransform, &Camera)>,
    mut cursor_world_pos: ResMut<CursorWorldPos>,
    windows: Res<Windows>,
) {
    let (global_transform, camera) = query.single_mut();

    // get current window - used to get the mouse cursors position for click events and drag movement
    let wnd = if let RenderTarget::Window(id) = camera.target {
        windows.get(id).unwrap()
    } else {
        windows.get_primary().unwrap()
    };

    //if the cursor is inside the current window then we want to update the cursor position
    if let Some(current_cursor_position) = wnd.cursor_position() {
        let ray = camera
            .viewport_to_world(global_transform, current_cursor_position)
            .unwrap();
        cursor_world_pos.cursor_world_pos = ray.origin.truncate();
    }
}

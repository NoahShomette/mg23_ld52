use crate::networking::ggrs::GGRSConfig;
use crate::physics::Movement;
use crate::player::input::DASH;
use crate::player::{MovementState, PlayerId, PlayerMovementState, PlayerMovementStats};
use bevy::prelude::{info, Entity, Query, Res, Time, Transform};
use bevy_ggrs::PlayerInputs;
use bevy_sepax2d::prelude::Movable;

pub fn move_players(
    inputs: Res<PlayerInputs<GGRSConfig>>,
    mut players_query: Query<(
        &mut Movement,
        &PlayerId,
        &mut PlayerMovementState,
        &mut Transform,
    )>,
    time: Res<Time>,
) {
    // collect and sort for determinism
    let mut info = players_query.iter_mut().collect::<Vec<_>>();
    info.sort_by_key(|x| x.1);

    for (mut movement, player, mut player_movement, mut transform) in info {
        let (input, _) = inputs[player.handle];

        let move_delta = input.move_direction;
        
        if input.action_vars & DASH != 0 {
            player_movement.movement_state = MovementState::Dashing { duration: 0.0 };
            player_movement.can_dash = false;
        }
        
        if move_delta.normalize().is_nan(){
            //player_movement.movement_state = MovementState::Idle;
        }
        
        movement.velocity = move_delta.normalize_or_zero();

    }
}

pub fn velocity_system(
    mut query: Query<(
        Entity,
        &mut Movement,
        &PlayerMovementStats,
        &mut PlayerMovementState,
        &mut Transform,
    )>,
    time: Res<Time>,
) {
    // collect and sort for determinism
    let mut info = query.iter_mut().collect::<Vec<_>>();
    info.sort_by_key(|x| x.0);

    for (_, mut movement, stats, mut state, mut transform) in info {
        let mut movement_speed = stats.speed;
        
        match state.movement_state{
            MovementState::Dashing { .. } => {
                movement_speed = movement_speed * stats.dash_power;
                transform.translation.x += (movement.velocity.x * movement_speed) * time.delta_seconds();
                transform.translation.y += (movement.velocity.y * movement_speed) * time.delta_seconds();
            }
            MovementState::Walking => {
                transform.translation.x += (movement.velocity.x * movement_speed) * time.delta_seconds();
                transform.translation.y += (movement.velocity.y * movement_speed) * time.delta_seconds();
            }
            MovementState::Idle => {
                transform.translation.x += (movement.velocity.x * movement_speed) * time.delta_seconds();
                transform.translation.y += (movement.velocity.y * movement_speed) * time.delta_seconds();
                //movement.velocity.x *= movement.damping.powf(time.delta_seconds());
                //movement.velocity.y *= movement.damping.powf(time.delta_seconds());
            }
        }

    }
}

pub fn update_dash_info(
    mut query: Query<(
        Entity,
        &Movement,
        &mut PlayerMovementStats,
        &mut PlayerMovementState,
        &mut Transform,
    )>,
    time: Res<Time>,
) {
    // collect and sort for determinism
    let mut info = query.iter_mut().collect::<Vec<_>>();
    info.sort_by_key(|x| x.0);

    for (_, movement, mut stats, mut state, mut transform) in info {
        match state.movement_state {
            MovementState::Dashing { mut duration } => {
                duration += time.delta_seconds();
                if duration >= stats.dash_duration {
                    state.movement_state = MovementState::Walking;
                }else{
                    state.movement_state = MovementState::Dashing {duration};
                }
            }
            _ => {
                if stats.dash_cooldown_length < stats.dash_cooldown{
                    stats.dash_cooldown += time.delta_seconds();
                }else{
                    state.can_dash = true;
                }
            }
        }
    }
}

pub fn velocity_correction_system(mut query: Query<(Entity, &mut Movement, &Movable)>) {
    // collect and sort for determinism
    let mut info = query.iter_mut().collect::<Vec<_>>();
    info.sort_by_key(|x| x.0);

    for (_, mut movement, correction) in info {
        for (_x, y) in correction.axes.iter() {
            if y.abs() > f32::EPSILON
                && movement.velocity.y.is_sign_positive() != y.is_sign_positive()
            {
                movement.velocity.y = 0.0;
            }
        }
    }
}

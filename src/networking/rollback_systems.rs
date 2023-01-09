use crate::networking::ggrs::GGRSConfig;
use crate::physics::Movement;
use crate::player::input::{CAST_SPELL, DASH};
use crate::player::{MovementState, PlayerCombatState, PlayerId, PlayerMovementState, PlayerMovementStats, PlayerSpellBuffer, PlayerSpells, SpellAction, SpellCastState, TeamId};
use crate::spell::{DamageDealer, DamageSpellProjectileBundle, SpellAnimation, SpellCasterId, SpellId, SpellLifetime};
use bevy::log::info;
use bevy::prelude::{
    default, Commands, Entity, Query, Res, ResMut, Time, Transform, With, Without,
};
use bevy_aseprite::anim::AsepriteAnimation;
use bevy_aseprite::AsepriteBundle;
use bevy_ggrs::{PlayerInputs, Rollback, RollbackIdProvider};
use bevy_sepax2d::prelude::Sepax;
use bevy_sepax2d::Convex;
use sepax2d::prelude::Circle;
use sepax2d::sat_overlap;
use crate::assets::SpellSprites;
use crate::map::{SpawnPoint, Wall};

pub fn handle_spell_casts(
    inputs: Res<PlayerInputs<GGRSConfig>>,
    mut players_query: Query<(&PlayerId, &mut PlayerCombatState, &PlayerSpells)>,
    //spell_buffer: ResMut<PlayerSpellBuffer>,
    mut commands: Commands,
    game_spells: Res<SpellSprites>,
    mut rip: ResMut<RollbackIdProvider>,
) {
    // collect and sort for determinism
    let mut info = players_query.iter_mut().collect::<Vec<_>>();
    info.sort_by_key(|x| x.0);

    for (id, mut combat_state, mut spells) in info {
        let (input, _) = inputs[id.handle];

        if input.action_vars & CAST_SPELL != 0 {
            commands.spawn(DamageSpellProjectileBundle {
                sepax: Sepax {
                    convex: Convex::Circle(Circle {
                        position: (input.mouse_position.x as f32, input.mouse_position.y as f32),
                        radius: 65.0,
                    }),
                },
                damage: DamageDealer { damage_amount: 30 },
                spell_id: Default::default(),
                spell_caster_id: SpellCasterId {
                    id: PlayerId {
                        handle: id.handle,
                    },
                },
                spell_lifetime: SpellLifetime {
                    max_cast_delay: 1.0,
                    current_cast_delay: 0.0,
                    max_cast_frame: 0,
                    max_explosion_frame: 6,
                },
                aseprite_bundle: AsepriteBundle {
                    transform: Transform {
                        translation: input.mouse_position.extend(20.0),
                        ..default()
                    },
                    animation: AsepriteAnimation::from("Explosion"),
                    aseprite: game_spells.explosion_spell.clone(),
                    ..default()
                },
                animation_state: SpellAnimation::CastDelay,
                rollback_id: Rollback::new(rip.next_id()),
            });

            combat_state.spell_cast_state = SpellCastState::None;
        }
    }
}

pub fn update_spell_lifetimes(
    mut commands: Commands,
    mut spell_query: Query<(Entity, &mut SpellLifetime, &SpellId, &mut AsepriteAnimation, &SpellAnimation)>,
    time: Res<Time>,
) {
    // collect and sort for determinism
    let mut info = spell_query.iter_mut().collect::<Vec<_>>();
    info.sort_by_key(|x| x.2);

    for (entity, mut lifetime, _, mut animation, spell_animation) in info {
        lifetime.current_cast_delay += time.delta_seconds();
        if lifetime.current_cast_delay >= lifetime.max_cast_delay{
            if SpellAnimation::Cast != *spell_animation{
                *animation = AsepriteAnimation::from("Explosion");
            }
            if animation.current_frame() == lifetime.max_cast_frame {
                commands.entity(entity).despawn();
            }
        }
    }
}

pub fn spell_collision_system(
    mut commands: Commands,
    spells: Query<(Entity, &Sepax, &SpellCasterId), (With<SpellId>, Without<PlayerId>)>,
    mut players: Query<(Entity, &Sepax, &PlayerId, &TeamId, &mut Transform), (With<PlayerId>, Without<SpellId>)>,
    mut spawn_points: Query<(&mut Transform, &SpawnPoint), Without<PlayerId>>,

) {

    // collect and sort for determinism
    let mut info = spawn_points.iter_mut().collect::<Vec<_>>();
    info.sort_by_key(|x| x.1);

    let mut spawn_points = vec![];

    for (transform, spawn_point) in info {
        spawn_points.push((transform.translation, spawn_point.0));
    }
    
    for (spell, spell_sepax, spell_caster_id) in spells.iter() {
        for (enemy, enemy_sepax, player_id, team_id, mut transform) in players.iter_mut() {
            if spell_caster_id.id.handle != player_id.handle {
                info!("overlapped enemy");
                if sat_overlap(enemy_sepax.shape(), spell_sepax.shape()) {
                    for spawn_point in spawn_points.clone(){
                        if spawn_point.1.id == team_id.id{
                            transform.translation = spawn_point.0;
                        }
                    }
                }
            }
        }
    }
}

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
            player_movement.movement_state = MovementState::Dashing {
                duration: 0.0,
                direction: move_delta.normalize_or_zero(),
            };
            player_movement.can_dash = false;
        }

        if move_delta.normalize().is_nan() {
            if let MovementState::Dashing { .. } = player_movement.movement_state {
            } else {
                player_movement.movement_state = MovementState::Idle;
            }
        } else {
            if let MovementState::Dashing { .. } = player_movement.movement_state {
            } else {
                player_movement.movement_state = MovementState::Walking;
            }
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
        match state.movement_state {
            MovementState::Dashing {
                duration,
                direction,
            } => {
                movement_speed = movement_speed * stats.dash_power;
                transform.translation.x += (direction.x * movement_speed) * time.delta_seconds();
                transform.translation.y += (direction.y * movement_speed) * time.delta_seconds();
            }
            MovementState::Walking => {
                transform.translation.x +=
                    (movement.velocity.x * movement_speed) * time.delta_seconds();
                transform.translation.y +=
                    (movement.velocity.y * movement_speed) * time.delta_seconds();
            }
            MovementState::Idle => {
                transform.translation.x +=
                    (movement.velocity.x * movement_speed) * time.delta_seconds();
                transform.translation.y +=
                    (movement.velocity.y * movement_speed) * time.delta_seconds();
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
            MovementState::Dashing {
                mut duration,
                direction,
            } => {
                duration += time.delta_seconds();
                if duration >= stats.dash_duration {
                    state.movement_state = MovementState::Idle;
                } else {
                    state.movement_state = MovementState::Dashing {
                        duration,
                        direction,
                    };
                }
            }
            _ => {
                if stats.dash_cooldown_length < state.dash_cooldown {
                    state.dash_cooldown += time.delta_seconds();
                    state.can_dash = false;
                } else {
                    state.can_dash = true;
                    state.dash_cooldown = 0.0;
                }
            }
        }
    }
}

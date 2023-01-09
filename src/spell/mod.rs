use crate::camera::CursorWorldPos;
use crate::player::{AnimationState, LocalPlayer, PlayerCombatState, PlayerId, PlayerMovementState, PlayerSpells, SpellCastState};
use bevy::app::App;
use bevy::prelude::{
    Bundle, Commands, Component, Entity, FromReflect, Handle, Image, In, Local, Plugin, Query,
    Reflect, Res, Resource, Transform, Vec2,
};
use bevy::sprite::SpriteBundle;
use bevy::utils::{default, HashMap};
use bevy_aseprite::AsepriteBundle;
use bevy_ggrs::Rollback;
use bevy_sepax2d::components::Sepax;
use bevy_sepax2d::prelude::Movable;

pub struct SpellPlugin;

impl Plugin for SpellPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(show_spell_preview);
    }
}

pub fn show_spell_preview(
    mut commands: Commands,
    local_handle: Res<LocalPlayer>,
    mouse_pos: Res<CursorWorldPos>,
    mut query: Query<(&PlayerId, &mut PlayerCombatState, &PlayerSpells)>,
    mut sprite_query: Query<&mut Transform>,
    mut preview_sprite: Local<Option<Entity>>,
) {
    //let preview_option = &mut *preview_sprite;

    for (id, combat_state, spells) in query.iter_mut() {
        if id.handle == local_handle.handle_id {
            match &combat_state.spell_cast_state {
                SpellCastState::None => {
                    if let Some(entity) = *preview_sprite {
                        *preview_sprite = None;
                        commands.entity(entity).despawn();
                    }
                }
                SpellCastState::Precast { spell_id } => {
                    if let Some(entity) = *preview_sprite {
                        let mut transform = sprite_query
                            .get_mut(entity)
                            .expect("Valid because entity is in local");
                        transform.translation = mouse_pos.cursor_world_pos.extend(15.0);
                    } else {
                        let id = commands
                            .spawn(SpriteBundle {
                                transform: Transform {
                                    translation: mouse_pos.cursor_world_pos.extend(15.0),
                                    ..default()
                                },
                                global_transform: Default::default(),
                                texture: spell_id.spell_indicator.clone(),
                                ..default()
                            })
                            .id();

                        *preview_sprite = Some(id);
                    }
                }
                SpellCastState::Cast => {
                    if let Some(entity) = *preview_sprite {
                        *preview_sprite = None;
                        commands.entity(entity).despawn();
                    }
                }
            }
        }
    }
}

// spells have two parts that are not connected perse
// the cast section of a spell, this controls all the cast timings, etc. This isnt set per a spell technically but basically would always be unique per a spell
// this controls the spell cooldown, how long it takes to cast, etc
// the second part is the spell entity and all its components. These control the actual spell qualities, its shapes, how it spawns, etc

/// The resource that holds all the spells in the game
#[derive(FromReflect, Reflect, Default, PartialEq, Debug, Clone, Resource)]
pub struct GameSpells {
    pub spell_cast_info: HashMap<u32, SpellCastInfo>,
}

#[derive(Bundle)]
pub struct DamageSpellProjectileBundle {
    pub sepax: Sepax,
    pub damage: DamageDealer,
    pub spell_id: SpellId,
    pub spell_caster_id: SpellCasterId,
    pub spell_lifetime: SpellLifetime,
    pub aseprite_bundle: AsepriteBundle,
    pub animation_state: SpellAnimation,
    pub rollback_id: Rollback,
}

impl DamageSpellProjectileBundle {
    fn spawn_spell() {}
}

/// The cast type of the spell
#[derive(FromReflect, Reflect, PartialEq, Debug, Clone)]
pub enum SpellType {
    SelfCast,
    Directional(Vec2),
    Location(Vec2),
    Targeted(PlayerId),
}

/// A struct holding information on how to cast a spell
#[derive(FromReflect, Reflect, PartialEq, Debug, Clone)]
pub struct SpellCastInfo {
    pub spell_type: SpellType,
    pub cooldown: f32,
    pub spell_id: SpellId,
    pub spell_indicator: Handle<Image>,
}

/// A struct holding the id of the spell
#[derive(
    FromReflect, Reflect, Default, Eq, PartialEq, Debug, PartialOrd, Ord, Clone, Component,
)]
pub struct SpellId {
    pub id: u32,
}

/// A struct holding the id of the spell
#[derive(FromReflect, Reflect, Default, PartialEq, Debug, PartialOrd, Clone, Component)]
pub struct SpellLifetime {
    pub max_cast_delay: f32,
    pub current_cast_delay: f32,
    /// Not used rn but keep for future.
    pub max_cast_frame: usize,
    pub max_explosion_frame: usize,
}

/// The id of the player who cast the spell
#[derive(FromReflect, Reflect, Eq, PartialEq, Debug, PartialOrd, Ord, Clone, Component)]
pub struct SpellCasterId {
    pub id: PlayerId,
}

/// A struct signifying the spell deals damage
#[derive(FromReflect, Reflect, Eq, PartialEq, Debug, PartialOrd, Ord, Clone, Component)]
pub struct DamageDealer {
    pub damage_amount: u32,
}

#[derive(FromReflect, Reflect, Eq, PartialEq, Debug, PartialOrd, Ord, Clone, Component)]
pub enum SpellAnimation {
    Indicator,
    CastDelay,
    Cast,
    PostCast,
}

use crate::player::{PlayerId, TeamId};
use bevy::asset::{AssetServer, Assets, Handle};
use bevy::prelude::{Bundle, Component, IVec2, Image, TextureAtlas, TransformBundle, FromReflect, Reflect};
use bevy_ecs_ldtk::ldtk::{FieldInstance, LayerInstance, TilesetDefinition};
use bevy_ecs_ldtk::prelude::LdtkEntity;
use bevy_ecs_ldtk::{EntityInstance, IntGridCell};
use bevy_sepax2d::prelude::Sepax;
use bevy_sepax2d::Convex;
use sepax2d::prelude::AABB;

#[derive(Bundle)]
pub struct WallCollisions {
    sepax: Sepax,
    transform_bundle: TransformBundle,
    wall: Wall,
}

#[derive(Component)]
pub struct Wall;

impl LdtkEntity for WallCollisions {
    fn bundle_entity(
        entity_instance: &EntityInstance,
        layer_instance: &LayerInstance,
        tileset: Option<&Handle<Image>>,
        tileset_definition: Option<&TilesetDefinition>,
        asset_server: &AssetServer,
        texture_atlases: &mut Assets<TextureAtlas>,
    ) -> Self {
        WallCollisions {
            sepax: Sepax {
                convex: Convex::AABB(AABB::new((0.0, 0.0), 16.0, 16.0)),
            },
            transform_bundle: Default::default(),
            wall: Wall,
        }
    }
}

#[derive(Bundle)]
pub struct SpawnPointBundle {
    transform_bundle: TransformBundle,
    pub spawn_point: SpawnPoint,
}

#[derive(
    FromReflect, Reflect, Default, Eq, PartialEq, Debug, PartialOrd, Ord, Copy, Clone, Component,
)]
pub struct SpawnPoint(pub TeamId);

impl LdtkEntity for SpawnPointBundle {
    fn bundle_entity(
        entity_instance: &EntityInstance,
        layer_instance: &LayerInstance,
        tileset: Option<&Handle<Image>>,
        tileset_definition: Option<&TilesetDefinition>,
        asset_server: &AssetServer,
        texture_atlases: &mut Assets<TextureAtlas>,
    ) -> Self {
        let coords = entity_instance.grid;

        let team_id = match &entity_instance.field_instances[0] {
            FieldInstance {
                identifier,
                tile,
                field_instance_type,
                value,
                def_uid,
                real_editor_values,
            } => {
                if field_instance_type == "Team0" {
                    TeamId { id: 0 }
                } else {
                    TeamId { id: 1 }
                }
            }
        };
        SpawnPointBundle {
            transform_bundle: Default::default(),
            spawn_point: SpawnPoint(team_id),
        }
    }
}

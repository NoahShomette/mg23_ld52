use bevy::asset::{AssetServer, Assets, Handle};
use bevy::prelude::{Bundle, Component, Image, TextureAtlas, TransformBundle};
use bevy_ecs_ldtk::ldtk::{LayerInstance, TilesetDefinition};
use bevy_ecs_ldtk::{EntityInstance, IntGridCell};
use bevy_ecs_ldtk::prelude::LdtkEntity;
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
                convex: Convex::AABB(AABB::new(
                    (0.0, 0.0),
                    16.0,
                    16.0,
                )),
            },
            transform_bundle: Default::default(),
            wall: Wall,
        }
    }
}

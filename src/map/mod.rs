use bevy::prelude::Bundle;
use bevy_ecs_ldtk::{IntGridCell, LdtkEntity};
use bevy_ecs_ldtk::ldtk::LayerInstance;
use bevy_ecs_ldtk::prelude::LdtkIntCell;
use bevy_sepax2d::prelude::Sepax;
use bevy_sepax2d::Convex;
use sepax2d::prelude::AABB;

#[derive(Bundle)]
pub struct WallCollisions {
    sepax: Sepax,
}

impl LdtkIntCell for WallCollisions{
    fn bundle_int_cell(int_grid_cell: IntGridCell, layer_instance: &LayerInstance) -> Self {
        WallCollisions {
            sepax: Sepax {
                convex: Convex::AABB(AABB::new((0.0, 0.0), 16.0, 16.0)),
            },
        }    }
}

impl Default for WallCollisions {
    fn default() -> Self {
        WallCollisions {
            sepax: Sepax {
                convex: Convex::AABB(AABB::new((0.0, 0.0), 16.0, 16.0)),
            },
        }
    }
}


use bevy::math::Vec2;
use bevy::prelude::{App, Component, CoreStage, Entity, IntoSystemDescriptor, Plugin, Query, Reflect, Transform, Without};
use bevy_sepax2d::plugin::SepaxSystems;
use bevy_sepax2d::prelude::{Movable, NoCollision, Sepax};
use sepax2d::sat_collision;
use crate::map::Wall;

#[derive(Reflect, Component, Debug, Copy, Clone, PartialEq)]
pub struct Movement {
    pub velocity: Vec2,
    pub damping: f32,
    pub speed: f32,
}

impl Default for Movement {
    fn default() -> Self {
        Movement {
            velocity: Default::default(),
            damping: 0.9,
            speed: 50.0,
        }
    }
}

pub struct SepaxCustomPlugin;

impl Plugin for SepaxCustomPlugin
{

    fn build(&self, app: &mut App)
    {

        app
            .add_system_to_stage
            (

                CoreStage::PostUpdate,
                clear_correction_system
                    .label(SepaxSystems::Clear)

            )
            .add_system_to_stage
            (

                CoreStage::PostUpdate,
                update_movable_system.after(clear_correction_system)
                    .label(SepaxSystems::Update)

            )
            .add_system_to_stage
            (

                CoreStage::PostUpdate,
                collision_system
                    .label(SepaxSystems::Collision)
                    .after(SepaxSystems::Update)
                    .before(bevy::transform::transform_propagate_system)

            );

        #[cfg(feature = "debug")]
        app.add_plugin(ShapePlugin);

    }

}

/// [`Movable`](crate::components::Movable) components store a list of axes
/// that were used for collision resolution on the previous frame. This system
/// resets that list each frame before the collision system generates new data.
pub fn clear_correction_system(mut query: Query<(Entity, &mut Movable)>)
{

    // collect and sort for determinism
    let mut info = query.iter_mut().collect::<Vec<_>>();
    info.sort_by_key(|x| x.0);
    
    for (_, mut correction) in info
    {

        correction.axes.clear();

    }

}

/// Updates the position information contained inside of [`Sepax`](crate::components::Sepax)
/// components to match the entity's translation in the world. This is necessary because
/// sepax2d is not a Bevy-centric crate, so it does not use Transforms natively.
pub fn update_movable_system(mut query: Query<(Entity, &Transform, &Movable, &mut Sepax)>)
{
    // collect and sort for determinism
    let mut info = query.iter_mut().collect::<Vec<_>>();
    info.sort_by_key(|x| x.0);
    
    for (_, transform, _movable, mut sepax) in info
    {

        let position = (transform.translation.x, transform.translation.y);

        let shape = sepax.shape_mut();
        shape.set_position(position);

    }

}

/// Updates the position information contained inside of [`Sepax`](crate::components::Sepax)
/// components to match the entity's translation in the world. This is necessary because
/// sepax2d is not a Bevy-centric crate, so it does not use Transforms natively.
pub fn update_walls_system(mut query: Query<(Entity, &Transform, &Wall, &mut Sepax)>)
{
    // collect and sort for determinism
    let mut info = query.iter_mut().collect::<Vec<_>>();
    info.sort_by_key(|x| x.0);

    for (_, transform, _wall, mut sepax) in info
    {

        let position = (transform.translation.x, transform.translation.y);

        let shape = sepax.shape_mut();
        shape.set_position(position);

    }

}

/// Performs inelastic collisions between all [`Movable`](crate::components::Movable) and all immovable
/// entities. If there is a collision, the normalized axis of resolution is stored inside the `Movable`
/// component for use in your app. This points away from the immovable object. For example, if you are 
/// making a platformer and want to check if the player has landed on something, you would check for
/// axes with a positive y component. 
pub fn collision_system(mut movable: Query<(Entity, &mut Movable, &mut Sepax, &mut Transform), Without<NoCollision>>, walls: Query<&Sepax, (Without<Movable>, Without<NoCollision>)>)
{
    // collect and sort for determinism
    let mut info = movable.iter_mut().collect::<Vec<_>>();
    info.sort_by_key(|x| x.0);
    
    for (_, mut correct, mut sepax, mut transform) in info
    {

        for wall in walls.iter()
        {

            let shape = sepax.shape_mut();
            let correction = sat_collision(wall.shape(), shape);

            let old_position = shape.position();
            let new_position = (old_position.0 + correction.0, old_position.1 + correction.1);

            shape.set_position(new_position);
            transform.translation.x = new_position.0;
            transform.translation.y = new_position.1;

            let length = f32::sqrt((correction.0 * correction.0) + (correction.1 * correction.1));

            if length > f32::EPSILON
            {

                correct.axes.push((correction.0 / length, correction.1 / length));

            }

        }

    }

}

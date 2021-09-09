use super::{Camera, DepthCalculation};
use crate::{draw::OutsideFrustum, prelude::Visible};
use bevy_core::FloatOrd;
use bevy_ecs::{entity::Entity, query::Without, system::Query};
use bevy_transform_spherical::prelude::GlobalTransform;
use bevy_utils::tracing::warn;

pub use bevy_render::camera::{Layer, RenderLayers, VisibleEntities, VisibleEntity};

pub fn visible_entities_system(
    mut camera_query: Query<(
        &Camera,
        &GlobalTransform,
        &mut VisibleEntities,
        Option<&RenderLayers>,
    )>,
    visible_query: Query<(Entity, &Visible, Option<&RenderLayers>), Without<OutsideFrustum>>,
    visible_transform_query: Query<&GlobalTransform, Without<OutsideFrustum>>,
) {
    for (camera, camera_global_transform, mut visible_entities, maybe_camera_mask) in
        camera_query.iter_mut()
    {
        visible_entities.value.clear();
        let camera_position = camera_global_transform.position();
        let camera_mask = maybe_camera_mask.copied().unwrap_or_default();

        let mut no_transform_order = 0.0;
        let mut transparent_entities = Vec::new();
        for (entity, visible, maybe_entity_mask) in visible_query.iter() {
            if !visible.is_visible {
                continue;
            }

            let entity_mask = maybe_entity_mask.copied().unwrap_or_default();
            if !camera_mask.intersects(&entity_mask) {
                continue;
            }

            let order = if let Ok(global_transform) = visible_transform_query.get(entity) {
                let position = global_transform.position();
                // smaller distances are sorted to lower indices by using the distance from the
                // camera
                FloatOrd(match camera.depth_calculation {
                    DepthCalculation::Distance => (camera_position - position).length_squared(),
                    DepthCalculation::ZDifference => {
                        warn!("The ZDifference depth calculation does not make sense in spherical space!");
                        (camera_position - position).length_squared()
                    }
                })
            } else {
                let order = FloatOrd(no_transform_order);
                no_transform_order += 0.1;
                order
            };

            if visible.is_transparent {
                transparent_entities.push(VisibleEntity { entity, order })
            } else {
                visible_entities.value.push(VisibleEntity { entity, order })
            }
        }

        // sort opaque entities front-to-back
        visible_entities.value.sort_by_key(|e| e.order);

        // sort transparent entities front-to-back
        transparent_entities.sort_by_key(|e| -e.order);
        visible_entities.value.extend(transparent_entities);

        // TODO: check for big changes in visible entities len() vs capacity() (ex: 2x) and resize
        // to prevent holding unneeded memory
    }
}

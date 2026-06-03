use bevy_camera::{
    primitives::Frustum,
    visibility::{self, Visibility, VisibilityClass, VisibleMeshEntities},
};
use bevy_ecs::prelude::*;
use bevy_math::Vec3;
use bevy_reflect::prelude::*;
use bevy_transform::components::{GlobalTransform, Transform};

use crate::cluster::{ClusterVisibilityClass, GlobalVisibleClusterableObjects};
use crate::{spot_light_clip_from_view, spot_light_world_from_view, SpotLight};

/// A bar-shaped light source based on [`SpotLight`].
///
/// This light behaves like a spot light with additional geometric information in
/// [`BarLight::length`] that defines the orientation and extent of an imagined
/// rod light source.
#[derive(Component, Debug, Clone, Copy, Reflect)]
#[reflect(Component, Default, Debug, Clone)]
#[require(Frustum, VisibleMeshEntities, Transform, Visibility, VisibilityClass)]
#[component(on_add = visibility::add_visibility_class::<ClusterVisibilityClass>)]
pub struct BarLight {
    /// Base spot-light parameters used for range, intensity, cone angles and shadows.
    pub spot_light: SpotLight,
    /// Vector describing the bar's orientation and length in the light's local space.
    pub length: Vec3,
}

impl Default for BarLight {
    fn default() -> Self {
        Self {
            spot_light: SpotLight::default(),
            length: Vec3::ZERO,
        }
    }
}

pub fn update_bar_light_frusta(
    global_lights: Res<GlobalVisibleClusterableObjects>,
    mut views: Query<
        (Entity, &GlobalTransform, &BarLight, &mut Frustum),
        Or<(Changed<GlobalTransform>, Changed<BarLight>)>,
    >,
) {
    for (entity, transform, bar_light, mut frustum) in &mut views {
        if !bar_light.spot_light.shadows_enabled || !global_lights.entities.contains(&entity) {
            continue;
        }

        let view_backward = transform.back();
        let spot_world_from_view = spot_light_world_from_view(transform);
        let spot_clip_from_view = spot_light_clip_from_view(
            bar_light.spot_light.outer_angle,
            bar_light.spot_light.shadow_map_near_z,
        );
        let clip_from_world = spot_clip_from_view * spot_world_from_view.inverse();

        *frustum = Frustum::from_clip_from_world_custom_far(
            &clip_from_world,
            &transform.translation(),
            &view_backward,
            bar_light.spot_light.range,
        );
    }
}

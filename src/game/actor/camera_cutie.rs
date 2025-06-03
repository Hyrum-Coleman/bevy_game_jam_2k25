use crate::core::camera::SmoothFollow;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<CameraCutie>();
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct CameraCutie;

impl Configure for CameraCutie {
    fn configure(app: &mut App) {
        app.register_type::<Self>();

        app.add_systems(
            Update,
            camera_follow
                .in_set(UpdateSystems::SyncEarly)
                .run_if(Pause::is_disabled),
        );
    }
}

fn camera_follow(
    mut camera_query: Query<&mut SmoothFollow>,
    camera_target_query: Query<Entity, With<CameraCutie>>,
) {
    if let Ok(camera_target) = camera_target_query.single() {
        if let Ok(mut follow) = camera_query.single_mut() {
            follow.target = camera_target;
        }
    }
}

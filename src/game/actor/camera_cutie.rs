use crate::core::camera::SmoothFollow;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<CameraCutieEvent>();
}

#[derive(Event, Reflect)]
pub struct CameraCutieEvent(pub Entity);

impl Configure for CameraCutieEvent {
    fn configure(app: &mut App) {
        app.add_event::<CameraCutieEvent>();
        app.add_systems(
            Update,
            camera_follow
                .in_set(UpdateSystems::SyncEarly)
                .run_if(Pause::is_disabled),
        );
    }
}

pub fn send_camera_follow_event(entity: Entity, mut event: EventWriter<CameraCutieEvent>) {
    event.write(CameraCutieEvent(entity));
}

fn camera_follow(
    mut camera_query: Query<&mut SmoothFollow>,
    mut ev_set: EventReader<CameraCutieEvent>,
) {
    let mut smooth_follow = r!(camera_query.single_mut());

    for ev in ev_set.read() {
        smooth_follow.target = ev.0;
    }
}

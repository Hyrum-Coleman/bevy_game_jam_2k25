use crate::game::actor::movement::MovementController;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<MovementAction>();
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Reflect, Debug)]
pub(crate) enum MovementAction {
    Move,
}

impl Actionlike for MovementAction {
    fn input_control_kind(&self) -> InputControlKind {
        match self {
            Self::Move => InputControlKind::DualAxis,
        }
    }
}

impl Configure for MovementAction {
    fn configure(app: &mut App) {
        app.init_resource::<ActionState<Self>>();
        app.add_plugins(InputManagerPlugin::<Self>::default());
        app.add_systems(
            Update,
            record_movement_action
                .in_set(UpdateSystems::RecordInput)
                .run_if(Pause::is_disabled),
        );
    }
}

fn record_movement_action(
    mut action_query: Query<(&ActionState<MovementAction>, &mut MovementController)>,
) {
    for (action, mut controller) in &mut action_query {
        controller.0 = action
            .axis_pair(&MovementAction::Move)
            .xy()
            .clamp_length_max(1.0);
        info!("Set controller.0 to: {}", controller.0);
    }
}

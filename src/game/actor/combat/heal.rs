use crate::game::actor::combat::health::Health;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<Heal>();
}

#[derive(Event, Reflect, Debug)]
pub struct Heal(pub f32);

impl Configure for Heal {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_observer(apply_heal);
    }
}

fn apply_heal(trigger: Trigger<Heal>, mut health_query: Query<&mut Health>) {
    let target = r!(trigger.get_target());
    let mut health = r!(health_query.get_mut(target));
    health.current += trigger.0;
}
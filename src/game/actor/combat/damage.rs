use crate::game::actor::combat::health::Health;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(Damage, OnDamage)>();
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct Damage(pub f32);

impl Configure for Damage {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
    }
}

#[derive(Event, Reflect, Debug)]
pub struct OnDamage(pub f32);

impl Configure for OnDamage {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_observer(on_damage);
        app.add_observer(deal_damage_on_collision);
    }
}

fn on_damage(trigger: Trigger<OnDamage>, mut health_query: Query<&mut Health>) {
    let target = r!(trigger.get_target());
    let mut target_health = r!(health_query.get_mut(target));
    target_health.current -= trigger.0;
    info!("Dealt {} damage", trigger.0);
}

fn deal_damage_on_collision(
    trigger: Trigger<OnCollisionStart>,
    mut commands: Commands,
    damage_query: Query<&Damage>,
    health_query: Query<(), With<Health>>,
) {
    let attacker = r!(trigger.get_target());
    let damage = r!(damage_query.get(attacker));

    let hit_entity = trigger.collider;
    rq!(health_query.contains(hit_entity));
    commands.entity(hit_entity).trigger(OnDamage(damage.0));
}

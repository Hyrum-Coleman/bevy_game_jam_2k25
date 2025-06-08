use crate::game::actor::combat::health::Health;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(Damage, OnDamage)>();
}

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct Damage(pub f32);

impl Configure for Damage {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
    }
}

#[derive(Event, Reflect, Debug)]
pub struct OnDamage {
    pub damage: f32,
    pub attacker: Option<Entity>,
}

impl OnDamage {
    pub fn new(damage: f32, attacker: Option<Entity>) -> Self {
        Self { damage, attacker }
    }
}

impl Configure for OnDamage {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_observer(decrease_health_on_damage);
        app.add_observer(deal_damage_on_collision);
    }
}

fn decrease_health_on_damage(trigger: Trigger<OnDamage>, mut health_query: Query<&mut Health>) {
    let target = r!(trigger.get_target());
    let mut target_health = r!(health_query.get_mut(target));
    target_health.current -= trigger.damage;
}

fn deal_damage_on_collision(
    trigger: Trigger<OnCollisionStart>,
    mut commands: Commands,
    damage_query: Query<&Damage>,
    health_query: Query<(), With<Health>>,
) {
    let attacker = r!(trigger.get_target());
    let damage = rq!(damage_query.get(attacker));

    let hit_entity = trigger.collider;
    rq!(health_query.contains(hit_entity));
    commands
        .entity(hit_entity)
        .trigger(OnDamage::new(damage.0, Some(attacker)));
}

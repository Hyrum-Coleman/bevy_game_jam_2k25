use crate::game::actor::combat::damage::OnDamage;
use crate::game::actor::combat::health::Health;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(LifeSteal, OnLifeSteal)>();
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct LifeSteal {
    pub proc_percent: f64,
    pub steal_percent: f32,
}

impl Configure for LifeSteal {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
    }
}

#[derive(Event, Reflect, Debug)]
pub struct OnLifeSteal {
    damage: f32,
}

impl OnLifeSteal {
    pub fn new(damage: f32) -> Self {
        Self { damage }
    }
}

impl Configure for OnLifeSteal {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_observer(increase_health_on_life_steal);
        app.add_observer(apply_lifesteal_on_damage);
    }
}

fn increase_health_on_life_steal(
    trigger: Trigger<OnLifeSteal>,
    mut health_query: Query<(&mut Health, &LifeSteal)>,
) {
    let target = r!(trigger.get_target());
    let (mut target_health, life_steal) = r!(health_query.get_mut(target));

    let effect_proc = thread_rng().gen_bool(life_steal.proc_percent);

    if effect_proc {
        target_health.current += trigger.damage * life_steal.steal_percent;
    }
}

pub fn apply_lifesteal_on_damage(
    trigger: Trigger<OnDamage>,
    mut commands: Commands,
    mut parent_query: Query<&ChildOf>,
    mut life_steal_query: Query<(), With<LifeSteal>>,
    mut name_debug_query: Query<&Name>,
) {
    let target = trigger.attacker;
    let damage = trigger.damage;
    let name = r!(name_debug_query.get(target));

    let parent = r!(parent_query.get(target)).parent();

    info!("Trigger name: {}", name);

    rq!(life_steal_query.contains(parent));
    commands.entity(parent).trigger(OnLifeSteal::new(damage));
}

use crate::game::actor::combat::damage::OnDamage;
use crate::game::actor::combat::heal::OnHeal;
use crate::game::actor::combat::health::Health;
use crate::prelude::*;

pub(in crate::game) fn plugin(app: &mut App) {
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
pub struct OnLifeSteal;

impl Configure for OnLifeSteal {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_observer(apply_lifesteal_on_damage);
    }
}


pub fn apply_lifesteal_on_damage(
    mut trigger: Trigger<OnDamage>,
    mut commands: Commands,
    mut parent_query: Query<&ChildOf>,
    mut life_steal_query: Query<(&LifeSteal)>,
) {
    let target = trigger.attacker;
    let damage = trigger.damage;

    let parent = r!(parent_query.get(target)).parent();

    let life_steal = rq!(life_steal_query.get(parent));

    rq!(thread_rng().gen_bool(life_steal.proc_percent));

    commands
        .entity(parent)
        .trigger(OnHeal(damage * life_steal.steal_percent));
}

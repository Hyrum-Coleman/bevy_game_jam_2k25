use crate::game::actor::combat::health::Health;
use crate::game::item::effects::damage_over_time::OnDamageOverTime;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(AppliesPoison, StartPoison)>();
}

const POISON_DAMAGE: f32 = 2.0;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct AppliesPoison {
    proc_chance: f64,
    duration: f32,
}

impl Configure for AppliesPoison {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
    }
}

impl AppliesPoison {
    pub fn new(proc_chance: f64) -> Self {
        Self {
            proc_chance,
            duration: 2.0,
        }
    }
}

#[derive(Event, Reflect, Debug)]
pub struct StartPoison;

impl Configure for StartPoison {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_observer(start_poison_on_collision);
    }
}

fn start_poison_on_collision(
    trigger: Trigger<OnCollisionStart>,
    mut commands: Commands,
    poison_query: Query<&AppliesPoison>,
    health_query: Query<(), With<Health>>,
    mut sprite_query: Query<&mut Sprite>,
) {
    let attacker = r!(trigger.get_target());
    let poison = rq!(poison_query.get(attacker));

    rq!(thread_rng().gen_bool(poison.proc_chance));

    let hit_entity = trigger.collider;
    rq!(health_query.contains(hit_entity));

    let mut sprite = r!(sprite_query.get_mut(hit_entity));

    sprite.color = sprite.color.mix(&Color::srgba(0.19, 0.77, 0.6, 1.0), 0.65);

    commands.entity(hit_entity).trigger(OnDamageOverTime {
        damage: POISON_DAMAGE,
        duration: poison.duration,
        interval: 0.5,
    });
}

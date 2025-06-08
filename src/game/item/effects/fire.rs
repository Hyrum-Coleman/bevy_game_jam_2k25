use crate::game::actor::combat::health::Health;
use crate::game::item::effects::damage_over_time::OnDamageOverTime;
use crate::prelude::*;

pub(in crate::game) fn plugin(app: &mut App) {
    app.configure::<(AppliesFire, StartFire)>();
}

const FIRE_DAMAGE: f32 = 5.0;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub(crate) struct AppliesFire {
    pub duration: Duration,
    pub proc_chance: f64,
}

impl Default for AppliesFire {
    fn default() -> Self {
        Self {
            duration: Duration::from_secs(2),
            proc_chance: 0.0,
        }
    }
}

impl AppliesFire {
    pub fn new(proc_chance: f64) -> Self {
        Self {
            duration: Duration::from_secs(2),
            proc_chance,
        }
    }
}

impl Configure for AppliesFire {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
    }
}

#[derive(Event, Reflect, Debug)]
pub struct StartFire;

impl Configure for StartFire {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_observer(start_fire_on_collision);
    }
}

fn start_fire_on_collision(
    trigger: Trigger<OnCollisionStart>,
    mut commands: Commands,
    fire_query: Query<&AppliesFire>,
    health_query: Query<(), With<Health>>,
    mut sprite_query: Query<&mut Sprite>,
) {
    let attacker = r!(trigger.get_target());
    let fire = r!(fire_query.get(attacker));

    rq!(thread_rng().gen_bool(fire.proc_chance));

    let hit_entity = trigger.collider;
    rq!(health_query.contains(hit_entity));

    let mut sprite = r!(sprite_query.get_mut(hit_entity));

    sprite.color = sprite.color.mix(&Color::srgba(1.0, 0.65, 0.0, 1.0), 0.65);

    commands.entity(hit_entity).trigger(OnDamageOverTime {
        damage: FIRE_DAMAGE,
        duration: 2.0,
        interval: 0.5,
    });
}

use crate::game::actor::combat::damage::OnDamage;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(DealsDamageOverTime, OnDamageOverTime, ActiveDOT)>();
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct DealsDamageOverTime {
    pub proc_chance: f64,
    pub damage: f32,
    pub duration: f32,
    pub interval: f32,
}

impl DealsDamageOverTime {
    pub fn new(proc_chance: f64, damage: f32, duration: f32, interval: f32) -> Self {
        Self {
            proc_chance,
            damage,
            duration,
            interval,
        }
    }
}

impl Configure for DealsDamageOverTime {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
    }
}

#[derive(Event, Reflect, Default)]
pub struct OnDamageOverTime {
    pub damage: f32,
    pub duration: f32,
    pub interval: f32,
}

impl Configure for OnDamageOverTime {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_observer(spawn_dot_effect);
        app.add_observer(apply_dot_on_damage);
    }
}

pub fn apply_dot_on_damage(
    trigger: Trigger<OnDamage>,
    mut commands: Commands,
    dot_query: Query<&DealsDamageOverTime>,
) {
    let target = trigger.target();
    let attacker = rq!(trigger.attacker);
    let dot = rq!(dot_query.get(attacker));

    rq!(thread_rng().gen_bool(dot.proc_chance));

    commands.entity(target).trigger(OnDamageOverTime {
        damage: dot.damage,
        duration: dot.duration,
        interval: dot.interval,
    });
}

pub fn spawn_dot_effect(trigger: Trigger<OnDamageOverTime>, mut commands: Commands) {
    let target = trigger.target();

    commands.entity(target).insert(ActiveDOT {
        damage: trigger.damage,
        remaining: trigger.duration,
        interval: trigger.interval,
        timer: Timer::from_seconds(trigger.interval, TimerMode::Repeating),
    });
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct ActiveDOT {
    pub damage: f32,
    pub remaining: f32,
    pub interval: f32,
    pub timer: Timer,
}

impl Configure for ActiveDOT {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(
            Update,
            dot_tick_system
                .in_set(UpdateSystems::Update)
                .run_if(Pause::is_disabled),
        );
    }
}

pub fn dot_tick_system(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut ActiveDOT)>,
) {
    for (entity, mut effect) in query.iter_mut() {
        effect.timer.tick(time.delta());

        effect.remaining -= time.delta_secs();

        if !effect.timer.just_finished() {
            continue;
        }

        if effect.remaining <= f32::EPSILON {
            commands.entity(entity).remove::<ActiveDOT>();
            continue;
        }

        commands
            .entity(entity)
            .trigger(OnDamage::new(effect.damage, None));
    }
}

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<Health>();
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct Health {
    pub max: f32,
    pub current: f32,
}

impl Configure for Health {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(
            Update,
            (
                handle_death.in_set(UpdateSystems::Update),
                clamp_health.in_set(UpdateSystems::SyncLate),
            ),
        );
    }
}

impl Health {
    pub fn new(max: f32) -> Self {
        Self { max, current: max }
    }
}

fn handle_death(mut commands: Commands, health_query: Query<(Entity, &Health), Changed<Health>>) {
    for (entity, health) in &health_query {
        if health.current >= f32::EPSILON {
            continue;
        }
        commands.entity(entity).despawn();
    }
}

fn clamp_health(mut health_query: Query<&mut Health, Changed<Health>>) {
    health_query.iter_mut().for_each(|mut health| {
        health.current = health.current.clamp(0.0, health.max);
    });
}

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
        app.add_systems(Update, handle_death.in_set(UpdateSystems::Update));
    }
}

impl Health {
    pub fn new(max: f32) -> Self {
        Self { max, current: max }
    }
}

fn handle_death(mut commands: Commands, health_query: Query<(Entity, &Health), Changed<Health>>) {
    for (entity, health) in health_query {
        rq!(health.current <= f32::EPSILON);
        commands.entity(entity).despawn();
    }
}

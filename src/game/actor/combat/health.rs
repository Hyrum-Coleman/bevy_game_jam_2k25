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
    }
}
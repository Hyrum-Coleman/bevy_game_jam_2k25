use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<Attack>();
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct Attack(pub f32);

impl Configure for Attack {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
    }
}
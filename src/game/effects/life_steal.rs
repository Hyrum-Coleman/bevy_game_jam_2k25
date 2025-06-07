use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(LifeSteal, OnLifeSteal)>();
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct LifeSteal(pub f32);

impl Configure for LifeSteal {
    fn configure(app: &mut App) {
        todo!()
    }
}

#[derive(Event, Reflect, Debug)]
pub struct OnLifeSteal;

impl Configure for OnLifeSteal {
    fn configure(app: &mut App) {
        todo!()
    }
}
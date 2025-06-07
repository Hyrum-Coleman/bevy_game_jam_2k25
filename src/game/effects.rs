mod LifeSteal;

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<Item>();
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct Item;

impl Configure for Item {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
    }
}
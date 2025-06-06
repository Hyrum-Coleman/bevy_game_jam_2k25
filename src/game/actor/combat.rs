pub mod health;
pub mod damage;

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((health::plugin, damage::plugin));
}


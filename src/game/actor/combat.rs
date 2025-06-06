pub mod health;
pub mod attack;

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((health::plugin, attack::plugin));
}


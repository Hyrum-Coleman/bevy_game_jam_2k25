pub mod damage;
pub mod health;

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((health::plugin, damage::plugin));
}

pub mod damage;
pub mod heal;
pub mod health;

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((health::plugin, damage::plugin, heal::plugin));
}

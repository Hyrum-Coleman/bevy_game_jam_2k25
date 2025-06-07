pub mod damage;
pub mod health;
pub mod heal;

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((health::plugin, damage::plugin));
}

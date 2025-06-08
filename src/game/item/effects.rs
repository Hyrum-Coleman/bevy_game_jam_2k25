pub mod damage_over_time;
pub mod fire;
pub mod life_steal;

use crate::prelude::*;

pub(in crate::game) fn plugin(app: &mut App) {
    app.add_plugins((life_steal::plugin, fire::plugin, damage_over_time::plugin));
}

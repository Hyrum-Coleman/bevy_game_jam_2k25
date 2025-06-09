pub mod damage_over_time;
pub mod fire;
pub mod life_steal;
pub mod poison;

use crate::prelude::*;

pub(in crate::game) fn plugin(app: &mut App) {
    app.add_plugins((
        life_steal::plugin,
        damage_over_time::plugin,
        fire::plugin,
        poison::plugin,
    ));
}

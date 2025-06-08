pub mod fire;
pub mod life_steal;

use crate::prelude::*;

pub(in crate::game) fn plugin(app: &mut App) {
    app.add_plugins((life_steal::plugin, fire::plugin));
}

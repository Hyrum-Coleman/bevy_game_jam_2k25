pub mod life_steal;

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(life_steal::plugin);
}

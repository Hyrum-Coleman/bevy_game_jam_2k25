use crate::prelude::*;

pub mod effects;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(effects::plugin);
}


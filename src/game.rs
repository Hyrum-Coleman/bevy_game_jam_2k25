use crate::prelude::*;

pub mod actor;
mod effects;
pub mod world;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((actor::plugin, world::plugin, effects::plugin));
}

#[derive(PhysicsLayer, Default)]
pub enum GameLayer {
    #[default]
    Default,
    Player,
    Enemy,
    _Projectile,
    Wall,
}

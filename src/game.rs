use item::effects;
use crate::prelude::*;

pub mod actor;
pub mod world;
pub mod item;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((actor::plugin, world::plugin, item::plugin));
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

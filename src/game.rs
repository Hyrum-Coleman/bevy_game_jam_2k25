use crate::prelude::*;

pub mod actor;
pub mod world;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((actor::plugin, world::plugin));
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

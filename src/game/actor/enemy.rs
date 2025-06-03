use crate::game::GameLayer;
use crate::game::actor::create_entity_sprite;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<Enemy>();
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Enemy;

impl Configure for Enemy {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
    }
}

pub fn get_enemy(name: &'static str, texture: Handle<Image>) -> impl Bundle {
    (
        Name::new(name),
        Enemy,
        create_entity_sprite(texture),
        Collider::rectangle(32., 32.),
        CollisionLayers::new(GameLayer::Enemy, LayerMask::ALL),
    )
}

use crate::game::GameLayer;
use crate::game::actor::combat::health::Health;
use crate::game::actor::create_entity_image;
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
        Health::new(100.),
        create_entity_image(texture),
        Collider::rectangle(32., 32.),
        CollisionLayers::new(GameLayer::Enemy, LayerMask::ALL),
        ExternalForce::new(Vec2::ZERO).with_persistence(false),
        Restitution::new(0.75),
    )
}

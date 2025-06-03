use super::movement::input::MovementAction;
use crate::game::GameLayer;
use crate::game::actor::camera_cutie::CameraCutie;
use crate::game::actor::create_entity_sprite;
use crate::game::actor::movement::{
    DECELERATION_RATE_PIXELS, Movement, MovementController, WALKING_SPEED_PIXELS_PER_SECOND,
};
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<Player>();
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Player;

impl Configure for Player {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
    }
}

pub fn get_player(texture: Handle<Image>) -> impl Bundle {
    (
        Name::new("Player"),
        Player,
        Movement::new(
            DECELERATION_RATE_PIXELS,
            DECELERATION_RATE_PIXELS,
            WALKING_SPEED_PIXELS_PER_SECOND,
            1.0,
        ),
        MovementController::default(),
        InputMap::default()
            .with_dual_axis(MovementAction::Move, GamepadStick::LEFT)
            .with_dual_axis(MovementAction::Move, VirtualDPad::wasd()),
        create_entity_sprite(texture),
        CollisionLayers::new(GameLayer::Player, LayerMask::ALL),
        Collider::rectangle(32., 64.),
        ColliderDensity(5.0),
        CameraCutie,
    )
    //     Collider::rectangle(32.0, 64.0),
}

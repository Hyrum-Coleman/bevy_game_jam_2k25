use super::movement::input::MovementAction;
use crate::game::GameLayer;
use crate::game::actor::create_entity_aseprite;
use crate::game::actor::movement::{Movement, MovementController};
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

// Walking Speed is in ft/s (1ft=12px)
const WALKING_SPEED_FEET_PER_SECOND: f32 = 20.0;

const WALKING_SPEED_PIXELS_PER_SECOND: f32 = 12.0 * WALKING_SPEED_FEET_PER_SECOND;

const _SPRINT_MULTIPLIER: f32 = 2.0;

//ft/s^2
const ACCELERATION_RATE_FEET: f32 = 100.0;
const ACCELERATION_RATE_PIXELS: f32 = ACCELERATION_RATE_FEET * 12.0;
const DECELERATION_RATE_FEET: f32 = 50.0;
const DECELERATION_RATE_PIXELS: f32 = DECELERATION_RATE_FEET * 12.0;

pub fn get_player(texture: Handle<Aseprite>) -> impl Bundle {
    (
        Name::new("Player"),
        Player,
        Movement::new(
            ACCELERATION_RATE_PIXELS,
            DECELERATION_RATE_PIXELS,
            WALKING_SPEED_PIXELS_PER_SECOND,
            1.0,
        ),
        MovementController::default(),
        InputMap::default()
            .with_dual_axis(MovementAction::Move, GamepadStick::LEFT)
            .with_dual_axis(MovementAction::Move, VirtualDPad::wasd()),
        create_entity_aseprite(texture),
        CollisionLayers::new(GameLayer::Player, LayerMask::ALL),
        Collider::rectangle(32., 64.),
        ColliderDensity(5.0),
    )
}

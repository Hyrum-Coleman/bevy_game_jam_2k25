use super::movement::input::MovementAction;
use crate::game::GameLayer;
use crate::game::actor::combat::damage::Damage;
use crate::game::actor::combat::health::Health;
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
const ACCELERATION_RATE_FEET: f32 = 4000.0;
const ACCELERATION_RATE_PIXELS: f32 = ACCELERATION_RATE_FEET * 12.0;
const DECELERATION_RATE_FEET: f32 = 150.0;
const DECELERATION_RATE_PIXELS: f32 = DECELERATION_RATE_FEET * 12.0;

pub fn get_player(texture: Handle<Aseprite>) -> impl Bundle {
    (
        Name::new("Player"),
        Player,
        Health::new(100.),
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
        children![(
            Name::new("Player Collider"),
            CollisionLayers::new(GameLayer::Player, LayerMask::ALL),
            Collider::rectangle(32., 16.),
            Transform::from_xyz(0.0, -24.0, 0.0),
            ColliderDensity(5.0),
            CollisionEventsEnabled,
            Damage(5.),
        )],
        create_entity_aseprite(texture),
    )
}

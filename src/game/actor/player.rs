use super::movement::input::PlayerAction;
use crate::game::GameLayer;
use crate::game::actor::combat::damage::Damage;
use crate::game::actor::combat::health::Health;
use crate::game::actor::create_entity_aseprite;
use crate::game::actor::movement::{Movement, MovementController};
use crate::game::item::effects::fire::AppliesFire;
use crate::game::item::effects::poison::AppliesPoison;
use crate::game::world::Level;
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
        Health {
            max: 500.,
            current: 100.,
        },
        Movement::new(
            ACCELERATION_RATE_PIXELS,
            DECELERATION_RATE_PIXELS,
            WALKING_SPEED_PIXELS_PER_SECOND,
            1.0,
        ),
        MovementController::default(),
        InputMap::default()
            .with_dual_axis(PlayerAction::Move, GamepadStick::LEFT)
            .with_dual_axis(PlayerAction::Move, VirtualDPad::wasd())
            .with(PlayerAction::Shoot, MouseButton::Left)
            .with(PlayerAction::Dash, KeyCode::ShiftLeft),
        children![(
            Name::new("Player Collider"),
            CollisionLayers::new(GameLayer::Player, LayerMask::ALL),
            Collider::rectangle(32., 16.),
            Transform::from_xyz(0.0, -24.0, 0.0),
            ColliderDensity(5.0),
            CollisionEventsEnabled,
        )],
        create_entity_aseprite(texture, "Idle"),
    )
}

pub fn get_player_projectile(
    sprite: Handle<Aseprite>,
    trajectory: Vec2,
    angle: f32,
    player_offset: Vec2,
) -> impl Bundle {
    (
        Name::new("Projectile"),
        RigidBody::Dynamic,
        AseAnimation {
            aseprite: sprite,
            animation: Animation::from("Idle"),
        },
        Damage(5.),
        Sprite { ..default() },
        Transform {
            translation: vec3(
                player_offset.x + trajectory.x * 35.0,
                player_offset.y - trajectory.y * 64.0,
                5.0,
            ),
            rotation: Quat::from_rotation_z(angle),
            scale: Vec3::ONE,
        },
        LinearVelocity(vec2(500.0 * trajectory.x, -500.0 * trajectory.y)),
        Collider::capsule(5.0, 5.0),
        CollisionLayers::new(GameLayer::Projectile, LayerMask::ALL),
        CollisionEventsEnabled,
        AppliesFire::new(0.5),
        AppliesPoison::new(0.2),
        DespawnOnExitState::<Level>::Recursive,
    )
}

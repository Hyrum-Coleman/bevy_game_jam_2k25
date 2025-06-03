pub mod input;

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(Movement, MovementController)>();
    app.add_plugins((input::plugin));
}

// Walking Speed is in ft/s (1ft=12px)
pub(crate) const WALKING_SPEED_FEET_PER_SECOND: f32 = 7.0;

pub(crate) const WALKING_SPEED_PIXELS_PER_SECOND: f32 = 12.0 * WALKING_SPEED_FEET_PER_SECOND;

pub(crate) const SPRINT_MULTIPLIER: f32 = 2.0;

//ft/s^2
pub(crate) const DEACCELERATION_RATE_FEET: f32 = 50.0;
pub(crate) const DEACCELERATION_RATE_PIXELS: f32 = DEACCELERATION_RATE_FEET * 12.0;

#[derive(Component, Reflect, Serialize, Deserialize, Copy, Clone, Default)]
#[reflect(Component)]
#[serde(default)]
pub struct Movement {
    pub accel: f32,
    pub decel: f32,
    pub speed: f32,
    pub direction: f32,
}

impl Movement {
    pub fn new(accel: f32, decel: f32, speed: f32, direction: f32) -> Self {
        Self {
            accel,
            decel,
            speed,
            direction,
        }
    }
}

impl Configure for Movement {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(
            Update,
            apply_movement
                .in_set(UpdateSystems::Update)
                .run_if(Pause::is_disabled),
        );
    }
}

fn apply_movement(
    time: Res<Time>,
    mut movement_query: Query<(&Movement, &mut MovementController, &mut LinearVelocity)>,
) {
    let dt = time.delta_secs();

    for (movement, mut controller, mut velocity) in &mut movement_query {
        if controller.0 == Vec2::ZERO || velocity.0.length_squared() >= movement.speed.powi(2) {
            // Apply deceleration
            velocity.0 *= movement.decel.powf(dt);

            if velocity.x != 0.0 {
                let sign_x = velocity.x.signum();
                velocity.x -= DEACCELERATION_RATE_PIXELS * time.delta_secs() * sign_x;
                if velocity.x.signum() != sign_x {
                    velocity.x = 0.0
                }
            }
            if velocity.y != 0.0 {
                let sign_y = velocity.y.signum();
                velocity.y -= DEACCELERATION_RATE_PIXELS * time.delta_secs() * sign_y;
                if velocity.y.signum() != sign_y {
                    velocity.y = 0.0
                }
            }
        } else {
            // Apply acceleration
            velocity.0 += movement.accel * controller.0 * dt;
            velocity.0 = velocity.0.clamp_length_max(movement.speed);
            controller.0 = Vec2::ZERO;
        }
    }
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct MovementController(pub Vec2);

impl Configure for MovementController {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
    }
}

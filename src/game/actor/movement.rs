pub mod input;

use crate::prelude::*;
use std::cmp::Ordering;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(Movement, MovementController)>();
    app.add_plugins(input::plugin);
}

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
    mut movement_query: Query<(
        &Movement,
        &mut MovementController,
        &mut LinearVelocity,
        &mut Sprite,
    )>,
) {
    let dt = time.delta_secs();

    for (movement, mut controller, mut velocity, mut sprite) in &mut movement_query {
        if controller.0 == Vec2::ZERO || velocity.0.length_squared() >= movement.speed.powi(2) {
            if velocity.x != 0.0 {
                let sign_x = velocity.x.signum();
                velocity.x -= movement.decel * time.delta_secs() * sign_x;
                if velocity.x.signum() != sign_x {
                    velocity.x = 0.0
                }
            }
            if velocity.y != 0.0 {
                let sign_y = velocity.y.signum();
                velocity.y -= movement.decel * time.delta_secs() * sign_y;
                if velocity.y.signum() != sign_y {
                    velocity.y = 0.0
                }
            }
        } else {
            // Apply acceleration
            velocity.0 += movement.accel * controller.0 * dt;
            velocity.0 = velocity.0.clamp_length_max(movement.speed);

            let flip = match velocity.0.x.partial_cmp(&0.0).unwrap() {
                Ordering::Less => false,
                Ordering::Equal => false,
                Ordering::Greater => true,
            };
            sprite.flip_x = flip;
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

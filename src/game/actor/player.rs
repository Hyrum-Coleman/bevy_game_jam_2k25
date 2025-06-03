use super::movement::input::MovementAction;
use crate::core::camera::{CameraRoot, SmoothFollow};
use crate::game::GameLayer;
use crate::game::actor::create_entity_sprite;
use crate::game::actor::movement::{
    DEACCELERATION_RATE_PIXELS, Movement, MovementController, WALKING_SPEED_PIXELS_PER_SECOND,
};
use crate::prelude::*;
use crate::screen::gameplay::GameplayAssets;
use bevy::ecs::system::SystemState;

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
            DEACCELERATION_RATE_PIXELS,
            DEACCELERATION_RATE_PIXELS,
            WALKING_SPEED_PIXELS_PER_SECOND,
            1.0,
        ),
        MovementController::default(),
        InputMap::default()
            .with_dual_axis(MovementAction::Move, GamepadStick::LEFT)
            .with_dual_axis(MovementAction::Move, VirtualDPad::wasd()),
        create_entity_sprite(texture),
        CollisionLayers::new(GameLayer::Player, LayerMask::ALL),
        ColliderDensity(5.0),
    )

    // commands.spawn((
    //     Sprite {
    //         image: assets.character_sprite.clone(),
    //         ..default()
    //     },
    //     Transform::from_xyz(128., 128., 1.),
    //     Player {
    //         movement_direction: Vec3::default(),
    //     },
    //     LockedAxes::ROTATION_LOCKED,
    //     RigidBody::Dynamic,
    //     Collider::rectangle(32.0, 64.0),
    //     DespawnOnExitState::<Screen>::Recursive,
    // ));
}

// fn camera_follow_player(
//     mut camera_query: Query<&mut SmoothFollow>,
//     player_query: Query<Entity, With<Player>>,
// ) {
//     if let Ok(player) = player_query.single() {
//         if let Ok(mut follow) = camera_query.single_mut() {
//             follow.target = player;
//         }
//     }
// }

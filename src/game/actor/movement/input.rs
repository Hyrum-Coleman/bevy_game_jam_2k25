use crate::game::GameLayer;
use crate::game::actor::ActorAssets;
use crate::game::actor::combat::damage::Damage;
use crate::game::actor::movement::MovementController;
use crate::game::actor::player::Player;
use crate::game::world::Level;
use crate::prelude::*;
use std::f32::consts::PI;

pub(super) fn plugin(app: &mut App) {
    app.configure::<PlayerAction>();
}

#[derive(Actionlike, Eq, PartialEq, Hash, Copy, Clone, Reflect, Debug)]
pub(crate) enum PlayerAction {
    #[actionlike(DualAxis)]
    Move,
    Shoot,
    Dash,
}

impl Configure for PlayerAction {
    fn configure(app: &mut App) {
        app.init_resource::<ActionState<Self>>();
        app.add_plugins(InputManagerPlugin::<Self>::default());
        app.add_systems(
            Update,
            (
                record_movement_action
                    .in_set(UpdateSystems::RecordInput)
                    .run_if(Pause::is_disabled),
                spawn_projectile
                    .in_set(UpdateSystems::RecordInput)
                    //.run_if(action_just_pressed(Action::Shoot)) not working for some reason don't care why
                    .run_if(input_just_pressed(MouseButton::Left)),
            ),
        );
        app.add_observer(despawn_shot_on_collision);
    }
}

fn record_movement_action(
    mut commands: Commands,
    mut action_query: Query<(&ActionState<PlayerAction>, &mut MovementController)>,
) {
    for (action, mut controller) in &mut action_query {
        controller.0 += action
            .axis_pair(&PlayerAction::Move)
            .xy()
            .clamp_length_max(1.0);
    }
}

fn spawn_projectile(
    mut commands: Commands,
    window_query: Query<&Window>,
    player_query: Query<&Position, With<Player>>,
    assets: Res<ActorAssets>,
) {
    window_query.iter().for_each(|window| {
        let mouse_position = rq!(window.cursor_position());
        let window_center = vec2(window.width() / 2.0, window.height() / 2.0);

        let trajectory = mouse_position - window_center;
        let clamped_traj = trajectory.clamp_length_max(1.0).normalize();
        let player_position = r!(player_query.single());

        let bounded_angle = f32::atan(trajectory.x / trajectory.y);
        let angle = if trajectory.y > 0.0 {
            bounded_angle + PI
        } else {
            bounded_angle
        };

        commands.spawn((
            Name::new("Projectile"),
            RigidBody::Dynamic,
            AseAnimation {
                aseprite: assets.projectile_image.clone(),
                animation: Animation::from("Idle"),
            },
            Damage(5.),
            Sprite { ..default() },
            Transform {
                translation: vec3(
                    player_position.x + clamped_traj.x * 35.0,
                    player_position.y - clamped_traj.y * 35.0,
                    5.0,
                ),
                rotation: Quat::from_rotation_z(angle),
                scale: Vec3::ONE,
            },
            LinearVelocity(vec2(500.0 * clamped_traj.x, -500.0 * clamped_traj.y)),
            Collider::capsule(5.0, 5.0),
            CollisionLayers::new(GameLayer::Projectile, LayerMask::ALL),
            CollisionEventsEnabled,
            DespawnOnExitState::<Level>::Recursive,

        ));
    });
}

fn despawn_shot_on_collision(trigger: Trigger<OnCollisionStart>, name_query: Query<&Name>, mut commands: Commands) {
    let projectile = r!(trigger.get_target());
    let name = r!(name_query.get(projectile));

    if name.as_str() == "Projectile" {
        commands.entity(projectile).despawn();
    }
}

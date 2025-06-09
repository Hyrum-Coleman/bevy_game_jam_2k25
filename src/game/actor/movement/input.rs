use crate::game::GameLayer;
use crate::game::actor::ActorAssets;
use crate::game::actor::combat::damage::Damage;
use crate::game::actor::movement::MovementController;
use crate::game::actor::player::Player;
use crate::game::world::Level;
use crate::prelude::*;

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
        let clamped_traj = trajectory.clamp_length_max(1.0);
        info!("Trajectory: {}", trajectory);
        info!("Clamped Traj: {}", clamped_traj);

        let player_position = r!(player_query.single());

        commands.spawn((
            Name::new("Projectile"),
            Transform::from_xyz(
                player_position.x + clamped_traj.x * 35.0,
                player_position.y - clamped_traj.y * 35.0,
                5.0,
            ),
            RigidBody::Dynamic,
            AseAnimation {
                aseprite: assets.projectile_image.clone(),
                animation: Animation::from("Idle"),
            },
            Damage(5.),
            Sprite {..default()},
            LinearVelocity(vec2(500.0 * clamped_traj.x, -500.0 * clamped_traj.y)),
            Collider::capsule(5.0, 5.0),
            CollisionLayers::new(GameLayer::Projectile, LayerMask::ALL),
            DespawnOnExitState::<Level>::Recursive,
        ));
    });
}

fn despawn_shot_on_collision() {

}

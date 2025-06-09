use crate::game::actor::ActorAssets;
use crate::game::actor::movement::MovementController;
use crate::game::actor::player::{Player, get_player_projectile};
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

        commands.spawn(get_player_projectile(
            assets.projectile_image.clone(),
            clamped_traj,
            angle,
            player_position.0,
        ));
    });
}

fn despawn_shot_on_collision(
    trigger: Trigger<OnCollisionStart>,
    name_query: Query<&Name>,
    mut commands: Commands,
) {
    let projectile = r!(trigger.get_target());
    let name = r!(name_query.get(projectile));

    if name.as_str() == "Projectile" {
        commands.entity(projectile).despawn();
    }
}

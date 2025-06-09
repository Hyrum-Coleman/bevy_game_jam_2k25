pub mod camera_cutie;
pub mod combat;
pub mod enemy;
pub mod facing;
pub mod movement;
pub mod player;

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<ActorAssets>();
    app.add_plugins((
        movement::plugin,
        facing::plugin,
        player::plugin,
        enemy::plugin,
        combat::plugin,
        camera_cutie::plugin,
    ));
}

#[derive(AssetCollection, Resource, Reflect, Default, Debug)]
#[reflect(Resource)]
pub struct ActorAssets {
    #[asset(path = "image/Player.png")]
    pub player_image: Handle<Image>,
    #[asset(path = "image/Orc_Guy.png")]
    pub orc_image: Handle<Image>,
    #[asset(path = "image/Rat_Base.aseprite")]
    pub rat_handle: Handle<Aseprite>,
    #[asset(path = "image/Pellet.aseprite")]
    pub projectile_image: Handle<Aseprite>,
}

impl Configure for ActorAssets {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_plugins(AsepriteUltraPlugin);
        app.init_collection::<Self>();
    }
}

fn create_entity_aseprite(sprite: Handle<Aseprite>) -> impl Bundle {
    (
        AseAnimation {
            aseprite: sprite,
            animation: Animation::tag("Idle")
                .with_repeat(AnimationRepeat::Loop)
                .with_speed(1.75),
        },
        Sprite { ..default() },
        RigidBody::Dynamic,
        LockedAxes::ROTATION_LOCKED,
    )
}

fn create_entity_image(sprite: Handle<Image>) -> impl Bundle {
    (
        Sprite {
            image: sprite,
            ..default()
        },
        RigidBody::Dynamic,
        LockedAxes::ROTATION_LOCKED,
    )
}

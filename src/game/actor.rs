pub mod camera_cutie;
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
}

impl Configure for ActorAssets {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.init_collection::<Self>();
    }
}

fn create_entity_sprite(sprite: Handle<Image>) -> impl Bundle {
    (
        Sprite::from_image(sprite),
        RigidBody::Kinematic,
        LockedAxes::ROTATION_LOCKED,
    )
}

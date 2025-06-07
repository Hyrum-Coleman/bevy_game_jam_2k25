use crate::prelude::*;
pub(super) fn plugin(app: &mut App) {
    app.configure::<Spring>();
}

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct Spring {
    pub stiffness: f32,
    pub offset: Vec2,
}

impl Configure for Spring {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(
            Update,
            apply_spring
                .in_set(UpdateSystems::Update)
                .run_if(Pause::is_disabled),
        );
    }
}

fn apply_spring(query: Query<(&Spring, &mut ExternalForce, &mut Position)>) {
    for (spring, mut external_force, position) in query {
        let k = spring.stiffness;
        let x = -position.0 + spring.offset;
        external_force.apply_force(k * x);
    }
}

impl Spring {
    pub fn with_stiffness(mut self, stiffness_val: f32) -> Self {
        self.stiffness = stiffness_val;
        self
    }

    pub fn with_offset(mut self, offset: Vec2) -> Self {
        self.offset = offset;
        self
    }
}

pub fn mass_spring_damper(m: f32, k: f32, b: f32, pos: Vec2) -> impl Bundle {
    (
        Mass(m),
        Spring {
            stiffness: k,
            offset: pos,
        },
        LinearDamping(b),
    )
}

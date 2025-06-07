use crate::prelude::*;
use nalgebra::{dmatrix, matrix, Const, DMatrix, Matrix, Matrix3};

pub(super) fn plugin(app: &mut App) {
    app.configure::<LevelGenerator>();
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct LevelGenerator {}

impl Configure for LevelGenerator {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
    }
}

fn create_level_shape(passes: u32) -> HashSet<(i32, i32)> {
    let mut points: HashSet<(i32, i32)> = HashSet::new();
    let mut location = (0, 0);
    let mut rng = rand::thread_rng();
    for _ in 1..=passes {
        let value = rng.gen_range(1..=4);
        let tuple = match value {
            1 => (0, 1),
            2 => (1, 0),
            3 => (0, -1),
            4 => (-1, 0),
            _ => panic!("Random Number Generator generated out of range."),
        };
        location.0 += tuple.0;
        location.1 += tuple.1;
        points.insert(location);
    }
    points
}

fn create_level_connections() {
    const PASSES: u32 = 4;
    let map_shape = create_level_shape(PASSES);
    let map_size = map_shape.len();
    let mut base_matrix = DMatrix::<f32>::zeros(map_size, map_size);
    
}

fn only_one_connection(point:(i32,i32),set:HashSet<(i32,i32)>)-> Option<u32> {
    let (point1,point2)=point;
    let mut direction = 0u32;
    let mut connections = 0u32;
    if set.contains(&(point1,point2+1)){
        direction=1;
        connections+=1;
    };
    if set.contains(&(point1+1,point2)){
        direction=2;
        connections+=1;
    };
    if set.contains(&(point1,point2-1)){
        direction=3;
        connections+=1;
    };
    if set.contains(&(point1-1,point2)){
        direction=4;
        connections+=1;
    };
    if connections==1{
        return Some(direction);
    }
    None
}
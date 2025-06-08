use crate::prelude::*;
use nalgebra::{coordinates, dmatrix, matrix, Const, DMatrix, Matrix, Matrix3};

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

#[derive(PartialEq,Clone)]
enum Direction{
    Left,
    Right,
    Up,
    Down
}

impl Direction{
    fn flip(&self)->Self{
        match self{
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
        }
    }
}

#[derive(Clone)]
struct Map{
    rooms:HashMap<(i32,i32),Room>,
}

impl Map{
    fn new(map_shape:HashSet<(i32,i32)>)->Self{
        let mut rooms:HashMap<(i32,i32), Room> = HashMap::new();
        for coordinate in map_shape.clone(){
            add_connection(coordinate, &mut rooms, &map_shape);
        }
        Self { rooms }
    }
    fn cut_square_connections(&mut self){
        let mut rng = rand::thread_rng();
        for ((x,y),_) in self.rooms.clone(){
            if check_square((x,y), self.clone()){
                let value = rng.gen_range(1..=4);
                let (room_location,direction)=match value {
                    1=>((x,y),Direction::Up),
                    2=>((x,y+1),Direction::Right),
                    3=>((x+1,y+1),Direction::Down),
                    4=>((x+1,y),Direction::Left),
                    _=> panic!("Random Number Generator generated out of range.")
                };
                self.disconnect_rooms(room_location, direction);
                
            }
        }
    }
    //Use only if it is assured that the rooms in question exist
    fn disconnect_rooms(&mut self,(x,y):(i32,i32),direction:Direction) {
        let main_room = r!(self.rooms.get_mut(&(x,y)));
        main_room.disconnect_one_side(direction.clone());
        let room_two_location = match direction{
            Direction::Left => (x-1,y),
            Direction::Right => (x+1,y),
            Direction::Up => (x,y+1),
            Direction::Down => (x,y-1),
        };
        let room_two = r!(self.rooms.get_mut(&room_two_location));
        room_two.disconnect_one_side(direction.flip());
    }   
}

#[derive(Clone)]
struct Room {
    connections: Connections,
}

impl Room{
    fn new()->Self{
        Room { connections: Connections::new() }
    }
    fn disconnect_one_side(&mut self,direction:Direction){
        match direction{
            Direction::Left => self.connections.left=None,
            Direction::Right => self.connections.right=None,
            Direction::Up => self.connections.up=None,
            Direction::Down => self.connections.down=None,
        }
    }

}

#[derive(Clone)]
struct Connections{
    left:Option<(i32,i32)>,
    right:Option<(i32,i32)>,
    up:Option<(i32,i32)>,
    down:Option<(i32,i32)>,
}

impl Connections{
    fn new()->Self{
        Self { left: None, right: None, up: None, down: None }
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
        for _ in 1..=rng.gen_range(1..=2) {
            location.0 += tuple.0;
            location.1 += tuple.1;
            points.insert(location);
        }
    }
    points
}

fn create_map_connections() {
    const PASSES: u32 = 4;
    let map_shape = create_level_shape(PASSES);
    let mut map:Map =  Map::new(map_shape);
    map.cut_square_connections();
}

fn create_connections((x,y):(i32,i32),set:&HashSet<(i32,i32)>)->Connections{
    let mut connections=Connections::new();
    if set.contains(&(x,y+1)){
        connections.up=Some((x,y+1))
    }
    if set.contains(&(x+1,y)){
        connections.right=Some((x+1,y))
    }
    if set.contains(&(x,y-1)){
        connections.down=Some((x,y-1))
    }
    if set.contains(&(x-1,y)){
        connections.left=Some((x-1,y))
    }
    connections
}

fn add_connection(coordinate:(i32,i32),rooms:&mut HashMap<(i32,i32), Room>,set:&HashSet<(i32,i32)>){
    let connections = create_connections(coordinate, set);
    let mut room=Room::new();
    room.connections=connections;
    rooms.insert(coordinate,  room);
}

fn check_square(coordinate:(i32,i32),map:Map)->bool{
    let rooms = map.rooms;
    let room_north= match rooms.get(&coordinate){
        Some(room) => match room.connections.up{
            Some(room_north) => room_north,
            None => return false,
        },
        None => return false,
    };
    let room_east= match rooms.get(&room_north){
        Some(room) => match room.connections.right{
            Some(room_east) => room_east,
            None => return false,
        },
        None => return false,
    };
    let room_south= match rooms.get(&room_east){
        Some(room) => match room.connections.down{
            Some(room_south) => room_south,
            None => return false,
        },
        None => return false,
    };
    let room_west= match rooms.get(&room_south){
        Some(room) => match room.connections.left{
            Some(room_west) => room_west,
            None => return false,
        },
        None => return false,
    };
    if room_west==coordinate{
        true
    } else {
        false
    }

}


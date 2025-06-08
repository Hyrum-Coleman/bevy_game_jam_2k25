use crate::prelude::*;

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

#[derive(PartialEq, Clone)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn flip(&self) -> Self {
        match self {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
        }
    }
}

#[derive(Copy, Clone, Debug, Default)]
enum WorldType {
    #[default]
    Dungeon,
}

impl std::fmt::Display for WorldType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

#[derive(Clone)]
pub struct Map {
    rooms: HashMap<(i32, i32), Room>,
    world_type: WorldType,
}

impl Map {
    fn new(map_shape: HashSet<(i32, i32)>) -> Self {
        let mut rooms: HashMap<(i32, i32), Room> = HashMap::new();
        for coordinate in map_shape.clone() {
            add_connection(coordinate, &mut rooms, &map_shape);
        }
        Self {
            rooms,
            world_type: WorldType::Dungeon,
        }
    }
    fn cut_square_connections(&mut self) {
        let mut rng = rand::thread_rng();
        for ((x, y), _) in self.rooms.clone() {
            if check_square((x, y), self.clone()) {
                let value = rng.gen_range(1..=4);
                let (room_location, direction) = match value {
                    1 => ((x, y), Direction::Up),
                    2 => ((x, y + 1), Direction::Right),
                    3 => ((x + 1, y + 1), Direction::Down),
                    4 => ((x + 1, y), Direction::Left),
                    _ => panic!("Random Number Generator generated out of range."),
                };
                self.disconnect_rooms(room_location, direction);
            }
        }
    }
    //Use only if it is assured that the rooms in question exist
    fn disconnect_rooms(&mut self, (x, y): (i32, i32), direction: Direction) {
        let main_room = r!(self.rooms.get_mut(&(x, y)));
        main_room.disconnect_one_side(direction.clone());
        let room_two_location = match direction {
            Direction::Left => (x - 1, y),
            Direction::Right => (x + 1, y),
            Direction::Up => (x, y + 1),
            Direction::Down => (x, y - 1),
        };
        let room_two = r!(self.rooms.get_mut(&room_two_location));
        room_two.disconnect_one_side(direction.flip());
    }
    pub fn shaped(passes: u32) -> Self {
        let map_shape = create_level_shape(passes);
        let mut map: Map = Map::new(map_shape);
        map.cut_square_connections();
        map
    }
    fn diversify(&mut self) {
        let mut rng = rand::thread_rng();
        let boss_index = rng.gen_range(1..=self.clone().size());
        let mut chest_index = boss_index;
        while chest_index == boss_index {
            chest_index = rng.gen_range(1..=self.clone().size());
        }
        let temp_map = self.clone();
        let rooms_keys = temp_map.rooms.keys().cloned();
        for key in rooms_keys.clone() {
            let random = rng.gen_range(1..=4);
            match random {
                1 | 2 | 3 => r!(self.rooms.get_mut(&key)).change_type(RoomType::Open),
                4 => r!(self.rooms.get_mut(&key)).change_type(RoomType::Courtyard),
                _ => panic!("Random generated out of range!"),
            }
        }
        let rooms_vec: Vec<(i32, i32)> = rooms_keys.collect();
        r!(self.rooms.get_mut(&rooms_vec[boss_index])).change_type(RoomType::Boss);
        r!(self.rooms.get_mut(&rooms_vec[chest_index])).change_type(RoomType::Chest);
    }
    pub fn full(passes: u32) -> Self {
        let mut map = Map::shaped(passes);
        map.diversify();
        map
    }
    fn size(self) -> usize {
        self.rooms.len()
    }
    fn create_world_info(&self) -> WorldData {
        let mut maps = Vec::new();
        for (coordiate, room) in &self.rooms {
            let map = room.create_room_info(self.world_type, coordiate);
            maps.push(map);
        }
        WorldData::new(maps, true, "world")
    }
    pub fn save_world_file(&self) {
        let world_data = self.create_world_info();
        let world_type = self.world_type;
        let json = serde_json::to_string(&world_data).expect("Aaron Made Bad Struct");
        std::fs::write(format!("assets/maps/{}.world", world_type), json)
            .expect("File System Failed");
    }
}

#[derive(Copy, Clone, Debug, Default)]
enum RoomType {
    Boss,
    Open,
    #[default]
    Empty,
    Chest,
    Courtyard,
}

impl std::fmt::Display for RoomType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

#[derive(Clone, Default)]
struct Room {
    connections: Connections,
    room_type: RoomType,
}

#[derive(Serialize, Deserialize)]
struct MapData {
    #[serde(rename = "fileName")]
    file_name: String,
    height: i32,
    width: i32,
    x: i32,
    y: i32,
}
impl MapData {
    fn new(file_name: String, height: i32, width: i32, x: i32, y: i32) -> Self {
        MapData {
            file_name,
            height,
            width,
            x,
            y,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct WorldData {
    maps: Vec<MapData>,
    #[serde(rename = "onlyShowAdjacentMaps")]
    only_show_adjacent_maps: bool,
    #[serde(rename = "type")]
    file_type: String,
}

impl WorldData {
    fn new(
        maps: Vec<MapData>,
        only_show_adjacent_maps: bool,
        file_type: impl Into<String>,
    ) -> Self {
        let file_type = file_type.into();
        WorldData {
            maps,
            only_show_adjacent_maps,
            file_type,
        }
    }
}

impl Room {
    fn disconnect_one_side(&mut self, direction: Direction) {
        match direction {
            Direction::Left => self.connections.left = None,
            Direction::Right => self.connections.right = None,
            Direction::Up => self.connections.up = None,
            Direction::Down => self.connections.down = None,
        }
    }
    fn change_type(&mut self, room_type: RoomType) {
        self.room_type = room_type;
    }
    fn create_room_path(&self, world_type: WorldType) -> String {
        let door_code = self.connections.door_code();
        let room_type = self.room_type;
        format!("{}/Room{}/Map{}.tmx", world_type, room_type, door_code)
    }
    fn create_room_info(&self, world_type: WorldType, (x, y): &(i32, i32)) -> MapData {
        const HEIGHT: i32 = 640;
        const WIDTH: i32 = 960;
        let file_name = self.create_room_path(world_type);
        MapData::new(file_name, HEIGHT, WIDTH, x * WIDTH, y * HEIGHT)
    }
}

#[derive(Clone, Default)]
struct Connections {
    left: Option<(i32, i32)>,
    right: Option<(i32, i32)>,
    up: Option<(i32, i32)>,
    down: Option<(i32, i32)>,
}

impl Connections {
    fn door_code(&self) -> String {
        let mut code = String::new();
        match self.up {
            Some(_) => code += "T",
            None => code += "x",
        }
        match self.right {
            Some(_) => code += "R",
            None => code += "x",
        }
        match self.down {
            Some(_) => code += "B",
            None => code += "x",
        }
        match self.left {
            Some(_) => code += "L",
            None => code += "x",
        }
        code
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

fn create_connections((x, y): (i32, i32), set: &HashSet<(i32, i32)>) -> Connections {
    let mut connections = Connections::default();
    if set.contains(&(x, y + 1)) {
        connections.up = Some((x, y + 1))
    }
    if set.contains(&(x + 1, y)) {
        connections.right = Some((x + 1, y))
    }
    if set.contains(&(x, y - 1)) {
        connections.down = Some((x, y - 1))
    }
    if set.contains(&(x - 1, y)) {
        connections.left = Some((x - 1, y))
    }
    connections
}

fn add_connection(
    coordinate: (i32, i32),
    rooms: &mut HashMap<(i32, i32), Room>,
    set: &HashSet<(i32, i32)>,
) {
    let connections = create_connections(coordinate, set);
    let mut room = Room::default();
    room.connections = connections;
    rooms.insert(coordinate, room);
}

fn check_square(coordinate: (i32, i32), map: Map) -> bool {
    let rooms = map.rooms;
    let room_north = match rooms.get(&coordinate) {
        Some(room) => match room.connections.up {
            Some(room_north) => room_north,
            None => return false,
        },
        None => return false,
    };
    let room_east = match rooms.get(&room_north) {
        Some(room) => match room.connections.right {
            Some(room_east) => room_east,
            None => return false,
        },
        None => return false,
    };
    let room_south = match rooms.get(&room_east) {
        Some(room) => match room.connections.down {
            Some(room_south) => room_south,
            None => return false,
        },
        None => return false,
    };
    let room_west = match rooms.get(&room_south) {
        Some(room) => match room.connections.left {
            Some(room_west) => room_west,
            None => return false,
        },
        None => return false,
    };
    if room_west == coordinate { true } else { false }
}

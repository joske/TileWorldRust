extern crate rand;
use rand::Rng;

const COLS: usize = 10;
const ROWS: usize = 10;

pub struct Grid {
    agents: Vec<GridObject>,
    tiles: Vec<GridObject>,
    holes: Vec<GridObject>,
    obstacles: Vec<GridObject>,
    objects: [[Option<GridObject>; COLS]; ROWS],
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Copy, Clone)]
pub struct Location {
    col: usize,
    row: usize,
}

#[derive(Copy, Clone)]
enum Type {
    Agent,
    Tile,
    Hole,
    Obstacle,
}

#[derive(Copy, Clone)]
struct GridObject {
    location: Location,
    object_type: Type,
    score: u32,
    id: u8,
}

impl Grid {
    pub fn new() -> Self {
        Grid {
            agents: Vec::with_capacity(5),
            tiles: Vec::with_capacity(5),
            holes: Vec::with_capacity(5),
            obstacles: Vec::with_capacity(5),
            objects: [[None; COLS]; ROWS],
        }
    }

    pub fn init(&mut self) {
        let mut rng = rand::thread_rng();
        for i in 1..5 {
            let a = GridObject {
                location: self.random_location(),
                object_type: Type::Agent,
                score: 0,
                id: i,
            };
            self.agents.push(a);
            self.set_object(&a, &a.location);
        }
        for i in 1..5 {
            let t = GridObject {
                location: self.random_location(),
                object_type: Type::Tile,
                score: rng.gen_range(1..6),
                id: i,
            };
            self.tiles.push(t);
            self.set_object(&t, &t.location);
        }
        for i in 1..5 {
            let h = GridObject {
                location: self.random_location(),
                object_type: Type::Hole,
                score: 0,
                id: i,
            };
            self.holes.push(h);
            self.set_object(&h, &h.location);
        }
        for i in 1..5 {
            let o = GridObject {
                location: self.random_location(),
                object_type: Type::Obstacle,
                score: 0,
                id: i,
            };
            self.obstacles.push(o);
            self.set_object(&o, &o.location);
        }
    }

    fn set_object(&mut self, o: &GridObject, l: &Location) {
        self.objects[l.col][l.row] = Some(*o);
    }

    fn is_free(&self, location: Location) -> bool {
        let o = &self.objects[location.col][location.row];
        match o {
            None => true,
            &Some(_) => false,
        }
    }

    fn random_location(&self) -> Location {
        let mut rng = rand::thread_rng();
        let mut c: usize = rng.gen_range(1..COLS);
        let mut r: usize = rng.gen_range(1..ROWS);

        let mut l = Location { col: c, row: r };
        while !self.is_free(l) {
            c = rng.gen_range(1..COLS);
            r = rng.gen_range(1..ROWS);
            l = Location { col: c, row: r };
        }
        return l;
    }

    pub fn update(&mut self) {}

    pub fn print(&mut self) {
        for c in 0..COLS {
            for r in 0..ROWS {
                let l = Location { col: c, row: r };
                if !self.is_free(l) {
                    let o = &self.objects[l.col][l.row];
                    match o.unwrap().object_type {
                        Type::Agent => print!("A"),
                        Type::Hole => print!("H"),
                        Type::Tile => print!("{}", o.unwrap().score.to_string()),
                        Type::Obstacle => print!("#"),
                    }
                } else {
                    print!(".");
                }
            }
            print!("\n");
        }        
    }
}

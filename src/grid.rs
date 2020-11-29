pub struct Grid {
    agents : Vec<Agent>,
    tiles : Vec<Tile>,
    holes : Vec<Hole>,
    obstacles : Vec<Obstacle>,
    objects : [[Option<Box<GridObject>>; 5] ; 5],
}

struct Location 
{
    col : u8,
    row : u8,
}

struct GridObject {
    grid : Grid,
    location : Location,
}
struct Agent {
    parent:GridObject
}
struct Tile {
    parent:GridObject
}
struct Hole {
    parent:GridObject
}
struct Obstacle {
    parent:GridObject
}

impl GridObject {
}
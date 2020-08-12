#[derive(Clone,Copy)]
pub struct Tile {
  Floor,
  Wall,
}

pub type Map = [Tile; 4000];

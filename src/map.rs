use bracket_lib::prelude::RandomNumberGenerator as RNG;

#[derive(Clone,Copy,PartialEq)]
pub enum Tile {
  Floor,
  Wall,
}

pub type Map = [Tile; 4000];

pub fn new_map() -> Map {
  // we start without any wall
  let mut map = [Tile::Floor; 80 * 50];

  // we replace the outer edges with walls
  for x in 0..80 {
    map[index_of(x, 0)] = Tile::Wall;
    map[index_of(x, 49)] = Tile::Wall;
  }
  for y in 1..49 {
    map[index_of(0, y)] = Tile::Wall;
    map[index_of(79, y)] = Tile::Wall;
  }

  let mut rng = RNG::new();

  // we randomly place up to 400 walls
  for _ in 0..400 {
    let x = rng.range(0, 80);
    let y = rng.range(0, 50);
    let idx = index_of(x, y);
    if idx != index_of(40, 25) {
      map[idx] = Tile::Wall;
    }
  }

  map
}

#[allow(dead_code)]
pub fn index_of(x: i32, y: i32) -> usize {
    ((y * 80) + x) as usize
}
#[allow(dead_code)]
pub fn coords_of(i: usize) -> (i32, i32) {
    ((i % 80) as i32, (i / 80) as i32)
}

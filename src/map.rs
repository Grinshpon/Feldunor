use bracket_lib::prelude::RandomNumberGenerator as RNG;

#[derive(Debug,Clone,Copy,PartialEq)]
pub enum Tile {
  Floor,
  Wall,
}

#[derive(Debug)]
pub struct Map {
  pub tiles: Vec<Tile>,
}

pub fn new_map(width: i32, height: i32) -> Map {
  let area = (width*height) as usize;
  // we start without any wall
  let mut tiles = [Tile::Floor].repeat(area);

  // we replace the outer edges with walls
  for x in 0..(width-1) {
    tiles[index_of(x, 0)] = Tile::Wall;
    tiles[index_of(x, height-1)] = Tile::Wall;
  }
  for y in 0..(height) {
    tiles[index_of(0, y)] = Tile::Wall;
    tiles[index_of(width-1, y)] = Tile::Wall;
  }

  let mut rng = RNG::new();

  // we randomly place up to 400 walls
  for _ in 0..400 {
    let x = rng.range(0, width);
    let y = rng.range(0, height);
    let idx = index_of(x, y);
    if idx != index_of(40, 25) {
      tiles[idx] = Tile::Wall;
    }
  }

  Map { tiles }
}

#[allow(dead_code)]
pub fn index_of(x: i32, y: i32) -> usize {
    ((y * 80) + x) as usize
}
#[allow(dead_code)]
pub fn coords_of(i: usize) -> (i32, i32) {
    ((i % 80) as i32, (i / 80) as i32)
}

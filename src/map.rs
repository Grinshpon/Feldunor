use std::cmp::{max, min};
use bracket_lib::prelude::RandomNumberGenerator as RNG;
use bracket_lib::prelude::{Algorithm2D, BaseMap, DistanceAlg, Point, SmallVec};
//use delaunator::*;

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum Tile {
  Floor,
  Wall,
}

#[derive(Debug)]
pub struct Map {
  pub tiles: Vec<Tile>,
  pub revealed_tiles: Vec<bool>,
  pub blocked_tiles: Vec<bool>,
  pub rooms: Vec<Room>,
  pub width: u32,
  pub height: u32,
}

impl Map {
  pub fn index_of(&self, x: i32, y: i32) -> usize {
      (((y as u32) * self.width) + (x as u32)) as usize
  }
  pub fn coords_of(&self, i: usize) -> (i32, i32) {
      ((i % self.width as usize) as i32, (i / self.width as usize) as i32)
  }
  pub fn new(width: u32, height: u32) -> Self {
    let area = (width*height) as usize;
    let max_rooms: i32 = (area as f32).log2().round() as i32;
    //println!("{}",max_rooms);
    let min_size: i32 = 5;
    let max_size: i32 = min(width as i32,height as i32)/2 - 5;

    let mut rng = RNG::new();

    let mut map = Map {
      tiles: vec![Tile::Wall;area],
      revealed_tiles: vec![false; area],
      blocked_tiles: vec![false; area],
      rooms: Vec::new(),
      width,
      height,
    };

    for _ in 0..max_rooms {
      let mut ok = false;
      let mut timeout = 100;
      while !ok && timeout > 0 {
        ok = true;
        let w = rng.range(min_size, max_size);
        let h = rng.range(min_size, max_size);
        // we're not allowing rooms to be built on the outer edge
        let x = rng.range(1, width as i32 - 1 - w);
        let y = rng.range(1, height as i32 - 1 - h);

        let room = Room::new(x,y,w,h);

        for other in map.rooms.iter() {
          if room.intersects(other) {
            ok = false;
          }
        }
        if ok {
          map.dig_room(&room);
          if let Some(last_room) = map.rooms.last() {
            let [new_x, new_y] = room.center();
            let [prev_x, prev_y] = last_room.center();
            if rng.range(0, 2) == 1 {
              map.dig_horizontal_tunnel(prev_x, new_x, prev_y);
              map.dig_vertical_tunnel(prev_y, new_y, new_x);
            } else {
              map.dig_vertical_tunnel(prev_y, new_y, prev_x);
              map.dig_horizontal_tunnel(prev_x, new_x, new_y);
            }
          }
          map.rooms.push(room);
        }
        else {
          timeout -= 1;
        }
      }
      //println!("{}", timeout);
    }

    map.populate_blocked();

    map
  }
  pub fn populate_blocked(&mut self) {
    for (i,tile) in self.tiles.iter_mut().enumerate() {
      self.blocked_tiles[i] = *tile == Tile::Wall;
    }
  }
  fn is_exit_valid(&self, x:i32, y:i32) -> bool {
    if (x < 1) || (x as u32 > self.width-1) || (y < 1) || (y as u32 > self.height-1) {
      false
    }
    else {
      let ix = self.index_of(x,y);
      !self.blocked_tiles[ix]
    }
  }
  fn dig_room(&mut self, room: &Room) {
    for y in room.y1..=room.y2 {
      for x in room.x1..=room.x2 {
        let ix = self.index_of(x,y);
        self.tiles[ix] = Tile::Floor;
      }
    }
  }

  fn dig_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
    for x in min(x1, x2)..=max(x1, x2) {
      let ix = self.index_of(x, y);
      if ix > 0 && ix < 80 * 50 {
        self.tiles[ix as usize] = Tile::Floor;
      }
    }
  }

  fn dig_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
    for y in min(y1, y2)..=max(y1, y2) {
      let ix = self.index_of(x, y);
      if ix > 0 && ix < 80 * 50 {
        self.tiles[ix as usize] = Tile::Floor;
      }
    }
  }
}

impl BaseMap for Map {
  fn is_opaque(&self, ix: usize) -> bool {
    self.tiles[ix] == Tile::Wall
  }
  fn get_available_exits(&self, ix: usize) -> SmallVec<[(usize,f32); 10]> {
    let mut exits = SmallVec::new();
    let (x,y) = self.coords_of(ix);
    let w = self.width as usize;

    // Cardinal directions
    if self.is_exit_valid(x-1, y) { exits.push((ix-1, 1.0)) };
    if self.is_exit_valid(x+1, y) { exits.push((ix+1, 1.0)) };
    if self.is_exit_valid(x, y-1) { exits.push((ix-w, 1.0)) };
    if self.is_exit_valid(x, y+1) { exits.push((ix+w, 1.0)) };

    // Diagonals
    if self.is_exit_valid(x-1, y-1) { exits.push(((ix-w)-1, 1.45)); }
    if self.is_exit_valid(x+1, y-1) { exits.push(((ix-w)+1, 1.45)); }
    if self.is_exit_valid(x-1, y+1) { exits.push(((ix+w)-1, 1.45)); }
    if self.is_exit_valid(x+1, y+1) { exits.push(((ix+w)+1, 1.45)); }

    exits
  }
  fn get_pathing_distance(&self, ix1: usize, ix2: usize) -> f32 {
    let w = self.width as usize;
    let p1 = Point::new(ix1%w, ix1/w);
    let p2 = Point::new(ix2%w, ix2/w);
    DistanceAlg::Pythagoras.distance2d(p1,p2)
  }
}

impl Algorithm2D for Map {
  fn dimensions(&self) -> Point {
    Point::new(self.width, self.height)
  }
}

#[derive(Debug)]
pub struct Room {
  x1: i32,
  y1: i32,
  x2: i32,
  y2: i32,
}
impl Room {
  pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
    Room {
      x1: x,
      y1: y,
      x2: x + w - 1,
      y2: y + h - 1,
    }
  }
  pub fn intersects(&self, other: &Room) -> bool {
    self.x1 - 1 <= other.x2
      && self.x2 + 1 >= other.x1
      && self.y1 - 1 <= other.y2
      && self.y2 + 1 >= other.y1
  }
  pub fn center(&self) -> [i32;2] {
    [(self.x1 + self.x2) / 2, (self.y1 + self.y2) / 2]
  }
}

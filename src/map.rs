use std::cmp::{max, min};
use bracket_lib::prelude::RandomNumberGenerator as RNG;
use bracket_lib::prelude::{Algorithm2D, BaseMap, Point};
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
  pub rooms: Vec<Room>,
  pub width: u32,
  pub height: u32,
}

impl Map {
  pub fn new(width: u32, height: u32) -> Self {
    let area = (width*height) as usize;
    let max_rooms: i32 = (area as f32).log2().round() as i32;
    //println!("{}",max_rooms);
    let min_size: i32 = 5;
    let max_size: i32 = min(width as i32,height as i32)/2 - 5;

    let mut rng = RNG::new();
    let mut tiles = vec![Tile::Wall;area];

    let mut rooms: Vec<Room> = Vec::new();
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

        for other in rooms.iter() {
          if room.intersects(other) {
            ok = false;
          }
        }
        if ok {
          dig_room(&mut tiles, &room);
          if let Some(last_room) = rooms.last() {
            let [new_x, new_y] = room.center();
            let [prev_x, prev_y] = last_room.center();
            if rng.range(0, 2) == 1 {
              dig_horizontal_tunnel(&mut tiles, prev_x, new_x, prev_y);
              dig_vertical_tunnel(&mut tiles, prev_y, new_y, new_x);
            } else {
              dig_vertical_tunnel(&mut tiles, prev_y, new_y, prev_x);
              dig_horizontal_tunnel(&mut tiles, prev_x, new_x, new_y);
            }
          }
          rooms.push(room);
        }
        else {
          timeout -= 1;
        }
      }
      //println!("{}", timeout);
    }

    Map {
      tiles,
      revealed_tiles: vec![false; area],
      rooms,
      width,
      height,
    }
  }
}

impl BaseMap for Map {
  fn is_opaque(&self, idx: usize) -> bool {
    self.tiles[idx] == Tile::Wall
  }
}

impl Algorithm2D for Map {
  fn dimensions(&self) -> Point {
    Point::new(self.width, self.height)
  }
}

#[allow(dead_code)]
pub fn index_of(x: i32, y: i32) -> usize {
    ((y * 80) + x) as usize
}
#[allow(dead_code)]
pub fn coords_of(i: usize) -> (i32, i32) {
    ((i % 80) as i32, (i / 80) as i32)
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

fn dig_room(tiles: &mut Vec<Tile>, room: &Room) {
  for y in room.y1..=room.y2 {
    for x in room.x1..=room.x2 {
      tiles[index_of(x,y)] = Tile::Floor;
    }
  }
}

fn dig_horizontal_tunnel(tiles: &mut Vec<Tile>, x1: i32, x2: i32, y: i32) {
  for x in min(x1, x2)..=max(x1, x2) {
    let idx = index_of(x, y);
    if idx > 0 && idx < 80 * 50 {
      tiles[idx as usize] = Tile::Floor;
    }
  }
}

fn dig_vertical_tunnel(tiles: &mut Vec<Tile>, y1: i32, y2: i32, x: i32) {
  for y in min(y1, y2)..=max(y1, y2) {
    let idx = index_of(x, y);
    if idx > 0 && idx < 80 * 50 {
      tiles[idx as usize] = Tile::Floor;
    }
  }
}

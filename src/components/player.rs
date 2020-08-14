use std::cmp::{max, min};

use bracket_lib::prelude::BEvent;
use shipyard::{IntoIter, UniqueView, View, ViewMut};
use crate::components::*;
//use crate::event::*;
use crate::map::*;

pub struct Player;

/// returns whether or not its turn is over
pub fn player_event(
  event: BEvent,
  map: UniqueView<Map>,
  players: View<Player>,
  mut pos: ViewMut<Pos>,
  mut viewsheds: ViewMut<Viewshed>,
) -> bool {
  let mut turn_finished = false;
  if let BEvent::KeyboardInput {key, scan_code:_, pressed} = event {
    for (_,pos, vs) in (&players, &mut pos, &mut viewsheds).iter() {
      if pressed {
        use crate::VirtualKeyCode::*;
        //movement
        let (dx,dy) = match &key {
          K | Up => (0,-1),
          J | Down => (0,1),
          H | Left => (-1,0),
          L | Right => (1,0),
          Y => (-1,-1),
          U => (1,-1),
          B => (-1,1),
          N => (1,1),
          _ => (0,0),
        };
        let nx = min(79, max(0, pos.x+dx));
        let ny = min(49, max(0, pos.y+dy));

        let ix = index_of(nx,ny);
        if let Tile::Floor = map.tiles[ix] {
          pos.x = nx;
          pos.y = ny;
          vs.dirty = true;
          turn_finished = true;
        }
      }
    }
  }
  turn_finished
}

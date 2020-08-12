use std::cmp::{max, min};

use bracket_lib::prelude::BEvent;
use shipyard::{IntoIter, View, ViewMut};
use crate::components::*;
//use crate::event::*;

pub struct Player;

pub fn player_event(event: BEvent, players: View<Player>, mut pos: ViewMut<Pos>) {
  if let BEvent::KeyboardInput {key, scan_code:_, pressed} = event {
    for (_,pos) in (&players, &mut pos).iter() {
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
        pos.x = min(79, max(0, pos.x+dx));
        pos.y = min(49, max(0, pos.y+dy));
      }
    }
  }
}

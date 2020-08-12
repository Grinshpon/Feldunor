use bracket_lib::prelude::BEvent;
use shipyard::{IntoIter, View, ViewMut};
use crate::components::*;
//use crate::event::*;

pub struct Player;

pub fn player_system(event: BEvent, players: View<Player>, mut pos: ViewMut<Pos>) {
  if let BEvent::KeyboardInput {key, scan_code:_, pressed} = event {
    for (_,pos) in (&players, &mut pos).iter() {
      if pressed {
        use crate::VirtualKeyCode::*;
        match &key {
          K | Up => {pos.y -=1;},
          J | Down => {pos.y += 1;},
          H | Left => {pos.x -= 1;},
          L | Right => {pos.x += 1;},
          Y => {},
          U => {},
          B => {}
          N => {},
          _ => {},
        }
      }
    }
  }
}

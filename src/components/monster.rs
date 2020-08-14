use shipyard::{IntoIter, View};
use bracket_lib::prelude::Point;
use crate::components::*;

pub struct Monster;

pub fn monster_update(players: View<Player>, monsters: View<Monster>, pos: View<Pos>, viewsheds: View<Viewshed>) {
  let mut p = Point::new(0,0);
  for (_, pos) in (&players, &pos).iter() {
    p.x = pos.x;
    p.y = pos.y;
  }

  for (_,_pos,vs) in (&monsters, &pos, &viewsheds).iter() {
    if vs.visible_tiles.contains(&p) {
      println!("Monster ponders his own existence");
    }
  }
}

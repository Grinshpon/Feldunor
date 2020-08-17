use shipyard::{IntoIter, UniqueViewMut, View, ViewMut};
use bracket_lib::prelude::{a_star_search, DistanceAlg, Point};
use crate::components::*;
use crate::map::Map;

pub struct Monster;

pub fn monster_update(
  mut map: UniqueViewMut<Map>,
  players: View<Player>,
  monsters: View<Monster>,
  mut pos: ViewMut<Pos>,
  mut viewsheds: ViewMut<Viewshed>,
  names: View<Name>,
  blocks: View<BlockTile>,
) {
  let mut p = Point::new(0,0);
  for (_, pos) in (&players, &pos).iter() {
    p.x = pos.x;
    p.y = pos.y;
  }

  for (_,_,name,pos,vs) in (&monsters, &blocks, &names, &mut pos, &mut viewsheds).iter() {
    if vs.visible_tiles.contains(&p) {
      let dist = DistanceAlg::Pythagoras.distance2d(Point::new(pos.x, pos.y), p);
      if dist < 1.5 {
        println!("{} attacks",&name.0);
      }
      let path = a_star_search(
        map.index_of(pos.x,pos.y) as i32,
        map.index_of(p.x,p.y) as i32,
        &mut *map,
      );
      if path.success && path.steps.len() > 2 {
        let (nx,ny) = map.coords_of(path.steps[1]);
        //println!("{} moves from ({},{}) to ({},{})", &name.0,pos.x,pos.y,nx,ny);
        let mut ix = map.index_of(pos.x,pos.y);
        map.blocked_tiles[ix] = false;

        pos.x = nx;
        pos.y = ny;

        ix = map.index_of(pos.x,pos.y);
        map.blocked_tiles[ix] = true;
        vs.dirty = true;
      }
    }
  }
}

use crate::components::*;
use crate::map::{Map};
use bracket_lib::prelude::{field_of_view, Point};
use shipyard::{IntoIter, UniqueViewMut, View, ViewMut};

pub struct Viewshed {
  pub dirty: bool,
  pub visible_tiles: Vec<Point>,
  pub range: u32,
}
impl Viewshed {
  pub fn new(range: u32) -> Self {
    Viewshed {
      dirty: true,
      visible_tiles: Vec::new(),
      range,
    }
  }
}

pub fn visibility(mut map: UniqueViewMut<Map>, players: View<Player>, pos: View<Pos>, mut viewsheds: ViewMut<Viewshed>) {
  for (viewshed, pos) in (&mut viewsheds, &pos).iter() {
    if viewshed.dirty {
      viewshed.dirty = false;
      viewshed.visible_tiles.clear();
      viewshed.visible_tiles = field_of_view(
        Point::new(pos.x, pos.y),
        viewshed.range as i32,
        &*map
      );
      viewshed.visible_tiles.retain(|p| {
        p.x >= 0 && p.x < map.width as i32 - 1 && p.y >= 0 && p.y < map.height as i32 - 1
      });
    }
  }
  for (_, viewshed) in (&players, &mut viewsheds).iter() {
    for p in &viewshed.visible_tiles {
      let ix = map.index_of(p.x,p.y);
      map.revealed_tiles[ix] = true;
    }
  }
}

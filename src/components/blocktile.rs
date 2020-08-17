use shipyard::{IntoIter, UniqueViewMut, View};
use crate::components::*;
use crate::map::*;

pub struct BlockTile;

pub fn map_index(mut map: UniqueViewMut<Map>, pos: View<Pos>, blocks: View<BlockTile>) {
  map.populate_blocked();
  for (pos, _) in (&pos, &blocks).iter() {
    let ix = map.index_of(pos.x,pos.y);
    map.blocked_tiles[ix] = true;
  }
}

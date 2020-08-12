use bracket_lib::prelude::*;
use shipyard::{AllStoragesViewMut, EntityId, EntitiesViewMut, ViewMut};
use std::any::Any;
use crate::state::{AppData, SEvent, State};
//use crate::event::*;
use crate::components::*;
use crate::context::*;

pub struct Game {
  pub entities: Vec<EntityId>,
}
impl Game {
  pub fn new() -> Self {
    Game {entities: Vec::new()}
  }
}

macro_rules! add_entity {
  ( $s:ident, $ents:ident, $q:expr, $e:expr, $(,)? ) => {
    let id = $ents.add_entity(
      $q,
      $e,
    );
    $s.entities.push(id);
  }
}

impl State for Game {
  any!();
  type Event = BEvent;
  fn load(&mut self, data: &mut AppData) {
  }
  fn unload(&mut self, data: &mut AppData) {
  }
  fn event(&mut self, data: &mut AppData, event: BEvent) -> SEvent<BEvent> {
    SEvent::Cont
  }
}

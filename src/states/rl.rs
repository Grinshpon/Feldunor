use bracket_lib::prelude::*;
use shipyard::{AllStoragesViewMut, EntityId, EntitiesViewMut, ViewMut};
use std::any::Any;
use crate::state::{AppData, SEvent, State};
use crate::components::*;

pub struct RL {
  pub entities: Vec<EntityId>,
}
impl RL {
  pub fn new() -> Self {
    RL {entities: Vec::new()}
  }
}

impl State for RL {
  any!();
  type Event = BEvent;
  fn load(&mut self, data: &mut AppData) {
    data.world.run(|mut entities: EntitiesViewMut, mut players: ViewMut<Player>, mut pos: ViewMut<Pos>| {
      add_entity!(self,entities,
        (&mut players, &mut pos),
        (Player, Pos { x: 40, y: 20 }),
      );
    });
  }
  fn unload(&mut self, data: &mut AppData) {
    data.world.run(|mut storages: AllStoragesViewMut| {
      for id in self.entities.iter() {
        storages.delete(*id);
      }
    });
    self.entities.clear();
  }
  fn event(&mut self, data: &mut AppData, event: BEvent) -> SEvent<BEvent> {
    data.world.run_with_data(player_system, event);
    SEvent::Cont
  }
}

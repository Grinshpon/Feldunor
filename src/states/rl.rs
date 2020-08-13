use bracket_lib::prelude::*;
use shipyard::{AllStoragesViewMut, EntityId, EntitiesViewMut, ViewMut};
use std::any::Any;
use crate::state::{AppData, SEvent, State};
use crate::components::*;
use crate::map::*;

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
    data.world.run(|mut entities: EntitiesViewMut, mut players: ViewMut<Player>, mut stats: ViewMut<Stat>, mut pos: ViewMut<Pos>| {
      add_entity!(self,entities,
        (&mut players, &mut stats, &mut pos),
        (Player, Stat::default(), Pos { x: 40, y: 20 }),
      );
    });
    data.world.add_unique(new_map());
  }
  fn unload(&mut self, data: &mut AppData) {
    data.world.run(|mut storages: AllStoragesViewMut| {
      for id in self.entities.iter() {
        storages.delete(*id);
      }
    });
    self.entities.clear();
    data.world.remove_unique::<Map>();
  }
  fn event(&mut self, data: &mut AppData, event: BEvent) -> SEvent<BEvent> {
    data.world.run_with_data(player_event, event);
    SEvent::Cont
  }
}

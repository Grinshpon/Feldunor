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
    let map = Map::new(80,50);
    let start = map.rooms[0].center();
    data.world.add_unique(map);
    data.world.run(|mut entities: EntitiesViewMut, mut players: ViewMut<Player>, mut stats: ViewMut<Stat>, mut pos: ViewMut<Pos>, mut viewsheds: ViewMut<Viewshed>| {
      add_entity!(self,entities,
        (&mut players, &mut stats, &mut pos, &mut viewsheds),
        (Player, Stat::default(), Pos { x: start[0], y: start[1] }, Viewshed::new(12)),
      );
    });
  }
  fn unload(&mut self, data: &mut AppData) {
    data.world.remove_unique::<Map>();
    data.world.run(|mut storages: AllStoragesViewMut| {
      for id in self.entities.iter() {
        storages.delete(*id);
      }
    });
    self.entities.clear();
  }
  fn update(&mut self, data: &mut AppData) -> SEvent<BEvent> {
    data.world.run(visibility);
    SEvent::Cont
  }
  fn event(&mut self, data: &mut AppData, event: BEvent) -> SEvent<BEvent> {
    data.world.run_with_data(player_event, event);
    SEvent::Cont
  }
}

use bracket_lib::prelude::*;
use shipyard::{AllStoragesViewMut, EntityId, EntitiesViewMut, UniqueView, ViewMut};
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
    data.world.add_unique(Map::new(80,50));
    data.world.run_with_data(initial_entities,self);
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

fn initial_entities(
  state: &mut RL,
  mut entities: EntitiesViewMut,
  map: UniqueView<Map>,
  mut players: ViewMut<Player>,
  mut stats: ViewMut<Stat>,
  mut pos: ViewMut<Pos>,
  mut viewsheds: ViewMut<Viewshed>,
  mut monsters: ViewMut<Monster>,
  mut renders: ViewMut<Render>,
) {
  let start = map.rooms[0].center();
  add_entity!(state,entities,
    (&mut players, &mut stats, &mut pos, &mut viewsheds, &mut renders),
    (Player, Stat::default(), Pos { x: start[0], y: start[1] }, Viewshed::new(12), Render::player()),
  );
  for room in map.rooms.iter().skip(1) {
    let [x,y] = room.center();
    add_entity!(state,entities,
      (&mut monsters, &mut pos, &mut viewsheds, &mut renders),
      (Monster, Pos {x,y}, Viewshed::new(12), Render::goblin()),
    );
  }
}

use bracket_lib::prelude::*;
use shipyard::{AllStoragesViewMut, EntityId, EntitiesViewMut, ViewMut};
use std::any::Any;
use crate::state::{AppData, State};
use crate::components::*;
use crate::context::*;

pub struct MainMenu {
  pub entities: Vec<EntityId>,
}
impl MainMenu {
  pub fn new() -> Self {
    MainMenu {entities: Vec::with_capacity(4)}
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

impl State for MainMenu {
  any!();
  type Event = InputEvent;
  fn load(&mut self, data: &mut AppData) {
    data.world.run(|mut entities: EntitiesViewMut, mut menus: ViewMut<Menu>, mut titles: ViewMut<Title>, mut texts: ViewMut<String>| {
      add_entity!(self,entities,
        (&mut titles, &mut texts),
        (Title, String::from("Feldunor")),
      );
      let menu = Menu {
        options: vec![MenuOption::Start, MenuOption::Options],
        selected: 0,
      };
      add_entity!(self,entities,
        &mut menus,
        menu,
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
}

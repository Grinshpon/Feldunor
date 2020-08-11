use shipyard::{EntityId, EntitiesViewMut, ViewMut};
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

impl State for MainMenu {
  any!();
  type Event = InputEvent;
  fn load(&mut self, data: &mut AppData) {
    data.world.run(|mut entities: EntitiesViewMut, mut uis: ViewMut<UI>, mut titles: ViewMut<Title>, mut texts: ViewMut<Text>| {
      let id = entities.add_entity(
        (&mut uis,&mut titles, &mut texts),
        (UI, Title, Text(String::from("Feldunor"))),
      );
      self.entities.push(id);
    });
  }
  fn unload(&mut self, _data: &mut AppData) {
    println!("main menu unloading");
  }
}

use bracket_lib::prelude::*;

#[macro_use]
mod state;
//mod event;
//mod input_map;
mod components;
mod context;
mod states;
mod map;

use crate::state::{StateManager};
//use crate::event::*;
//use crate::components::*;
use crate::context::*;
use crate::states::{MainMenu};

fn main () -> BError {
  let mut game = Game(StateManager::new()); //with_capacity(4)
  game.0.push(Box::new(MainMenu::new()));

  let context = BTermBuilder::simple80x50()
    .with_title("Feldunor")
    .with_advanced_input(true)
    .build()?;

  main_loop(context,game)
}

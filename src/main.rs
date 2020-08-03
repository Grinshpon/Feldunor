use std::any::Any;

mod state;
use crate::state::{State, StateManager, StateObject, SAction};
mod inputmap;

macro_rules! any {
  () => {
    fn as_any(&self) -> &dyn Any { self }
  };
}

struct MyState;
struct MyOtherState;
struct SelfDestructState;

impl State for MyState {
  any!();// necessary boilerplate
  fn load(&mut self) {
    println!("state pushed");
  }
  fn update(&mut self) -> SAction {
    println!("tick");
    SAction::None
  }
}

impl State for MyOtherState {
  //fn as_any(&self) -> &dyn Any { self }
  any!();
  fn load(&mut self) {
    println!("other state pushed");
  }
  fn update(&mut self) -> SAction {
    println!("tock");
    SAction::None
  }
}

impl State for SelfDestructState {
  any!();
  fn update(&mut self) -> SAction {
    println!("self destruct");
    SAction::Pop
  }
}

fn print_state_type(state: &StateObject) {
  let state_any = state.as_any();

  if let Some(_) =  state_any.downcast_ref::<MyOtherState>() {
    println!("MyOtherState");
  }
  else if let Some(_) = state_any.downcast_ref::<MyState>() {
    println!("MyState");
  }
}

fn main() {
  let mut manager = StateManager::with_capacity(2);
  manager.push(Box::new(MyState));
  manager.push(Box::new(MyOtherState));
  manager.update();
  let s = manager.pop().unwrap();
  print_state_type(&s);
  manager.update();
  manager.push(s);
  manager.push(Box::new(SelfDestructState));
  println!("{}",manager.stack_len());
  manager.update();
  println!("{}, {}",manager.quit(), manager.stack_len());
}

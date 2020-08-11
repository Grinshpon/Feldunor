use std::any::Any;

mod state;
use crate::state::{State, StateManager, StateObject, SEvent, AppData};
mod event;
mod inputmap;

struct MyState;
struct MyOtherState;
struct SelfDestructState;

impl State for MyState {
  any!();// necessary boilerplate
  type Event = ();
  fn load(&mut self) {
    println!("state pushed");
  }
  fn update(&mut self, _data: &mut AppData) -> SEvent<Self::Event> {
    println!("tick");
    SEvent::Cont
  }
}

impl State for MyOtherState {
  //fn as_any(&self) -> &dyn Any { self }
  any!();
  type Event = ();
  fn load(&mut self) {
    println!("other state pushed");
  }
  fn update(&mut self, _data: &mut AppData) -> SEvent<Self::Event> {
    println!("tock");
    SEvent::Cont
  }
}

impl State for SelfDestructState {
  any!();
  type Event = ();
  fn update(&mut self, _data: &mut AppData) -> SEvent<Self::Event> {
    println!("self destruct");
    SEvent::Pop
  }
}

fn print_state_type<T>(state: &StateObject<T>) {
  let state_any = state.as_any();

  if let Some(_) = state_any.downcast_ref::<MyOtherState>() {
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

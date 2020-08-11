use std::any::Any;

use shipyard::{World};

#[macro_export]
macro_rules! any {
  () => {
    fn as_any(&self) -> &dyn Any { self }
  };
}

/// The state manager is modeled after Amethyst's state machine

// State Event (what to do after handling input)
#[allow(dead_code)]
pub enum SEvent<T> {
  Cont, // do nothing and continue
  Push(StateObject<T>),
  Pop,
  Switch(StateObject<T>),
  Swap, // swap last and second-to-last states
  Quit,
}

pub struct AppData {
  //ctx_modified: bool, //private;if window settings (like size) or some other stuff is changed, this will prompt a refresh
  pub world: World,
}

pub trait State {
  type Event;
  fn as_any(&self) -> &dyn Any;
  fn load(&mut self, _data: &mut AppData) {}
  fn update(&mut self, _data: &mut AppData) -> SEvent<Self::Event> {SEvent::Cont}
  // fn event(&mut self, event: Event) -> SEvent {SEvent::Cont}
  // fn unhandled_event(&mut self, event: Event) -> SEvent {SEvent::Cont}
  fn unload(&mut self, _data: &mut AppData) {}
  fn quit(&mut self, _is_current_scene: bool) -> bool {true}
}

pub type StateObject<T> = Box<dyn State<Event = T>>;

pub struct StateManager<T> {
  stack: Vec<StateObject<T>>,
  data: AppData,
  //context/config,
}

#[allow(dead_code)]
impl <T> StateManager <T> {
  pub fn new() -> Self {
    StateManager {
      stack: Vec::new(),
      data: AppData {world: World::new()},
    }
  }
  pub fn with_capacity(capacity: usize) -> Self {
    StateManager {
      stack: Vec::with_capacity(capacity),
      data: AppData {world: World::new()},
    }
  }
  pub fn push(&mut self, mut state: StateObject<T>) {
    state.load(&mut self.data);
    self.stack.push(state);
  }
  pub fn pop(&mut self) -> Option<StateObject<T>> {
    match self.stack.pop() {
      Some(mut popped) => { popped.unload(&mut self.data); Some(popped) },
      None => None,
    }
  }
  pub fn switch(&mut self, state: StateObject<T>) -> Option<StateObject<T>> {
    let popped = self.pop();
    self.push(state);
    popped
  }
  pub fn swap(&mut self) {
    let ix = self.stack.len()-1;
    self.stack.swap(ix,ix-1);
  }
  pub fn stack_len(&self) -> usize {
    self.stack.len()
  }
  fn handle_sevent(&mut self, sevent: SEvent<T>) {
    match sevent {
      SEvent::Cont => {},
      SEvent::Push(state) => self.push(state),
      SEvent::Pop => { self.pop(); ()},
      SEvent::Switch(state) => { self.switch(state); () },
      SEvent::Swap => self.swap(),
      SEvent::Quit => (), //todo
    };
  }
  pub fn update(&mut self) {
    let len = self.stack.len();
    if len > 0 {
      let ix = len-1;
      let sevent = self.stack[ix].update(&mut self.data);
      self.handle_sevent(sevent);
    }
  }
  pub fn quit(&mut self /*, force_quit: bool */) -> bool {
    let len = self.stack.len();
    let mut can_quit = true;
    for i in 0..len {
      can_quit = can_quit && self.stack[i].quit(i == len-1);
    }
    can_quit
  }
  pub fn unload(&mut self) {
    for s in self.stack.iter_mut() {
      s.unload(&mut self.data);
    }
  }
  //pub fn run(&mut self) -> Error // main run loop
}

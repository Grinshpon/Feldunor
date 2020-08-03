use std::any::Any;

/// The state manager is modeled after Amethyst's state machine

// State Action (what to do after handling input)
pub enum SAction {
  None,
  Push(StateObject),
  Pop,
  Switch(StateObject),
  Swap, // swap last and second-to-last states
  Quit,
}

pub trait State {
  fn as_any(&self) -> &dyn Any;
  fn load(&mut self) {}
  fn update(&mut self) -> SAction {SAction::None}
  // fn action(&mut self, action: Action) -> SAction {SAction::None}
  // fn unhandled_action(&mut self, action: Action) -> SAction {SAction::None}
  fn unload(&mut self) {}
  fn quit(&mut self, _is_current_scene: bool) -> bool {true}
}

pub type StateObject = Box<dyn State>;

pub struct StateManager {
  stack: Vec<StateObject>,
}

impl StateManager {
  pub fn new() -> Self {
    StateManager { stack: Vec::new() }
  }
  pub fn with_capacity(capacity: usize) -> Self {
    StateManager { stack: Vec::with_capacity(capacity) }
  }
  pub fn push(&mut self, mut state: StateObject) {
    state.load();
    self.stack.push(state);
  }
  pub fn pop(&mut self) -> Option<StateObject> {
    match self.stack.pop() {
      Some(mut popped) => { popped.unload(); Some(popped) },
      None => None,
    }
  }
  pub fn switch(&mut self, state: StateObject) -> Option<StateObject> {
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
  fn handle_saction(&mut self, saction: SAction) {
    match saction {
      SAction::None => {},
      SAction::Push(state) => self.push(state),
      SAction::Pop => { self.pop(); ()},
      SAction::Switch(state) => { self.switch(state); () },
      SAction::Swap => self.swap(),
      SAction::Quit => (), //todo
    };
  }
  pub fn update(&mut self) {
    let ix = self.stack.len()-1;
    if ix >= 0 {
      let saction = self.stack[ix].update();
      self.handle_saction(saction);
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
}

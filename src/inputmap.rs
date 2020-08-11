use crate::event::Event;

pub trait InputMap {
  type Input;
  fn input_to_event(&self, input: Self::Input) -> Event; 
}

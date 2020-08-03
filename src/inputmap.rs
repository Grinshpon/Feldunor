struct Action; //placeholder, eventually Action will be an enum with every possible action

pub trait InputMap {
  type Input;
  fn input_to_action(&self, input: Self::Input) -> Action; 
}

use bracket_lib::prelude::*;
use shipyard::{View,IntoIter};
use crate::state::{StateManager};
use crate::components::*;

pub type InputEvent = VirtualKeyCode;

pub struct Game(pub StateManager<InputEvent>);
impl GameState for Game {
  fn tick(&mut self, ctx: &mut BTerm) {
    self.0.update();
    let mut input = INPUT.lock();
    input.for_each_message(|event: BEvent| {
      if let BEvent::CloseRequested = event {
        if self.0.quit() {
          self.0.unload();
          ctx.quitting = true;
        }
      }
    });
  }
}

fn render_title(ctx: &mut BTerm, uis: View<UI>, titles: View<Title>, texts: View<Text>) {
  for (_,_,txt) in (&uis,&titles,&texts).iter() {
    ctx.print(1,1,&txt.0);
  }
}

use bracket_lib::prelude::*;
use shipyard::{IntoIter, View};
use crate::state::{StateManager};
use crate::components::*;
use crate::states::*;

pub type InputEvent = BEvent;

pub struct Game(pub StateManager<InputEvent>);
impl GameState for Game {
  fn tick(&mut self, ctx: &mut BTerm) {
    ctx.cls();
    self.0.update();
    let mut input = INPUT.lock();
    input.for_each_message(|event: BEvent| {
      if let BEvent::CloseRequested = event {
        if self.0.quit() {
          self.0.unload();
          ctx.quitting = true;
        }
      }
      else {
        self.0.event(event);
      }
    });
    if self.0.stack_len() == 0 {
      ctx.quitting = true;
    }
    else {
      let state_any = self.0.peek().unwrap().as_any();
      if let Some(_) = state_any.downcast_ref::<MainMenu>() {
        self.0.data.world.run_with_data(render_title, ctx);
      }
    }
  }
}

fn render_title(ctx: &mut BTerm, uis: View<UI>, titles: View<Title>, texts: View<Text>) {
  for (_,_,txt) in (&uis,&titles,&texts).iter() {
    ctx.print(1,1,&txt.0);
  }
}

use bracket_lib::prelude::*;
use shipyard::{IntoIter, View};
//use crate::event::*;
use crate::state::{StateManager};
use crate::components::*;
use crate::states::*;

pub struct Game(pub StateManager<BEvent>);
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
        self.0.data.world.run_with_data(render_options, ctx);
      }
      else if let Some(_) = state_any.downcast_ref::<RL>() {
        self.0.data.world.run_with_data(render_game, ctx);
      }
    }
  }
}

fn render_title(ctx: &mut BTerm, titles: View<Title>, texts: View<String>) {
  for (_,txt) in (&titles,&texts).iter() {
    ctx.print(1,1,&txt);
  }
}

fn render_options(ctx: &mut BTerm, menus: View<Menu>) {
  let mut line = 5;
  for menu in (&menus).iter() {
    for (i,opt) in menu.options.iter().enumerate() {
      let option = {if menu.selected == i {format!("> {:?}", opt)} else {format!("  {:?}", opt)}};
      ctx.print(1,line,option);
      line += 2;
    }
  }
}

fn render_game(ctx: &mut BTerm, players: View<Player>, pos: View<Pos>) {
  for (_,pos) in (&players, &pos).iter() {
    ctx.set(pos.x,pos.y,RGB::from_f32(1.0, 1.0, 1.0), RGB::from_f32(0., 0., 0.), to_cp437('@'));
  }
}

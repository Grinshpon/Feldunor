use bracket_lib::prelude::*;
use shipyard::{IntoIter, UniqueView, View};
//use crate::event::*;
use crate::state::{StateManager};
use crate::components::*;
use crate::states::*;
use crate::map;
use crate::map::Map;

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
        self.0.data.world.run_with_data(render_map, ctx);
        self.0.data.world.run_with_data(render_player, ctx);
        self.0.data.world.run_with_data(render_hud, ctx);
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

fn render_player(ctx: &mut BTerm, players: View<Player>, pos: View<Pos>) {
  for (_,pos) in (&players, &pos).iter() {
    ctx.set(pos.x,pos.y,RGB::from_f32(1.0, 1.0, 1.0), RGB::from_f32(0., 0., 0.), to_cp437('@'));
  }
}

fn render_hud(ctx: &mut BTerm, players: View<Player>, stats: View<Stat>) {
  for (_,stat) in (&players, &stats).iter() {
    ctx.print(0,0,format!(
      "HP:{}/{} STR:{} AGL:{} VIT:{}  LVL:{} XP:{}/{}",
      stat.hp,
      stat.max_hp,
      stat.strength,
      stat.agility,
      stat.vitality,
      stat.level,
      stat.xp,
      stat.req_xp
    ));
  }
}

fn render_map(ctx: &mut BTerm, map: UniqueView<Map>) {
  for (i,tile) in map.iter().enumerate() {
    let (x,y) = map::coords_of(i);

    match tile {
      map::Tile::Floor => {
        ctx.set(x, y, RGB::from_f32(0.5, 0.5, 0.5), RGB::from_f32(0., 0., 0.), to_cp437('.'));
      },
      map::Tile::Wall => {
        ctx.set(x, y, RGB::from_f32(0., 1., 0.), RGB::from_f32(0., 0., 0.), to_cp437('#'));
      },
    }
  }
}

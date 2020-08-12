use bracket_lib::prelude::*;
use shipyard::{IntoIter, ViewMut};
//use crate::event::*;

#[derive(Debug,Clone,Copy)]
pub enum MenuOption {
  Start,
//  Options,
  Quit,
}
/*pub enum OptionType {
  Button,
  Checkbox,
  Slider(usize,usize),
  Dropdown(Vec<usize>),
}*/
pub struct Menu {
  pub options: Vec<MenuOption>,
  pub selected: usize,
}

pub fn menu_system(event: BEvent, mut menus: ViewMut<Menu>) -> Option<MenuOption> {
  if let BEvent::KeyboardInput {key, scan_code:_, pressed} = event {
    for menu in (&mut menus).iter() {
      if pressed {
        use crate::VirtualKeyCode::*;
        match &key {
          K | Up => {if menu.selected > 0 {menu.selected -= 1;}},
          J | Down => {if menu.selected < menu.options.len()-1 {menu.selected += 1;}},
          Space | Return => {return Some(menu.options[menu.selected]);},
          _ => {},
        }
      }
    }
  }
  None
}

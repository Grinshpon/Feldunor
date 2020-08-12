use bracket_lib::prelude::BEvent;
use shipyard::{IntoIter, View, ViewMut};
use crate::components::*;
//use crate::event::*;

pub struct Player;

pub fn player_system(event: &mut BEvent, players: View<Player>, pos: ViewMut<Pos>) {

}

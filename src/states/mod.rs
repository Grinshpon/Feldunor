#[macro_export]
macro_rules! add_entity {
  ( $s:ident, $ents:ident, $q:expr, $e:expr, $(,)? ) => {
    let id = $ents.add_entity(
      $q,
      $e,
    );
    $s.entities.push(id);
  }
}

mod main_menu;
pub use main_menu::*;

mod game;
pub use game::*;

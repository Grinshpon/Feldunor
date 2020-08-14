use bracket_lib::prelude::*;

pub struct Render {
  pub glyph: FontCharType,
  pub fg: RGB,
  pub bg: RGB,
}

impl Render {
  pub fn player() -> Self {
    Render {
      glyph: to_cp437('@'),
      fg: RGB::named(WHITE),
      bg: RGB::named(BLACK),
    }
  }
  pub fn goblin() -> Self {
    Render {
      glyph: to_cp437('g'),
      fg: RGB::named(RED),
      bg: RGB::named(BLACK),
    }
  }
}

pub struct Stat {
  pub max_hp: u32,
  pub hp: u32,

  pub strength: u32, //str determines weapon damage mod and using
  pub agility: u32, //agl determines evade and % to hit (and movement/action speed)
  pub vitality: u32, //vit determines health and resistance
  //pub intelligence: u32, //int determines identification and magic use

  pub level: u32,
  pub xp: usize,
  pub req_xp: usize,
}

impl Stat {
  pub fn default() -> Self {
    let (strength,agility,vitality) = (5,5,5);
    let max_hp = strength/2 + agility/2 + vitality*2;
    let hp = max_hp;
    Stat { max_hp, hp, strength, agility, vitality, level: 1, xp: 0, req_xp: BASE_XP }
  }
}

static BASE_XP: usize = 100;

#[allow(dead_code)]
pub fn next_level(prev: usize) -> usize {
  prev + prev/2
}

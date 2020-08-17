pub struct Stat {
  pub max_hp: u32,
  pub hp: u32,

  pub strength: u32, //str determines weapon damage mod and using
  pub dexterity: u32, //dex determines % to hit and ranged combat (and lockpicking in future)
  pub agility: u32, //agl determines evade and % to hit (and movement/action speed)
  pub vitality: u32, //vit determines health and resistance
  //pub intelligence: u32, //int determines identification and magic use

  pub level: u32,
  pub xp: usize,
  pub req_xp: usize,
}

impl Stat {
  pub fn default() -> Self {
    Self::new(5,5,5,5)
  }
  pub fn new(strength: u32, dexterity: u32, agility: u32, vitality: u32) -> Self {
    let max_hp = calc_hp(strength,dexterity,agility,vitality);
    let hp = max_hp;
    Stat { max_hp, hp, strength, dexterity, agility, vitality, level: 1, xp: 0, req_xp: BASE_XP }
  }
}

static BASE_XP: usize = 100;

#[allow(dead_code)]
pub fn next_level(prev: usize) -> usize {
  prev + prev/2
}

fn calc_hp(strength: u32, dexterity: u32, agility: u32, vitality: u32) -> u32 {
    strength/2 + dexterity/2 + agility/2 + vitality*2
}

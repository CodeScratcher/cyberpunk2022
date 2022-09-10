#[derive(Clone)]
pub enum Sprite {
    PlayerSprite
}

pub enum Event {
    
}

pub type GameState<'a> = Vec<&'a dyn Entity>;
pub trait Entity {
    fn get_sprite(&self) -> Sprite;
    fn take_turn(&mut self, game_state: &mut GameState, events: Vec<Event>);
    fn take_damage(&mut self, dmg: u32);
    fn get_coords(&self) -> (i64, i64);
} 


pub type Spell<'a> = &'a dyn Fn(&mut GameState<'a>);
pub struct Player<'a> {
    pub x: i64,
    pub y: i64,
    pub hp: u32,
    pub spr: Sprite,
    pub spells: [Option<Spell<'a>>;5]
}

impl<'a> Entity for Player<'a> {
    fn get_sprite(&self) -> Sprite {
        self.spr.clone()
    }
    fn take_turn(&mut self, game_state: &mut GameState, events: Vec<Event>) {
        
    }
    fn take_damage(&mut self, dmg: u32) {
        self.hp -= dmg;
    }
    fn get_coords(&self) -> (i64, i64) {
        (self.x, self.y)
    }
}

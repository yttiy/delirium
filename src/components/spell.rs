use engine::Component;

pub struct Spell{
    pub damage: f32,
    pub is_player_cast: bool,
    pub interval: f32,
    pub current_time: f32
}

impl Spell{
    pub fn new(damage: f32, is_player_cast: bool, interval: f32) -> Spell{
        Spell{
            damage: damage,
            is_player_cast: is_player_cast,
            interval: interval,
            current_time: 0.0
        }
    }
}

impl Component for Spell{}
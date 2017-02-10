use engine::Component;

pub struct Weapon{
    pub attack_interval: f32,
    pub current_time: f32,
    pub dx: f32,
    pub dy: f32,
    pub offset_x: f32,
    pub offset_y: f32,
    pub sound: i32,
    pub activated: bool,
    pub spell_type: i32,
}

impl Weapon{
    pub fn new(attack_interval: f32, dx: f32, dy: f32, sound: i32, spell_type: i32, activated: bool) -> Weapon{
        Weapon{
            attack_interval: attack_interval,
            current_time: 0.0,
            dx: dx,
            dy: dy,
            offset_x: 0.0,
            offset_y: 0.0,
            sound: sound,
            activated: activated,
            spell_type: spell_type,
        }
    }
}

impl Component for Weapon{}
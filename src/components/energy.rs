use engine::Component;

pub struct Energy{
    health: f32,
    max_health: f32,
    mana: f32,
    max_mana: f32,
}

impl Energy{
    pub fn new(max_health: f32, max_mana: f32) -> Energy{
        Energy{
            health: max_health,
            max_health: max_health,
            mana: max_mana,
            max_mana: max_mana,
        }
    }

    pub fn get_health(&self) -> f32 {
        self.health
    }

    pub fn get_mana(&self) -> f32 {
        self.mana
    }

    pub fn get_max_health(&self) -> f32 {
        self.max_health
    }

    pub fn get_max_mana(&self) -> f32 {
        self.max_mana
    }

    pub fn set_health(&mut self, h: f32) {
        self.health = h;
    }

    pub fn set_mana(&mut self, m: f32) {
        self.mana = m;
    }

    pub fn reduce_health(&mut self, h: f32){
        self.health -= h;
    }

    pub fn reduce_mana(&mut self, m: f32){
        self.mana -= m;
    }

    pub fn add_health(&mut self, h: f32){
        self.health += h;
        if self.health > self.max_health {
            self.health = self.max_health;
        }
    }

    pub fn add_mana(&mut self, m: f32){
        self.mana += m;
        if self.mana > self.max_mana {
            self.mana = self.max_mana;
        }
    }
}

impl Component for Energy{}
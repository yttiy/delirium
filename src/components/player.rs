use engine::Component;

pub struct Player{
    pub vector: i32
}

impl Player{
    pub fn new() -> Player{
        Player{
			vector: 0
		}
    }
}

impl Component for Player{}
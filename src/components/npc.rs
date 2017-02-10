use engine::Component;

pub struct NPC{
    pub start_x: f32,
    pub vector: i32,
    pub distance: f32,
    pub ash: i32
}

impl NPC{
    pub fn new(start_x: f32, distance: f32, ash: i32) -> NPC{
        NPC{
            start_x: start_x,
            vector: 1,
            distance: distance,
            ash: ash
        }
    }
}

impl Component for NPC{}
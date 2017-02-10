use engine::Engine;

pub trait Screen{
    fn init(&mut self, engine: &mut Engine){}
    fn update(&mut self, engine: &mut Engine, dt: f32){}
}

use engine::{ Engine, Entity, Aspect, EntityManager, MessageManager };
use engine::world::SystemDataTest;

pub trait System{
    fn init(&mut self, mm: &mut MessageManager, em: &mut EntityManager, engine: &mut Engine){}
    fn aspect(&self) -> Aspect;
    fn entity_changed(&mut self, e: &Vec<i32>, em: &mut EntityManager, engine: &mut Engine){}
    fn process(&mut self, id: &i32, em: &mut EntityManager, engine: &mut Engine, dt: f32){}
    fn process_m(&mut self, id: &i32, mm: &mut MessageManager, em: &mut EntityManager, engine: &mut Engine, dt: f32){}
    fn process_one(&mut self, em: &mut EntityManager, mm: &mut MessageManager, engine: &mut Engine, dt: f32){}
    fn receive(&mut self, mm: &MessageManager, em: &mut EntityManager, engine: &mut Engine, dt: f32){}
}

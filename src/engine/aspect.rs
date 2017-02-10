use std::any::{ Any, TypeId };
use engine::Component;

pub struct Aspect{
    r_types: Vec<TypeId>
}

impl Aspect{
    pub fn new() -> Aspect{
        Aspect{
            r_types: Vec::new()
        }
    }

    pub fn add<T: Any + Component>(mut self) -> Aspect{
        self.r_types.push(TypeId::of::<T>());
        self
    }

    pub fn get(&self) -> &Vec<TypeId>{
        &self.r_types
    }
}
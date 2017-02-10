use engine::{ Engine, Entity, Component, System, Aspect};
use std::any::{ Any, TypeId };
use std::collections::HashMap;

pub struct EntityManager{
    entities: HashMap<i32, Entity>,
    added_id: Vec<i32>,
    removed_id: Vec<i32>,
    entities_id: i32,
}

impl EntityManager{
    pub fn new() -> EntityManager{
        EntityManager{
            entities: HashMap::new(),
            added_id: Vec::new(),
            removed_id: Vec::new(),
            entities_id: 0
        }
    }

    pub fn create_entity(&mut self) -> &mut Entity{
        let mut e = Entity::new(self.entities_id.clone());
        self.add(e);

        let n = self.entities_id.clone() - 1;
        self.get_by_id(&n)
    }

    pub fn add(&mut self, e: Entity){
        self.entities.insert(self.entities_id.clone(), e);
        self.added_id.push(self.entities_id.clone());
        self.entities_id +=  1;
    }

    pub fn update(&mut self){
        for id in self.removed_id.iter(){
            self.entities.remove(id);
        }
        self.removed_id.clear();
        self.added_id.clear();
    }

    pub fn get_added(&self) -> &Vec<i32>{
        &self.added_id
    }

    pub fn get_removed(&self) -> &Vec<i32>{
        &self.removed_id
    }

    pub fn get_entities(&mut self) -> &mut HashMap<i32, Entity>{
        &mut self.entities
    }

    pub fn remove(&mut self, id: &i32){
        self.removed_id.push(*id);
    }

    pub fn get_by_id(&mut self, id: &i32) -> &mut Entity{
        self.entities.get_mut(id).unwrap()
    }
}
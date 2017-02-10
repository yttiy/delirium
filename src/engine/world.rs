use engine::{ Engine, Entity, Component, System, Aspect, EntityManager, MessageManager };
use std::any::{ Any, TypeId };
use std::collections::HashMap;

pub struct SystemDataTest<'a>{
    s: &'a mut SystemData
}

struct SystemData{
    systems: Vec<Box<System>>,
    aspects: Vec<Aspect>,
    entities_id: Vec<Vec<i32>>,
}

pub struct World{
    entity_manager: EntityManager,
    systems: SystemData,
    message_manager: MessageManager,
}

impl World{
    pub fn new() -> World{
        World{
            entity_manager: EntityManager::new(),
            systems: SystemData{
                systems: Vec::new(),
                aspects: Vec::new(),
                entities_id: Vec::new(),
            },
            message_manager: MessageManager::new()
        }
    }

    pub fn add_entity_manager(&mut self, em: EntityManager){
        self.entity_manager = em;
    }

    pub fn get_entity_manager(&mut self) -> &mut EntityManager{
        &mut self.entity_manager
    }

    pub fn add_system<T: System + 'static>(&mut self, s: T){
        let r = s.aspect();
        self.systems.systems.push(Box::new(s));
        self.systems.aspects.push(r);
        self.systems.entities_id.push(Vec::new());
    }

    pub fn update(&mut self, engine: &mut Engine, dt: f32){
        let mut em = &mut self.entity_manager;
        if !em.get_added().is_empty(){
            for s_id in 0..self.systems.systems.len() {
                for n in 0..em.get_added().len(){
                    let e_id = em.get_added().get(n).unwrap().clone();
                    let e = em.get_by_id(&e_id);
                    let mut result = false;
                    let aspect = self.systems.aspects.get(s_id).unwrap().get();
                    for r in aspect.iter(){
                        if !e.has_component_type(r){
                            result = false;
                            break;
                        }else{
                            result = true;
                        }
                    }
                    if result {
                        self.systems.entities_id.get_mut(s_id).unwrap().push(e_id);
                    }

                }
            }
        }
        if !em.get_removed().is_empty(){
            for s_id in 0..self.systems.systems.len() {
                let mut entities = self.systems.entities_id.get_mut(s_id).unwrap();
                for n in 0..em.get_removed().len(){
                    let e_id = em.get_removed().get(n).unwrap().clone();
                    entities.retain(|&i|i != e_id);
                }
            }
        }
        if !em.get_added().is_empty() || !em.get_removed().is_empty() {
            for s_id in 0..self.systems.systems.len() {
                let mut s = self.systems.systems.get_mut(s_id).unwrap();
                s.entity_changed(&self.systems.entities_id.get(s_id).unwrap(), em, engine);
            }
        }
        em.update();
        for i in 0..self.systems.systems.len() {
            let mut e_id = self.systems.entities_id.get_mut(i).unwrap();
            let mut s = self.systems.systems.get_mut(i).unwrap();

            {
                s.process_one(em, &mut self.message_manager, engine, dt);
            }
            for j in e_id.iter(){
                {
                    s.process(j, em, engine, dt);
                }
                {
                    s.process_m(j, &mut self.message_manager, em, engine, dt);
                }
            }

        }

        if !self.message_manager.get().is_empty(){
            for s in self.systems.systems.iter_mut(){
                s.receive(&self.message_manager, em, engine, dt);
            }
            self.message_manager.clear();
        }
    }

    pub fn initialize_system(&mut self, engine: &mut Engine){
        for s in self.systems.systems.iter_mut(){
            s.init(&mut self.message_manager, &mut self.entity_manager, engine);
        }
    }
}

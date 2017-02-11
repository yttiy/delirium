use std::any::{ Any, TypeId };
use std::collections::HashMap;
use std::cell::RefCell;
use engine::Component;
use std::ops::{Deref, DerefMut, Drop};

pub struct Entity{
    components: RefCell<HashMap<TypeId, Box<Any>>>,
    id: i32,
}

impl Entity{
    pub fn new(id: i32) -> Entity{
        Entity{
            components: RefCell::new(HashMap::new()),
            id: id
        }
    }

    pub fn get_id(&self) -> &i32 {
        &self.id
    }

    pub fn add_component<T: Any + Component>(&self, c: T){
        self.components.borrow_mut().insert(TypeId::of::<T>(), Box::new(c));
    }

    pub fn get_component<T: Any + Component>(&self) -> ComponentManager<T>{
        let c = self.components.borrow_mut().remove(&TypeId::of::<T>()).unwrap().downcast().unwrap();
        ComponentManager {
            component: Some(c),
            collection: &self.components,
        }
    }

    pub fn has_component<T : Any>(&self) -> bool {
        self.components.borrow().contains_key(&TypeId::of::<T>())
    }

    pub fn has_component_type(&self, t: &TypeId) -> bool {
        self.components.borrow().contains_key(t)
    }
}

/////////////

pub struct ComponentManager<'a, T : Any> {
    collection : &'a RefCell<HashMap<TypeId, Box<Any>>>,
    component  : Option<Box<T>>,
}

impl <'a, T : Any> DerefMut for ComponentManager<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.component.as_mut().unwrap()
    }
}

impl <'a, T : Any> Deref for ComponentManager<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.component.as_ref().unwrap()
    }
}

impl<'a, T : Any> Drop for ComponentManager<'a, T> {
    fn drop(&mut self) {
        self.component.take().and_then(|component| {
            self.collection.borrow_mut().insert(TypeId::of::<T>(), component)});
    }
}
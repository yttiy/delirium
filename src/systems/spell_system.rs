use components::{ Body, Spell };
use engine::{ Engine, System, Entity, Aspect, EntityManager };

pub struct SpellSystem{}

impl System for SpellSystem {
    fn aspect(&self) -> Aspect {
        Aspect::new()
            .add::<Body>()
            .add::<Spell>()
    }

    fn process(&mut self, id: &i32, em: &mut EntityManager, engine: &mut Engine, dt: f32) {
        let mut remove = false;
        {
            let mut entity = em.get_by_id(id);
            let mut body = entity.get_component::<Body>();
            let mut s = entity.get_component::<Spell>();
            if body.dx == 0.0 || body.on_ground {
                remove = true;
            }
            s.current_time += dt;
            if s.current_time > s.interval {
                remove = true;
            }
        }
        if remove {
            em.remove(&id.clone());
        }
        /*let mut entity = em.get_by_id(id);
        let mut body = entity.get_component::<Body>();
        if body.dx == 0.0 {

        }*/
        //body.dx = 80.0 * body.vector as f32;
        //em.remove(&id.clone());

    }
}

use engine::{ Engine, System, Entity, Aspect, EntityManager, UI, Bar };
use components::{ Position, NPC, Energy, NPCHealthBar, Drawable };

pub struct NPCHealthRenderSystem{}

impl System for NPCHealthRenderSystem {
    fn aspect(&self) -> Aspect {
        Aspect::new()
            .add::<NPC>()
            .add::<Position>()
            .add::<Drawable>()
            .add::<Energy>()
            .add::<NPCHealthBar>()
    }

    fn process(&mut self, id: &i32, em: &mut EntityManager, engine: &mut Engine, dt: f32){
        let mut entity = em.get_by_id(id);
        let mut pos = entity.get_component::<Position>();
        let mut energy = entity.get_component::<Energy>();
        let mut bar = entity.get_component::<NPCHealthBar>();
        let mut d = entity.get_component::<Drawable>();
        let n = bar.bar.get_width().clone() as f32;
        bar.bar.set_position(pos.x + d.tex_w as f32 / 2.0 - n / 2.0, pos.y - 3.0);
        bar.bar.set_value(energy.get_max_health() as i32, energy.get_health() as i32);
        bar.bar.draw(engine);
    }
}


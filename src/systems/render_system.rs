extern crate sdl2;
use components::{ Position, Drawable };
use engine::{ Engine, System, Entity, Aspect, EntityManager };

pub struct RenderSystem{}

impl System for RenderSystem{
    fn aspect(&self) -> Aspect{
        Aspect::new()
            .add::<Position>()
            .add::<Drawable>()
    }

    fn process(&mut self, id: &i32, em: &mut EntityManager, engine: &mut Engine, dt: f32){
        let mut entity = em.get_by_id(id);
        let d = entity.get_component::<Drawable>();
        let mut pos = entity.get_component::<Position>();
        engine.renderer.draw_region(&d.tex, pos.x, pos.y, d.tex_x, d.tex_y, d.tex_w, d.tex_h);
    }
}

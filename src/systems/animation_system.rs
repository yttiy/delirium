use engine::{ Engine, System, Entity, Aspect, EntityManager };
use components::{ Animation, Drawable };
pub struct AnimationSystem{}

impl System for AnimationSystem{
    fn aspect(&self) -> Aspect{
        Aspect::new()
            .add::<Animation>()
            .add::<Drawable>()
    }

    fn process(&mut self, id: &i32, em: &mut EntityManager, engine: &mut Engine, dt: f32){
        let mut entity = em.get_by_id(id);
        let mut d = entity.get_component::<Drawable>();
        let mut anim = entity.get_component::<Animation>();

        let q = &d.tex.query();
        d.tex_w = q.width  / anim.column() as u32;
        d.tex_h = q.height  / anim.row() as u32;

        let frame = anim.get_frame(dt);
        d.tex_y = frame / anim.column() * (d.tex_h as i32);
        d.tex_x = (frame - anim.column() * ((frame / anim.column()) as i32)) * d.tex_w as i32;

        //anim.update_time(dt);
    }


}

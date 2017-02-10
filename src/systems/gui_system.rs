use engine::{ Engine, System, Entity, Aspect, EntityManager, UI, Bar };
use components::{ GUI, Energy, Drawable };

pub struct GUISystem{}

impl System for GUISystem {
    fn aspect(&self) -> Aspect {
        Aspect::new()
            .add::<GUI>()
            .add::<Energy>()
    }

    fn process(&mut self, id: &i32, em: &mut EntityManager, engine: &mut Engine, dt: f32){
        engine.renderer.set_ui_layer();
        let mut entity = em.get_by_id(id);
        let mut gui = entity.get_component::<GUI>();
        let mut energy = entity.get_component::<Energy>();

        gui.health.set_value(energy.get_max_health() as i32, energy.get_health() as i32);
        gui.health.draw(engine);

        gui.mana.set_value(energy.get_max_mana() as i32, energy.get_mana() as i32);
        gui.mana.draw(engine);

        gui.text.update(dt);
        if gui.text.is_active(){
            gui.background.draw(engine);
            //engine.renderer.draw(&gui.background, 0.0, 0.0);
        }
        gui.text.draw(engine);
    }
}
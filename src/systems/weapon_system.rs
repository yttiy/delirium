use engine::{ Engine, System, Entity, Aspect, EntityManager, Message, MessageManager };
use components::{ Position, Weapon };

pub struct WeaponSystem{}

impl System for WeaponSystem {
    fn aspect(&self) -> Aspect {
        Aspect::new()
            .add::<Weapon>()
            .add::<Position>()
    }

    fn process_m(&mut self, id: &i32, mm: &mut MessageManager, em: &mut EntityManager, engine: &mut Engine, dt: f32){
        let mut e = em.get_by_id(id);
        let mut pos = e.get_component::<Position>();
        let mut w = e.get_component::<Weapon>();

        if w.current_time > 500.0 {
            w.current_time = 0.0;
        }
        if w.activated {
            w.current_time += dt;
            if w.current_time > w.attack_interval {
                w.current_time = 0.0;
                    mm.add(Message::CastSpell { x: pos.x + w.offset_x, y: pos.y + w.offset_y, dx: w.dx, dy: w.dy, is_player_cast: false, spell_type: w.spell_type });
                    if w.sound != -1 {
                        mm.add(Message::PlaySound{ s_id: w.sound });
                    }
                }
        }

    }
}
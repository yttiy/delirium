extern crate sdl2;
use engine::{ Engine, System, Entity, Aspect, EntityManager, MessageManager, Message };
use components::{ Player, Animation, Position, Drawable, Body, Energy };
use self::sdl2::keyboard::Keycode;
use fsm::States;

pub struct PlayerSystem{}

impl System for PlayerSystem{
    fn aspect(&self) -> Aspect{
        Aspect::new()
            .add::<Position>()
            .add::<Player>()
            .add::<Body>()
            .add::<Animation>()
            .add::<Drawable>()
            .add::<Energy>()
    }

    fn process_m(&mut self, id: &i32, mm: &mut MessageManager, em: &mut EntityManager, engine: &mut Engine, dt: f32){
        let mut entity = em.get_by_id(id);
        let mut p = entity.get_component::<Player>();
        let mut anim = entity.get_component::<Animation>();
        let mut pos = entity.get_component::<Position>();
        let mut d = entity.get_component::<Drawable>();
        let mut body = entity.get_component::<Body>();
        let mut energy = entity.get_component::<Energy>();

        if engine.event_manager.is_key_pressed(Keycode::Left) || engine.event_manager.is_key_pressed(Keycode::A) {
            body.dx = -40.0;
            p.vector = -1;
        }
        if engine.event_manager.is_key_pressed(Keycode::Right)|| engine.event_manager.is_key_pressed(Keycode::D){
            body.dx = 40.0;
            p.vector = 1;
        }
        if (engine.event_manager.is_key_pressed(Keycode::Up) || engine.event_manager.is_key_pressed(Keycode::Space)|| engine.event_manager.is_key_pressed(Keycode::W)) && body.on_ground{
            body.dy = -48.0;
        }

        if body.dx != 0.0 && body.dy == 0.0 {
            if body.dx > 0.0 {
                anim.set_animation(0);
            }else{
                anim.set_animation(1);
            }
        }else{
            if p.vector > 0 {
                anim.set_animation(2);
            }else{
                anim.set_animation(3);
            }
        }

        let mut cam = engine.renderer.get_camera();
        cam.set_central_position(pos.x + d.tex_w as f32 / 2.0, pos.y + d.tex_h as f32 / 2.0);

        if engine.event_manager.is_key_down(Keycode::E){
            if energy.get_mana() > 10.0 {
                let mut x = pos.x;
                if p.vector == 1 {
                    x = pos.x + d.tex_w as f32;
                }else if p.vector == -1 {
                    x = pos.x - 16.0;
                }
                let mut y = pos.y + (d.tex_w as f32) / 2.0;
                mm.add(Message::CastSpell{x: x, y: y, dx: 120.0 * p.vector as f32, dy: 0.0, is_player_cast: true, spell_type: 0 });
                mm.add(Message::PlaySound{ s_id: 1 });
                energy.reduce_mana(5.0);
            }
        }
        if energy.get_health() <= 0.0 {
            engine.state = States::GameOver;
        }
        energy.add_health(2.0 * dt);
        energy.add_mana(3.0 * dt);
    }
}
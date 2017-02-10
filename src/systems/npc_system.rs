use engine::{ Engine, System, Entity, Aspect, EntityManager, Message, MessageManager };
use components::{ Position, Body, Player, NPC, Energy, Drawable, Animation, Weapon };

pub struct NPCSystem{
    player_id: i32
}

impl NPCSystem{
    pub fn new() -> NPCSystem{
        NPCSystem{
            player_id: -1
        }
    }
    /*
    pub fn is_spider(&mut self, id: &i32, mm: &mut MessageManager, em: &mut EntityManager, engine: &mut Engine, dt: f32){
        let mut remove = false;
        {
            let mut entity = em.get_by_id(id);
            let mut pos = entity.get_component::<Position>();
            let mut energy = entity.get_component::<Energy>();
            let mut body = entity.get_component::<Body>();
            let mut npc = entity.get_component::<NPC>();
            let mut d = entity.get_component::<Drawable>();
            let mut anim = entity.get_component::<Animation>();

            if energy.get_health() < 1.0 {
                remove = true;
                mm.add(Message::CreateEntity{e_type: 0, x: pos.x, y: pos.y + d.tex_h as f32 });
            }


            body.dx = npc.vector as f32 * 30.0;


            if pos.x + d.tex_w as f32 > npc.start_x + npc.distance {
                npc.vector = -1;
            }
            if pos.x < npc.start_x {
                npc.vector = 1;
            }

            if npc.vector == 1 {
                anim.set_animation(0);
            }else if npc.vector == -1 {
                anim.set_animation(1);
            }

        }
        {
            if self.player_id != -1 {
                let mut player_x: f32;
                let mut player_bottom_y: f32;
                {
                    let mut e = em.get_by_id(&self.player_id);
                    let pos = e.get_component::<Position>();
                    let d = e.get_component::<Drawable>();
                    player_x = pos.x;
                    player_bottom_y = pos.y + d.tex_h as f32;

                }
                let mut entity = em.get_by_id(id);
                let mut npc = entity.get_component::<NPC>();
                let mut pos = entity.get_component::<Position>();
                let d = entity.get_component::<Drawable>();


                let npc_bottom_y = pos.y + d.tex_h as f32;
                npc.current_time += dt;

                if npc.current_time > 500.0 {
                    npc.current_time = 0.0;
                }
                if player_x > npc.start_x && player_x < npc.start_x + npc.distance && player_bottom_y == npc_bottom_y {
                    if pos.x - player_x > 0.0 {
                        npc.vector = -1;
                    }else{
                        npc.vector = 1;
                    }

                    if npc.current_time > npc.attack_interval {
                        npc.current_time = 0.0;
                        let mut x: f32;
                        if npc.vector == 1{
                            x = d.tex_w as f32 + pos.x;
                        }else{
                            x = pos.x - 16.0;
                        }
                        mm.add(Message::PlaySound{ s_id: 2 });
                        mm.add(Message::CastSpell{x: x, y: pos.y + 8.0, dx: 120.0 * npc.vector as f32, dy: 0.0, is_player_cast: false, spell_type: 1 });
                    }

                }

            }
        }
        if remove {
            em.remove(id);
        }
    }

    pub fn is_rod(&mut self, id: &i32, mm: &mut MessageManager, em: &mut EntityManager, engine: &mut Engine, dt: f32){
        let mut entity = em.get_by_id(id);
        let mut pos = entity.get_component::<Position>();
        let mut npc = entity.get_component::<NPC>();

        npc.current_time += dt;
        if npc.current_time > 500.0 {
            npc.current_time = 0.0;
        }

        if npc.current_time > npc.attack_interval {

            npc.current_time = 0.0;
            mm.add(Message::CastSpell{x: pos.x, y: pos.y, dx: -80.0, dy: 80.0, is_player_cast: false, spell_type: 1 });
        }

    }
    */
}

impl System for NPCSystem {
    fn aspect(&self) -> Aspect {
        Aspect::new()
            .add::<NPC>()
            .add::<Position>()
            .add::<Weapon>()
            //.add::<Energy>()
            //.add::<Body>()
            //.add::<Drawable>()
            //.add::<Animation>()
    }

    fn process_m(&mut self, id: &i32, mm: &mut MessageManager, em: &mut EntityManager, engine: &mut Engine, dt: f32){
        //weapon
        let mut remove = false;
        {
            let mut entity = em.get_by_id(id);
            let mut pos = entity.get_component::<Position>();
            let mut energy = entity.get_component::<Energy>();
            let mut body = entity.get_component::<Body>();
            let mut npc = entity.get_component::<NPC>();
            let mut d = entity.get_component::<Drawable>();
            let mut anim = entity.get_component::<Animation>();

            if energy.get_health() < energy.get_health() / 100.0 {
                remove = true;
                mm.add(Message::CreateEntity{e_type: npc.ash, x: pos.x, y: pos.y + d.tex_h as f32 });
            }


            body.dx = npc.vector as f32 * 30.0;


            if pos.x + d.tex_w as f32 > npc.start_x + npc.distance {
                npc.vector = -1;
            }
            if pos.x < npc.start_x {
                npc.vector = 1;
            }

            if npc.vector == 1 {
                anim.set_animation(0);
            }else if npc.vector == -1 {
                anim.set_animation(1);
            }

        }
        {
            if self.player_id != -1 {
                let mut player_x: f32;
                let mut player_bottom_y: f32;
                {
                    let mut e = em.get_by_id(&self.player_id);
                    let pos = e.get_component::<Position>();
                    let d = e.get_component::<Drawable>();
                    player_x = pos.x;
                    player_bottom_y = pos.y + d.tex_h as f32;

                }
                let mut entity = em.get_by_id(id);
                let mut npc = entity.get_component::<NPC>();
                let mut pos = entity.get_component::<Position>();
                let d = entity.get_component::<Drawable>();
                let mut w = entity.get_component::<Weapon>();

                let npc_bottom_y = pos.y + d.tex_h as f32;

                if player_x > npc.start_x && player_x < npc.start_x + npc.distance && player_bottom_y == npc_bottom_y {
                    w.activated = true;
                    if pos.x - player_x > 0.0 {
                        npc.vector = -1;
                        w.dx = - 120.0;
                        w.offset_x = - 8.0;
                    }else{
                        npc.vector = 1;
                        w.dx = 120.0;
                        w.offset_x = d.tex_w as f32;
                    }
                }else{
                    w.activated = false;
                }

            }
        }
        if remove {
            em.remove(id);
        }
        /*let r = em.get_by_id(id).get_component::<NPC>().npc_type.clone();
        match r {
            NPCType::Spider => {
                self.is_spider(id, mm, em, engine, dt);

            },
            NPCType::Rod => {
                self.is_rod(id, mm, em, engine, dt);
            }
            NPCType::Robot => {
                //self.is_rod(id, mm, em, engine, dt);
            }
        }*/
    }

    fn receive(&mut self, mm: &MessageManager, em: &mut EntityManager, engine: &mut Engine, dt: f32) {
        for m in mm.get().iter() {
            match *m {
                Message::SendPlayerId { id: id } => {
                    self.player_id = id;
                },
                _ => {}
            }
        }
    }
}
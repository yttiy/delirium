use engine::{ Engine, System, Entity, Aspect, EntityManager, MessageManager, Message };
use components::{ Position, Body, Drawable, Player, Spell, NPC, Energy, Info, GUI, Portal, Death, Bottle };
use std::rc::Rc;
use util::MapLoader;
use fsm::States;
use util::Pref;

pub struct CollisionSystem{
    map: Rc<MapLoader>,
    gravity: f32,
    entities: Vec<i32>,
    interval: f32,
    current_time: f32,
}

impl CollisionSystem {
    pub fn new(map: Rc<MapLoader>, gravity: f32) -> CollisionSystem {
        CollisionSystem {
            map: map,
            gravity: gravity,
            entities: Vec::new(),
            interval: 1.0 / 60.0,
            current_time: 0.0,
        }
    }

    pub fn collision(&mut self, mode: i32, pos: &mut Position, body: &mut Body, d:&Drawable){
        let n_y = (pos.y / self.map.tile_height as f32) as i32;
        let n_y1 = (pos.y as i32 + d.tex_h as i32) / self.map.tile_height;

        let n_x = (pos.x / self.map.tile_width as f32) as i32;
        let n_x1 = (pos.x as i32 + d.tex_w as i32) / self.map.tile_width;

        if n_y1 > self.map.row - 1 || n_x1 > self.map.column - 1 || n_y < 0 || n_x < 0 {
            return
        }
        for i in n_y..(n_y1 + 1){
            for j in n_x..(n_x1 + 1){
                let n = self.map.map[2][(self.map.column * (i) + j) as usize];

                if n != 0{

                    if mode == 0 {
                        if body.dx < 0.0 {
                            pos.x = ((j + 1) * self.map.tile_width) as f32 + 0.0001;
                        }
                        if body.dx > 0.0 {
                            pos.x = (j * self.map.tile_width) as f32 - d.tex_w as f32 - 0.0001;
                        }
                        if body.is_static {
                            body.dx = 0.0;
                        }
                    }else{
                        if body.dy > 0.0 {
                            body.on_ground = true;
                            pos.y = ((i) * self.map.tile_height) as f32  - d.tex_h as f32 - 0.0001;
                            body.dy = 0.0;
                        }
                        if body.dy < 0.0 {
                            pos.y = ((i + 1) * self.map.tile_height) as f32; //+ d.tex_h as f32 + 0.0001;
                            body.dy = 0.0;
                        }
                    }
                }
            }
        }
    }

    pub fn intersects(&self, x: f32, y: f32, w: f32, h: f32, x1: f32, y1: f32, w1: f32, h1: f32) -> bool {
        if x + w < x1 || x1 + w1 < x || y + h < y1 || y1 + h1 < y {
            return false;
        }
        true
    }

    pub fn is_body_collision(&mut self, id: i32, id1: i32, em: &mut EntityManager, mm: &mut MessageManager, engine: &mut Engine){
        self.npc_and_spell(id, id1, em);
        self.player_and_npc(id, id1, em, mm);
        self.player_and_spell(id, id1, em);
        self.player_and_info(id, id1, em, engine);
        self.player_and_portal(id, id1, em, engine);
        self.player_and_death(id, id1, em, engine);
        self.player_and_bottle(id, id1, em);
    }

    pub fn npc_and_spell(&mut self, id: i32, id1: i32, em: &mut EntityManager){
        if em.get_by_id(&id).has_component::<NPC>() && em.get_by_id(&id1).has_component::<Spell>(){
            if !em.get_by_id(&id1).get_component::<Spell>().is_player_cast {
                return
            }
            {
                let damage = em.get_by_id(&id1).get_component::<Spell>().damage.clone();
                let mut e = em.get_by_id(&id).get_component::<Energy>();
                e.reduce_health(damage);
            }
            em.remove(&id1);
        }
    }

    pub fn player_and_npc(&mut self, id: i32, id1: i32, em: &mut EntityManager, mm: &mut MessageManager){
        if em.get_by_id(&id).has_component::<Player>() && em.get_by_id(&id1).has_component::<NPC>() && em.get_by_id(&id1).has_component::<Body>() && em.get_by_id(&id1).has_component::<Drawable>(){
            let npc_x: f32;
            let npc_w: f32;
            {
                let mut e = em.get_by_id(&id1);
                npc_x = e.get_component::<Position>().x.clone();
                let mut body = e.get_component::<Body>();
                body.dx = 0.0;
                npc_w = e.get_component::<Drawable>().tex_w.clone() as f32;
            }

            let mut e = em.get_by_id(&id);
            let mut body = e.get_component::<Body>();
            let mut pl = e.get_component::<Player>();
            let mut pos = e.get_component::<Position>();
            let mut d = e.get_component::<Drawable>();

            if npc_x - pos.x > 0.0 {
                body.dx = -98.0;
            }else{
                body.dx = 98.0;
            }
            let mut energy = e.get_component::<Energy>();

            energy.reduce_health(1.0);
            mm.add(Message::PlaySound{ s_id: 3});
        }
    }

    pub fn player_and_spell(&mut self, id: i32, id1: i32, em: &mut EntityManager){
        if em.get_by_id(&id).has_component::<Player>() && em.get_by_id(&id1).has_component::<Spell>(){
            if em.get_by_id(&id1).get_component::<Spell>().is_player_cast {
                return
            }
            {
                let damage = em.get_by_id(&id1).get_component::<Spell>().damage.clone();
                let mut e = em.get_by_id(&id).get_component::<Energy>();
                e.reduce_health(damage);
            }
            em.remove(&id1);
        }
    }

    pub fn player_and_info(&mut self, id: i32, id1: i32, em: &mut EntityManager, engine: &mut Engine){
        if em.get_by_id(&id).has_component::<Player>() && em.get_by_id(&id1).has_component::<Info>(){
            {
                let mut info_id = em.get_by_id(&id1).get_component::<Info>().id.clone();
                let mut gui = em.get_by_id(&id).get_component::<GUI>();
                gui.text.set_text(info_id);
                gui.set_info_position(engine);
                //let text = em.get_by_id(&id1).get_component::<Info>().text.clone();
                //println!("{}", text);
            }
            em.remove(&id1);
        }
    }

    pub fn player_and_bottle(&mut self, id: i32, id1: i32, em: &mut EntityManager){
        if em.get_by_id(&id).has_component::<Player>() && em.get_by_id(&id1).has_component::<Bottle>(){
            {
                if em.get_by_id(&id1).get_component::<Bottle>().health{
                    em.get_by_id(&id).get_component::<Energy>().add_health(45.0);
                }
                if em.get_by_id(&id1).get_component::<Bottle>().mana{
                    em.get_by_id(&id).get_component::<Energy>().add_mana(45.0);
                }
            }
            em.remove(&id1);
        }
    }

    pub fn player_and_portal(&mut self, id: i32, id1: i32, em: &mut EntityManager, engine: &mut Engine) {
        if em.get_by_id(&id).has_component::<Player>() && em.get_by_id(&id1).has_component::<Portal>() {
            if !em.get_by_id(&id1).get_component::<Portal>().end {
                let mut p = Pref::open();
                let lvl = p.get_int("l").clone() + 1;
                p.set_int("l", lvl);
                p.save();
                engine.state = States::Play;
            }else{
                let mut p = Pref::open();
                p.set_int("l", 0);
                p.save();
                engine.state = States::GameEnd;
            }

        }
    }

    pub fn player_and_death(&mut self, id: i32, id1: i32, em: &mut EntityManager, engine: &mut Engine) {
        if em.get_by_id(&id).has_component::<Player>() && em.get_by_id(&id1).has_component::<Death>() {
            engine.state = States::GameOver;
        }
    }
}

impl System for CollisionSystem {
    fn aspect(&self) -> Aspect {
        Aspect::new()
            .add::<Position>()
            .add::<Body>()
            .add::<Drawable>()
    }

    fn entity_changed(&mut self, e: &Vec<i32>, em: &mut EntityManager, engine: &mut Engine){
        self.entities = e.clone();
    }

    fn process(&mut self, id: &i32, em: &mut EntityManager, engine: &mut Engine, dt: f32){
        let mut entity = em.get_by_id(id);
        let mut pos = entity.get_component::<Position>();
        let mut body = entity.get_component::<Body>();
        let d = entity.get_component::<Drawable>();

        pos.x += body.dx * dt;
        self.collision(0, &mut pos, &mut body, &d);
        if !body.on_ground && !body.is_static {
            body.dy += self.gravity * dt;
        }
        pos.y += body.dy * dt;
        body.on_ground = false;
        self.collision(1, &mut pos, &mut body, &d);

        if !body.is_static && body.dy == 0.0 {
            body.dx = 0.0;
        }
    }

    fn process_one(&mut self, em: &mut EntityManager, mm: &mut MessageManager, engine: &mut Engine, dt: f32){
        if self.current_time > self.interval {
            self.current_time = 0.0;
        }else{
            self.current_time += dt;
            return;
        }
        //govnocod
        self.entities.sort_by_key(|&k| em.get_by_id(&k).get_component::<Position>().x as i32);
        for i in 0..self.entities.len(){
            let eid = self.entities.get(i).unwrap().clone();
            let mut ix: f32;
            let mut iy: f32;
            let mut iwidth: f32;
            let mut iheight: f32;

            {
                let mut e = em.get_by_id(&eid);
                let pos = e.get_component::<Position>();
                let body = e.get_component::<Body>();
                let d = e.get_component::<Drawable>();
                ix = pos.x;
                iy = pos.y;
                iwidth = d.tex_w as f32;
                iheight = d.tex_h as f32;
            }
            ////


            for j in (i + 1)..self.entities.len(){
                let eid1 = self.entities.get(j).unwrap().clone();
                let mut jx: f32;
                let mut jy: f32;
                let mut jwidth: f32;
                let mut jheight: f32;
                {
                    let mut e = em.get_by_id(&eid1);
                    let pos = e.get_component::<Position>();
                    let body = e.get_component::<Body>();
                    let d = e.get_component::<Drawable>();
                    jx = pos.x;
                    jy = pos.y;
                    jwidth = d.tex_w as f32;
                    jheight = d.tex_h as f32;
                }

                if ix + iwidth < jx {
                    break;
                }

                if self.intersects(ix, iy, iwidth, iheight, jx, jy, jwidth, jheight){
                    self.is_body_collision(eid, eid1, em, mm, engine);
                    self.is_body_collision(eid1, eid, em, mm, engine);
                }
            }
        }
    }
}
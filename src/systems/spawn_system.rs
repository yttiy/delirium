extern crate sdl2;
use self::sdl2::render::Texture;
use engine::{ Engine, System, Entity, Aspect, EntityManager, MessageManager, Message };
use std::path::Path;
use std::rc::Rc;
use components::{ Position, Drawable, Map, Animation, Player, Body, Spell, NPC, Energy, NPCHealthBar, GUI, Info, Portal, Death, Weapon, Bottle };
use util::MapLoader;

pub struct SpawnSystem{
    tex: Rc<Texture>,
    tex01: Rc<Texture>,
    tex02: Rc<Texture>,
    tex03: Rc<Texture>,
    tex04: Rc<Texture>,
    tex05: Rc<Texture>,
    tex06: Rc<Texture>,
    tex07: Rc<Texture>,
    tex08: Rc<Texture>,
    tex09: Rc<Texture>,
    tex10: Rc<Texture>,
    tex11: Rc<Texture>,
    tex12: Rc<Texture>,
    tex13: Rc<Texture>,
    tex14: Rc<Texture>,
    tex15: Rc<Texture>,
    map: Rc<MapLoader>
}

impl SpawnSystem{
    pub fn new(engine: &mut Engine, map: Rc<MapLoader>) -> SpawnSystem {
        SpawnSystem {
            tex: Rc::new(engine.renderer.create_texture(Path::new("res/textures/player.png")).unwrap()),
            tex01: Rc::new(engine.renderer.create_texture(Path::new("res/textures/spell.png")).unwrap()),
            tex02: Rc::new(engine.renderer.create_texture(Path::new("res/textures/spider.png")).unwrap()),
            tex03: Rc::new(engine.renderer.create_texture(Path::new("res/textures/info.png")).unwrap()),
            tex04: Rc::new(engine.renderer.create_texture(Path::new("res/textures/blood.png")).unwrap()),
            tex05: Rc::new(engine.renderer.create_texture(Path::new("res/textures/portal.png")).unwrap()),
            tex06: Rc::new(engine.renderer.create_texture(Path::new("res/textures/skeleton.png")).unwrap()),
            tex07: Rc::new(engine.renderer.create_texture(Path::new("res/textures/water.png")).unwrap()),
            tex08: Rc::new(engine.renderer.create_texture(Path::new("res/textures/water1.png")).unwrap()),
            tex09: Rc::new(engine.renderer.create_texture(Path::new("res/textures/robot.png")).unwrap()),
            tex10: Rc::new(engine.renderer.create_texture(Path::new("res/textures/robot1.png")).unwrap()),
            tex11: Rc::new(engine.renderer.create_texture(Path::new("res/textures/bottle.png")).unwrap()),
            tex12: Rc::new(engine.renderer.create_texture(Path::new("res/textures/ash.png")).unwrap()),
            tex13: Rc::new(engine.renderer.create_texture(Path::new("res/textures/ash1.png")).unwrap()),
            tex14: Rc::new(engine.renderer.create_texture(Path::new("res/textures/bug.png")).unwrap()),
            tex15: Rc::new(engine.renderer.create_texture(Path::new("res/textures/alien.png")).unwrap()),
            map: map
        }
    }

    pub fn create_spell(&mut self, em: &mut EntityManager, x: f32, y: f32, dx: f32, dy: f32, is_player_cast: bool, spell_type: i32){
        let mut e = em.create_entity();
        e.add_component(Drawable::new(self.tex01.clone()));
        e.add_component(Position{x: x, y: y});
        e.add_component(Body::new(dx, dy, true));
        let mut anim = Animation::new(2, 5, 3.0);
        let mut damage = 17.0;
        match spell_type{
            0 => {
                anim.add_animation(vec![0, 1]);
            },
            1 => {
                anim.add_animation(vec![2, 3]);
            },
            2 => {
                anim.add_animation(vec![4, 5]);
            },
            3 => {
                anim.add_animation(vec![6, 7]);
                damage = 5.0;
            },
            4 => {
                anim.add_animation(vec![8, 9]);
                damage = 5.0;
            },
            _ => {}
        }

        e.add_component(Spell::new(damage, is_player_cast, 0.5));
        e.add_component(anim);
    }

    pub fn create_blood(&mut self, em: &mut EntityManager, engine: &mut Engine, x: f32, y: f32){
        let mut e = em.create_entity();
        e.add_component(Drawable::new(self.tex04.clone()));
        e.add_component(Position{x: x, y: y});
    }

    pub fn create_ash(&mut self, em: &mut EntityManager, engine: &mut Engine, x: f32, y: f32){
        let mut e = em.create_entity();
        e.add_component(Drawable::new(self.tex12.clone()));
        e.add_component(Position{x: x, y: y});
    }

    pub fn create_ash1(&mut self, em: &mut EntityManager, engine: &mut Engine, x: f32, y: f32){
        let mut e = em.create_entity();
        e.add_component(Drawable::new(self.tex13.clone()));
        e.add_component(Position{x: x, y: y});
    }

    pub fn create_info(&mut self, id: i32, em: &mut EntityManager, engine: &mut Engine, x: f32, y: f32){
        let mut e = em.create_entity();
        e.add_component(Drawable::new(self.tex03.clone()));
        e.add_component(Position{x: x, y: y});
        e.add_component(Body::new(0.0, 0.0, false));
        e.add_component(Info::new(id));

    }

    pub fn create_portal(&mut self, em: &mut EntityManager, engine: &mut Engine, x: f32, y: f32, end: bool){
        let mut e = em.create_entity();
        e.add_component(Drawable::new(self.tex05.clone()));
        e.add_component(Position{x: x, y: y});
        e.add_component(Body::new(0.0, 0.0, false));
        let mut anim = Animation::new(2, 1, 5.0);
        anim.add_animation(vec![0, 1]);
        e.add_component(anim);
        e.add_component(Portal{end: end});

    }

    pub fn create_robot(&mut self, em: &mut EntityManager, engine: &mut Engine, x: f32, y: f32){
        let mut e = em.create_entity();
        e.add_component(Drawable::new(self.tex09.clone()));
        e.add_component(Position{x: x, y: y});
        let mut anim = Animation::new(4, 2, 5.0);
        anim.add_animation(vec![0, 1, 2, 3]);
        anim.add_animation(vec![4, 5, 6, 7]);
        e.add_component(anim);
        e.add_component(Body::new(0.0, 0.0, false));
        e.add_component(NPC::new(x, 32.0 * 4.0, 2));
        e.add_component(Energy::new(200.0, 0.0));
        e.add_component(NPCHealthBar::new(16, 1, engine));
        let mut w = Weapon::new(1.0, 0.0, 30.0 , 2 , 2, false);
        w.offset_y = 13.0;
        e.add_component(w);
    }

    pub fn create_robot1(&mut self, em: &mut EntityManager, engine: &mut Engine, x: f32, y: f32){
        let mut e = em.create_entity();
        e.add_component(Drawable::new(self.tex10.clone()));
        e.add_component(Position{x: x, y: y});
        let mut anim = Animation::new(4, 2, 5.0);
        anim.add_animation(vec![0, 1, 2, 3]);
        anim.add_animation(vec![4, 5, 6, 7]);
        e.add_component(anim);
        e.add_component(Body::new(0.0, 0.0, false));
        e.add_component(NPC::new(x, 32.0 * 4.0, 2));
        e.add_component(Energy::new(200.0, 0.0));
        e.add_component(NPCHealthBar::new(16, 1, engine));
        let mut w = Weapon::new(0.2, 0.0, 30.0 , 4 , 3, false);
        w.offset_y = 14.0;
        e.add_component(w);
    }

    pub fn create_death(&mut self, em: &mut EntityManager, engine: &mut Engine, x: f32, y: f32){
        let mut e = em.create_entity();
        e.add_component(Drawable::new(self.tex03.clone()));
        e.add_component(Position{x: x, y: y});
        e.add_component(Body::new(0.0, 0.0, true));
        e.add_component(Death{});

    }

    pub fn create_spider(&mut self, em: &mut EntityManager, engine: &mut Engine, x: f32, y: f32){
        let mut e = em.create_entity();
        e.add_component(Drawable::new(self.tex02.clone()));
        e.add_component(Position{x: x, y: y});
        let mut anim = Animation::new(3, 2, 5.0);
        anim.add_animation(vec![0, 1, 2, 1]);
        anim.add_animation(vec![3, 4, 5, 4]);
        e.add_component(anim);
        e.add_component(Body::new(0.0, 0.0, false));
        e.add_component(NPC::new(x, 32.0 * 4.0, 0));
        e.add_component(Energy::new(110.0, 0.0));
        e.add_component(NPCHealthBar::new(16, 1, engine));
        e.add_component(Weapon::new(1.0, 0.0, 0.0 , -1 , 1, false));
    }

    pub fn create_bug(&mut self, em: &mut EntityManager, engine: &mut Engine, x: f32, y: f32){
        let mut e = em.create_entity();
        e.add_component(Drawable::new(self.tex14.clone()));
        e.add_component(Position{x: x, y: y});
        let mut anim = Animation::new(2, 2, 3.0);
        anim.add_animation(vec![0, 1]);
        anim.add_animation(vec![3, 2]);

        e.add_component(anim);
        e.add_component(Body::new(0.0, 0.0, false));
        e.add_component(NPC::new(x, 32.0 * 6.0, 0));
        e.add_component(Energy::new(190.0, 0.0));
        e.add_component(NPCHealthBar::new(32, 1, engine));
        e.add_component(Weapon::new(1.0, 0.0, 0.0 , -1 , 1, false));
    }

    pub fn create_rod(&mut self, em: &mut EntityManager, engine: &mut Engine, x: f32, y: f32){
        {
            let mut e = em.create_entity();
            e.add_component(Position{x: x, y: y});
            e.add_component(Weapon::new(1.0, -80.0, 80.0 , -1 , 3, true));
        }

        {
            let mut e = em.create_entity();
            e.add_component(Position{x: x, y: y});
            e.add_component(Weapon::new(6.0, 80.0, 80.0 , -1 , 3, true));
        }

     }

    pub fn create_player(&mut self, em: &mut EntityManager, mm: &mut MessageManager, engine: &mut Engine, x: f32, y: f32, ){
        let mut e = em.create_entity();
        let mut p = Player::new();
        p.vector = 1;
        e.add_component(p);
        e.add_component(Drawable::new(self.tex.clone()));
        e.add_component(Position{x: x, y: y});
        e.add_component(Body::new(0.0, 0.0, false));
        let mut anim = Animation::new(4, 2, 6.0);
        anim.add_animation(vec![0, 1, 2, 1]);
        anim.add_animation(vec![6, 5, 4, 5]);
        anim.add_animation(vec![1, 1, 1, 1, 1, 3, 3, 3, 3, 3]);
        anim.add_animation(vec![7, 7, 7, 7, 7, 5, 5, 5, 5, 5]);
        e.add_component(anim);
        e.add_component(GUI::new(engine));
        e.add_component(Energy::new(100.0, 100.0));
        let p_id = e.get_id().clone();
        mm.add(Message::SendPlayerId{id: p_id})
    }


    pub fn create_skeleton(&mut self, em: &mut EntityManager, engine: &mut Engine, x: f32, y: f32){
        let mut e = em.create_entity();
        e.add_component(Drawable::new(self.tex06.clone()));
        let mut anim = Animation::new(3, 2, 5.0);
        anim.add_animation(vec![0, 1, 2, 1]);
        anim.add_animation(vec![3, 4, 5, 4]);
        e.add_component(anim);
        e.add_component(Position{x: x, y: y});
        e.add_component(Body::new(0.0, 0.0, false));
        e.add_component(NPC::new(x, 32.0 * 4.0, 1));
        e.add_component(Energy::new(100.0, 0.0));
        e.add_component(NPCHealthBar::new(16, 1, engine));
        e.add_component(Weapon::new(1.0, 0.0, 0.0 , -1 , 1, false));
    }

    pub fn create_alien(&mut self, em: &mut EntityManager, engine: &mut Engine, x: f32, y: f32){
        let mut e = em.create_entity();
        e.add_component(Drawable::new(self.tex15.clone()));
        let mut anim = Animation::new(3, 2, 5.0);
        anim.add_animation(vec![0, 1, 2, 1]);
        anim.add_animation(vec![3, 4, 5, 4]);
        e.add_component(anim);
        e.add_component(Position{x: x, y: y});

        e.add_component(Body::new(0.0, 0.0, false));
        e.add_component(NPC::new(x, 32.0 * 4.0, 0));
        e.add_component(Energy::new(140.0, 0.0));
        e.add_component(NPCHealthBar::new(16, 1, engine));
        e.add_component(Weapon::new(0.2, 0.0, 0.0 , -1 , 4, false));
    }

    pub fn create_water(&mut self, em: &mut EntityManager, engine: &mut Engine, x: f32, y: f32){
        let mut e = em.create_entity();
        e.add_component(Drawable::new(self.tex07.clone()));
        let mut anim = Animation::new(1, 3, 4.0);
        anim.add_animation(vec![0, 1, 2, 1]);
        e.add_component(anim);
        e.add_component(Position{x: x, y: y});
        e.add_component(Death{});
        e.add_component(Body::new(0.0, 0.0, true));
    }

    pub fn create_water1(&mut self, em: &mut EntityManager, engine: &mut Engine, x: f32, y: f32){
        let mut e = em.create_entity();
        e.add_component(Drawable::new(self.tex08.clone()));
        let mut anim = Animation::new(3, 1, 6.0);
        anim.add_animation(vec![0, 1, 2]);
        e.add_component(anim);
        e.add_component(Position{x: x, y: y});
    }

    pub fn create_bottle(&mut self, em: &mut EntityManager, engine: &mut Engine, x: f32, y: f32, b_type: i32){
        let mut e = em.create_entity();
        e.add_component(Drawable::new(self.tex11.clone()));
        let mut anim = Animation::new(2, 2, 2.0);
        if b_type == 0 {
            anim.add_animation(vec![0, 1]);
            e.add_component(Bottle{ health: true, mana: false });
        }else{
            anim.add_animation(vec![2, 3]);
            e.add_component(Bottle{ health: false, mana: true });
        }
        e.add_component(anim);
        e.add_component(Position{x: x, y: y});
        e.add_component(Body::new(0.0, 0.0, false));
    }


    pub fn load_map(&mut self, em: &mut EntityManager, mm: &mut MessageManager, engine: &mut Engine){
        {
            let mut e = em.create_entity();
            e.add_component(Map::new(self.map.clone(), engine));
        }

        for o in self.map.clone().objects.get(0).unwrap().iter() {
            if o.m_type == "player" {
                self.create_player(em, mm, engine, o.x, o.y);
            }
            if o.m_type == "spider" {
                self.create_spider(em, engine, o.x, o.y);
            }
            if o.m_type == "bug" {
                self.create_bug(em, engine, o.x, o.y);
            }
            if o.m_type == "info0" {
                self.create_info(0, em, engine, o.x, o.y);
            }
            if o.m_type == "info1" {
                self.create_info(1, em, engine, o.x, o.y);
            }
            if o.m_type == "info2" {
                self.create_info(2, em, engine, o.x, o.y);
            }
            if o.m_type == "info3" {
                self.create_info(3, em, engine, o.x, o.y);
            }
            if o.m_type == "portal" {
                self.create_portal( em, engine, o.x, o.y, false);
            }
            if o.m_type == "portal_end" {
                self.create_portal( em, engine, o.x, o.y, true);
            }
            if o.m_type == "death" {
                self.create_death( em, engine, o.x, o.y);
            }
            if o.m_type == "skeleton" {
                self.create_skeleton( em, engine, o.x, o.y);
            }
            if o.m_type == "alien" {
                self.create_alien( em, engine, o.x, o.y);
            }
            if o.m_type == "water" {
                self.create_water( em, engine, o.x, o.y);
            }
            if o.m_type == "water1" {
                self.create_water1( em, engine, o.x, o.y);
            }
            if o.m_type == "rod" {
                self.create_rod( em, engine, o.x, o.y);
            }
            if o.m_type == "robot" {
                self.create_robot( em, engine, o.x, o.y);
            }
            if o.m_type == "robot1" {
                self.create_robot1( em, engine, o.x, o.y);
            }
            if o.m_type == "health" {
                self.create_bottle( em, engine, o.x, o.y, 0);
            }
            if o.m_type == "mana" {
                self.create_bottle( em, engine, o.x, o.y, 1);
            }
        }
    }

}

impl System for SpawnSystem{
    fn aspect(&self) -> Aspect{
        Aspect::new()
    }

    fn init(&mut self, mm: &mut MessageManager, em: &mut EntityManager, engine: &mut Engine){
        self.load_map(em, mm, engine);
        engine.renderer.get_camera().set_zoom(0.2);

    }

    fn receive(&mut self, mm: &MessageManager, em: &mut EntityManager, engine: &mut Engine, dt: f32){
        for message in mm.get().iter(){

            match *message{
                Message::CastSpell{x: x, y: y, dx: dx, dy: dy, is_player_cast, spell_type } => {
                    self.create_spell(em, x, y, dx, dy, is_player_cast, spell_type);
                },
                Message::CreateEntity{e_type: e_type, x: x, y: y} => {
                    //self.create_spell(em, x, y, vector, is_player_cast, spell_type);
                    if e_type == 0 {
                        self.create_blood(em, engine, x, y, );
                    }
                    if e_type == 1 {
                        self.create_ash(em, engine, x, y);
                    }
                    if e_type == 2 {
                        self.create_ash1(em, engine, x, y);
                    }
                },
                _ => {}
            }

        }
    }
}

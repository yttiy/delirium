extern crate sdl2;
use engine::{ Engine, Screen, World };
use systems::{
    SpawnSystem,
    MapRenderSystem,
    PlayerSystem,
    RenderSystem,
    CollisionSystem,
    AnimationSystem,
    SpellSystem,
    SoundSystem,
    NPCSystem,
    NPCHealthRenderSystem,
    GUISystem,
    WeaponSystem
};
use std::rc::Rc;
use util::MapLoader;
use std::path::Path;
use util::Pref;

pub struct PlayScreen{
    world: World,
}

impl PlayScreen{
    pub fn new(mut engine: &mut Engine) -> PlayScreen {
        let mut world = World::new();

        let pref = Pref::open();
        let mut s  = "res/map/m.json";
        let lvl = pref.get_int("l");
        match lvl{
            1 => s = "res/map/map1.json",
            2 => s = "res/map/map2.json",
            3 => s = "res/map/map3.json",
            _ => {}
        }

        let map = Rc::new(MapLoader::new(s));
        world.add_system(SpawnSystem::new(engine, map.clone()));
        world.add_system(SoundSystem::new(lvl));
        world.add_system(AnimationSystem{});
        world.add_system(SpellSystem{});
        world.add_system(PlayerSystem{});
        world.add_system(WeaponSystem{});
        world.add_system(NPCSystem::new());
        world.add_system(CollisionSystem::new(map.clone(), 60.0));
        world.add_system(MapRenderSystem{layer: 0});
        world.add_system(MapRenderSystem{layer: 1});
        world.add_system(MapRenderSystem{layer: 2});
        world.add_system(RenderSystem{});
        world.add_system(NPCHealthRenderSystem{});
        world.add_system(GUISystem{});
        world.initialize_system(engine);

        PlayScreen{
            world: world,
        }
    }
}

impl Screen for PlayScreen{
    fn update(&mut self, engine: &mut Engine, dt: f32){
        self.world.update(engine, dt);
    }
}
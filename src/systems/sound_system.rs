extern crate sdl2;
use self::sdl2::mixer::Music;
use self::sdl2::mixer::Chunk;
use self::sdl2::mixer::Channel;
use std::path::Path;
use engine::{ Engine, System, Entity, Aspect, EntityManager, MessageManager, Message };
use util::Pref;

pub struct SoundSystem{
    theme: Music,
    spell: Chunk,
    robot: Chunk,
    robot1: Chunk,
    hit: Chunk,
    channel: Channel,
    channel01: Channel,
    channel02: Channel
}

impl SoundSystem{
    pub fn new(lvl: i32) -> SoundSystem{
        let mut path = "";
        match lvl{
            1 => path = "res/sound/kids_from_the_ocean_shores.ogg",
            2 => path = "res/sound/memories.ogg",
            3 => path = "res/sound/autumn.ogg",
            _ => {}
        }
        SoundSystem{
            theme: Music::from_file(Path::new(path)).unwrap(),
            spell: Chunk::from_file(Path::new("res/sound/spell.wav")).unwrap(),
            robot: Chunk::from_file(Path::new("res/sound/robot.ogg")).unwrap(),
            robot1: Chunk::from_file(Path::new("res/sound/robot1.ogg")).unwrap(),
            hit: Chunk::from_file(Path::new("res/sound/hit.ogg")).unwrap(),
            channel: self::sdl2::mixer::channel(1),
            channel01: self::sdl2::mixer::channel(2),
            channel02: self::sdl2::mixer::channel(3)
        }
    }
}

impl System for SoundSystem {
    fn aspect(&self) -> Aspect {
        Aspect::new()
    }

    fn init(&mut self, mm: &mut MessageManager, em: &mut EntityManager, engine: &mut Engine) {
        self.theme.play(-1);
        let mut p = Pref::open();
        let volume = p.get_int("volume");
        Music::set_volume(volume);
        self.channel.set_volume(volume);
        self.channel01.set_volume(volume);
        self.channel02.set_volume(volume);
    }

    fn receive(&mut self, mm: &MessageManager, em: &mut EntityManager, engine: &mut Engine, dt: f32){
        for m in mm.get().iter(){
            match *m{
                Message::PlaySound{s_id: s_id} => {
                    if s_id == 1 {
                        self.channel.play(&self.spell, 0);
                    }
                    if s_id == 2 {
                        self.channel01.play(&self.robot, 0);
                    }
                    if s_id == 3 {
                        self.channel02.play(&self.hit, 0);
                    }
                    if s_id == 4 {
                        if !self.channel01.is_playing(){
                            self.channel01.play(&self.robot1, 0);
                        }
                    }
                },
                _ => {}
            }
        }
    }
}
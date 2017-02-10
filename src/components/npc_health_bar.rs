extern crate sdl2;
use self::sdl2::pixels::Color;
use engine::Component;
use engine::Bar;
use engine::Engine;

pub struct NPCHealthBar{
    pub bar: Bar
}

impl NPCHealthBar{
    pub fn new(width: i32, height: i32, e: &mut Engine) -> NPCHealthBar{
        NPCHealthBar{
            bar: Bar::new(width, height, Color::RGB(255, 255, 255), Color::RGB(183, 47, 28), e)
        }
    }
}

impl Component for NPCHealthBar{}
extern crate sdl2;
use self::sdl2::pixels::Color;
use engine::Component;
use engine::{ Bar, UI, Engine, Image, TextInfo };
use std::path::Path;
use self::sdl2::render::Texture;

pub struct GUI{
    pub health: Bar,
    pub mana: Bar,
    pub background: Image,
    pub text: TextInfo
}

impl GUI{
    pub fn new(e: &mut Engine) -> GUI{
        let mut health = Bar::new(150, 8, Color::RGB(255, 255, 255), Color::RGB(183, 47, 28), e);
        health.set_position(20.0, 20.0);

        let mut mana = Bar::new(150, 8, Color::RGB(255, 255, 255), Color::RGB(107, 41, 228), e);
        mana.set_position(20.0, 40.0);

        let mut text = TextInfo::new(Path::new("res/fonts/font.ttf"), 32, Color::RGB(204, 240, 240), 5.0, e, vec![
            "Нажмите правую стрелку, о храбрый путник!",
            "Отлично! Теперь прыгните (стрелка вверх)",
            "Используйте заклинание - для этого нажмите 'E'",
            "Осторожность еще никому не помешала",
        ]);

        let wh = e.renderer.get_width_height();
        GUI{
            health: health,
            mana: mana,
            background: Image::new(e.renderer.create_rgb_texture(wh.0, 100, Color::RGB(45, 59, 79)).unwrap()),
            text: text
        }
    }

    pub fn set_info_position(&mut self, e: &mut Engine){
        let wh = e.renderer.get_width_height();
        let w = self.text.get_width();
        let h = self.text.get_height();
        self.text.set_position(wh.0 as f32 / 2.0 - w as f32 / 2.0, wh.1 as f32 - 50.0 - h as f32 / 2.0);
        self.background.set_position(0.0, wh.1 as f32 - 100.0);
    }
}

impl Component for GUI{}
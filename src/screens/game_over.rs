extern crate sdl2;
use engine::{ Engine, Screen, UI, Image };
use fsm::States;
use self::sdl2::pixels::Color;
use std::path::Path;

pub struct GameOverScreen{
    text: Image,
    text1: Image,
    yes_btn: Image,
    no_btn: Image,
}

impl GameOverScreen{
    pub fn new(engine: &mut Engine) -> GameOverScreen{
        let font = engine.ttf.load_font(Path::new("res/fonts/hardpixel.otf"), 30).unwrap();
        let font1 = engine.ttf.load_font(Path::new("res/fonts/font.ttf"), 23).unwrap();
        GameOverScreen{
            text: Image::new(engine.renderer.create_texture_from_font(&font, "Вы были сломлены, и теперь стали частью Делириума.", Color::RGB(255, 255, 255)).unwrap()),
            text1: Image::new(engine.renderer.create_texture_from_font(&font, "Загрузить последнее сохранение?", Color::RGB(255, 255, 255)).unwrap()),
            yes_btn: Image::new(engine.renderer.create_texture_from_font(&font1, "Да", Color::RGB(204, 240, 240)).unwrap()),
            no_btn: Image::new(engine.renderer.create_texture_from_font(&font1, "Нет", Color::RGB(204, 240, 240)).unwrap())
        }
    }
}

impl Screen for GameOverScreen{
    fn init(&mut self, engine: &mut Engine){
        engine.renderer.set_color(Color::RGB(19, 33, 42));

        let wh = engine.renderer.get_width_height();
        let text_w = self.text.get_width() as f32;
        let text_h = self.text.get_height() as f32;

        self.text.set_position(wh.0 as f32 / 2.0 - text_w / 2.0, wh.1 as f32 / 2.0 - 100.0);

        let text1_w = self.text1.get_width() as f32;
        //let text1_h = self.text1.get_height() as f32;
        self.text1.set_position(wh.0 as f32 / 2.0 - text1_w / 2.0, self.text.get_y() + text_h + 18.0);

        let yes_btn_w = self.yes_btn.get_width() as f32;
        self.yes_btn.set_position(wh.0 as f32 / 2.0 - 100.0 - yes_btn_w, self.text1.get_y() + text_h + 30.0);

        //let тщ_btn_w = self.yes_btn.get_width() as f32;
        self.no_btn.set_position(wh.0 as f32 / 2.0 + 100.0, self.text1.get_y() + text_h + 30.0);
    }

    fn update(&mut self, engine: &mut Engine, dt: f32){
        if self.yes_btn.is_click(engine){
            engine.state = States::Play;
        }

        if self.no_btn.is_click(engine){
            engine.state = States::Menu;
        }

        engine.renderer.set_ui_layer();
        self.text.draw(engine);
        self.text1.draw(engine);
        self.yes_btn.draw(engine);
        self.no_btn.draw(engine);
    }
}
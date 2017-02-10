extern crate sdl2;
use engine::{ Engine, Screen, UI, Image };
use self::sdl2::pixels::Color;
use self::sdl2::render::Texture;
use std::path::Path;
use fsm::States;

pub struct Intro{
    text: Image,
    loading: Image,
    inerval: f32,
    current_time: f32
}

impl Intro{
    pub fn new(mut engine: &mut Engine) -> Intro {
        let font = engine.ttf.load_font(Path::new("res/fonts/font1.ttf"), 27).unwrap();
        Intro{
            text: Image::new(engine.renderer.create_texture_from_font(&font, "Специально для Two Weeks Game 9", Color::RGB(255, 255, 255)).unwrap()),
            loading: Image::new(engine.renderer.create_texture_from_font(&font, "Загрузка...", Color::RGB(255, 255, 255)).unwrap()),
            inerval: 4.0,
            current_time: 0.0
        }
    }
}

impl Screen for Intro{
    fn init(&mut self, engine: &mut Engine){
        let wh = engine.renderer.get_width_height();
        let text_w = self.text.get_width() as f32;
        let text_h = self.text.get_height() as f32;
        self.text.set_position(wh.0 as f32 / 2.0 - text_w / 2.0, wh.1 as f32 / 2.0 - text_h / 2.0 - 30.0);

        let load_w = self.loading.get_width() as f32;
        self.loading.set_position(wh.0 as f32 / 2.0 - load_w / 2.0, self.text.get_y() + 44.0);
     }

    fn update(&mut self, engine: &mut Engine, dt: f32){
        self.current_time += dt;
        if self.current_time > self.inerval {
            engine.state = States::Menu;
        }
        engine.renderer.set_ui_layer();
        self.text.draw(engine);
        self.loading.draw(engine);
    }
}
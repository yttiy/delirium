extern crate sdl2;
use engine::{ Engine, Screen, UI, Image, MultilineText };
use self::sdl2::pixels::Color;
use self::sdl2::render::Texture;
use fsm::States;
use std::path::Path;
use self::sdl2::mixer::Music;
use util::Pref;

pub struct GameEndScreen{
    btn: Image,
    text: MultilineText,
    i: Image,
    sound: Music,
}

impl GameEndScreen{
    pub fn new(engine: &mut Engine) -> GameEndScreen{
        let mut btn: Image;
        {
            let font = engine.ttf.load_font(Path::new("res/fonts/font.ttf"), 23).unwrap();
            btn = Image::new(engine.renderer.create_texture_from_font(&font, "В меню", Color::RGB(204, 240, 240)).unwrap());
        }
        GameEndScreen{
            btn: btn,
            text: MultilineText::new(Path::new("res/fonts/hardpixel.otf"), 30, Color::RGB(255, 255, 255), engine, vec!["Вы очнулись лежащим ночью в поле.", "Было ли это все сном? Кто знает..."]),
            i: Image::new(engine.renderer.create_texture(Path::new("res/textures/e.png")).unwrap()),
            sound: Music::from_file(Path::new("res/sound/end.ogg")).unwrap(),
        }
    }
}

impl Screen for GameEndScreen{
    fn init(&mut self, engine: &mut Engine){
        engine.renderer.set_color(Color::RGB(19, 33, 42));
        engine.renderer.get_camera().set_zoom(0.32);
        engine.renderer.get_camera().set_central_position(48.0, 28.0);
        self.text.set_position(26.0, 20.0);
        let wh = engine.renderer.get_width_height();
        self.btn.set_position(wh.0 as f32 - 160.0, wh.1 as f32 - 40.0);

        let mut p = Pref::open();
        let volume = p.get_int("volume");

        Music::set_volume(volume);
        self.sound.play(-1);

    }

    fn update(&mut self, engine: &mut Engine, dt: f32){
        self.i.draw(engine);
        engine.renderer.set_ui_layer();
        self.text.draw(engine);
        self.btn.draw(engine);
        if self.btn.is_click(engine){
            engine.state = States::Menu;
        }
    }
}
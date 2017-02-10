extern crate sdl2;
use engine::{ Engine, Screen, UI, Image, MultilineText };
use self::sdl2::mixer::Music;
use self::sdl2::pixels::Color;
use self::sdl2::render::Texture;
use std::path::Path;
use fsm::States;
use self::sdl2::keyboard::Keycode;
use util::Pref;

pub struct PrologScreen{
    state: i32,
    sound: Music,
    btn: Image,
    text: MultilineText,
    i: Image,
    text1: MultilineText,
    i1: Image,
    text2: MultilineText,
    i2: Image,
    text3: MultilineText,
    i3: Image,
}

impl PrologScreen{
    pub fn new(mut engine: &mut Engine) -> PrologScreen{
        //
        let mut btn: Image;
        {
            let font = engine.ttf.load_font(Path::new("res/fonts/font.ttf"), 23).unwrap();
            btn = Image::new(engine.renderer.create_texture_from_font(&font, "Далее", Color::RGB(204, 240, 240)).unwrap());
        }

        PrologScreen{
            state: 0,
            btn: btn,
            sound: Music::from_file(Path::new("res/sound/dream.ogg")).unwrap(),
            text: MultilineText::new(Path::new("res/fonts/hardpixel.otf"), 30, Color::RGB(255, 255, 255), engine, vec!["Сколько лет уже странствую я по границам", "этих пустых и пугающих измерений?"]),
            i: Image::new(engine.renderer.create_texture(Path::new("res/textures/p.png")).unwrap()),
            text1: MultilineText::new(Path::new("res/fonts/hardpixel.otf"), 30, Color::RGB(255, 255, 255), engine, vec!["Существуем ли мы на самом деле, или являемся лишь", "призрачными тенями, которые породил Делириум?"]),
            i1: Image::new(engine.renderer.create_texture(Path::new("res/textures/p1.png")).unwrap()),
            text2: MultilineText::new(Path::new("res/fonts/hardpixel.otf"), 30, Color::RGB(255, 255, 255), engine, vec!["Стоя здесь, среди мертвых снов и видений, я обращаю", "свой тихий голос к Великому Океану, чей свет", "дает мне силы продолжать свой путь."]),
            i2: Image::new(engine.renderer.create_texture(Path::new("res/textures/p2.png")).unwrap()),
            text3: MultilineText::new(Path::new("res/fonts/hardpixel.otf"), 30, Color::RGB(255, 255, 255), engine, vec!["И лишь последняя надежда не дает мне раствориться", "в этой одурманивающей липкой тьме."]),
            i3: Image::new(engine.renderer.create_texture(Path::new("res/textures/p3.png")).unwrap()),
        }
    }
}

impl Screen for PrologScreen{
    fn init(&mut self, engine: &mut Engine){
        engine.renderer.set_color(Color::RGB(19, 33, 42));
        engine.renderer.get_camera().set_zoom(0.32);
        engine.renderer.get_camera().set_central_position(48.0, 28.0);

        self.text.set_position(26.0, 20.0);
        self.text1.set_position(26.0, 20.0);
        self.text2.set_position(26.0, 20.0);
        self.text3.set_position(26.0, 20.0);
        let wh = engine.renderer.get_width_height();
        self.btn.set_position(wh.0 as f32 - 135.0, wh.1 as f32 - 40.0);
        self.sound.play(-1);
    }

    fn update(&mut self, engine: &mut Engine, dt: f32){
        match self.state{
            0 => {
                self.i.draw(engine);
                engine.renderer.set_ui_layer();
                self.text.draw(engine);
                self.btn.draw(engine);
                if self.btn.is_click(engine){
                    self.state = 1;
                }
            },
            1 => {
                self.i1.draw(engine);
                engine.renderer.set_ui_layer();
                self.text1.draw(engine);
                self.btn.draw(engine);
                if self.btn.is_click(engine){
                    self.state = 2;
                }
            },
            2 => {
                self.i2.draw(engine);
                engine.renderer.set_ui_layer();
                self.text2.draw(engine);
                self.btn.draw(engine);
                if self.btn.is_click(engine){
                    self.state = 3;
                }
            },
            3 => {
                self.i3.draw(engine);
                engine.renderer.set_ui_layer();
                self.text3.draw(engine);
                self.btn.draw(engine);
                if self.btn.is_click(engine){
                    let mut p = Pref::open();
                    p.set_int("l", 1);
                    p.save();
                    engine.state = States::Play;
                }
            },
            _ => {}
        }
        /*self.i.draw(engine);
        engine.renderer.set_ui_layer();
        self.text.draw(engine);
        self.btn.draw(engine);
        if engine.event_manager.is_key_pressed(Keycode::Right){
            let mut p = Pref::open();
            p.set_int("l", 1);
            p.save();
            engine.state = States::Play;
        }*/
    }
}
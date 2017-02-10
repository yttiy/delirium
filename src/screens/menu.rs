extern crate sdl2;
use engine::{ Engine, Screen, UI, Image, Slider };
use self::sdl2::mixer::Music;
use self::sdl2::pixels::Color;
use self::sdl2::render::Texture;
use std::path::Path;
use fsm::States;
use util::Pref;

pub struct MenuScreen{
    magic: Texture,
    title: Image,
    play_btn: Image,
    continue_btn: Image,
    setting_btn: Image,
    exit_btn: Image,
    sound: Music,
    pref: Pref,
    t: bool,
    state: i32,
    setting_title: Image,
    slider: Slider,
    vol_text: Image,
    menu_btn: Image,
}

impl MenuScreen{
    pub fn new(mut engine: &mut Engine) -> MenuScreen {

        let mut slider: Slider;
        {
            slider = Slider::new(10.0, 10.0, 40, Color::RGB(255, 255, 255), engine);
        }

        let font_title = engine.ttf.load_font(Path::new("res/fonts/font.ttf"), 32).unwrap();
        let font_setting_title = engine.ttf.load_font(Path::new("res/fonts/font.ttf"), 27).unwrap();
        let font = engine.ttf.load_font(Path::new("res/fonts/font.ttf"), 25).unwrap();
        let font1 = engine.ttf.load_font(Path::new("res/fonts/hardpixel.otf"), 30).unwrap();
        let font4 = engine.ttf.load_font(Path::new("res/fonts/font.ttf"), 23).unwrap();
        let mut pref = Pref::open();
        let mut color = Color::RGB(204, 240, 240);
        let mut t = true;
        if pref.get_int("l") == 0 {
            color = Color::RGB(90, 124, 131);
            t = false;
        }

        MenuScreen{
            magic: engine.renderer.create_texture(Path::new("res/textures/magic.png")).unwrap(),
            title: Image::new(engine.renderer.create_texture_from_font(&font_title, "D E L I R I U M", Color::RGB(255, 255, 255)).unwrap()),
            play_btn: Image::new(engine.renderer.create_texture_from_font(&font, "Новая игра", Color::RGB(204, 240, 240)).unwrap()),
            continue_btn: Image::new(engine.renderer.create_texture_from_font(&font, "Продолжить", color).unwrap()),
            setting_btn: Image::new(engine.renderer.create_texture_from_font(&font, "Настройки", Color::RGB(204, 240, 240)).unwrap()),
            exit_btn: Image::new(engine.renderer.create_texture_from_font(&font, "Выход", Color::RGB(204, 240, 240)).unwrap()),
            sound: Music::from_file(Path::new("res/sound/menu1.ogg")).unwrap(),
            pref: pref,
            t: t,
            state: 0,
            setting_title: Image::new(engine.renderer.create_texture_from_font(&font_setting_title, "Настройки", Color::RGB(255, 255, 255)).unwrap()),
            slider: slider,
            vol_text: Image::new(engine.renderer.create_texture_from_font(&font1, "Громкость", Color::RGB(255, 255, 255)).unwrap()),
            menu_btn: Image::new(engine.renderer.create_texture_from_font(&font4, "В меню", Color::RGB(204, 240, 240)).unwrap())
        }
    }

    pub fn set_ui_position(&mut self, engine: &mut Engine){
        let wh = engine.renderer.get_width_height();

        let title_w = self.title.get_width() as f32;
        let title_h = self.title.get_height() as f32;

        self.title.set_position(wh.0 as f32 / 2.0 - title_w / 2.0, wh.1 as f32 / 2.0 - 180.0);

        let w = self.play_btn.get_width() as f32;
        let h = self.play_btn.get_height() as f32;

        self.play_btn.set_position(wh.0 as f32 / 2.0 - w / 2.0, self.title.get_y() + title_h + 50.0 + 10.0);

        let w1 = self.continue_btn.get_width() as f32;
        self.continue_btn.set_position(wh.0 as f32 / 2.0 - w1 / 2.0, self.play_btn.get_y() + h + 40.0);

        //let w2 = self.exit_btn.get_width() as f32;
        //self.exit_btn.set_position(wh.0 as f32 / 2.0 - w2 / 2.0, self.continue_btn.get_y() + h + 40.0);
        let w2 = self.setting_btn.get_width() as f32;
        self.setting_btn.set_position(wh.0 as f32 / 2.0 - w2 / 2.0, self.continue_btn.get_y() + h + 40.0);

        let w3 = self.exit_btn.get_width() as f32;
        self.exit_btn.set_position(wh.0 as f32 / 2.0 - w3 / 2.0, self.setting_btn.get_y() + h + 40.0);

        ///
        let s_title_w = self.setting_title.get_width() as f32;
        let s_title_h = self.setting_title.get_height() as f32;

        self.setting_title.set_position(wh.0 as f32 / 2.0 - s_title_w / 2.0, wh.1 as f32 / 2.0 - 170.0);

        self.vol_text.set_position(wh.0 as f32 / 2.0 - 170.0, self.title.get_y() + s_title_h + 50.0 + 10.0);
        self.slider.set_position(self.vol_text.get_x() + self.vol_text.get_width() as f32 + 10.0, self.vol_text.get_y() + 12.0);

        self.menu_btn.set_position(wh.0 as f32 - 160.0, wh.1 as f32 - 40.0);
    }

    pub fn update_menu(&mut self, engine: &mut Engine, dt: f32){
        if self.play_btn.is_click(engine){
            let mut p = Pref::open();
            p.set_int("l", 0);
            p.save();
            engine.state = States::Prolog;
        }

        if self.continue_btn.is_click(engine) && self.t {
            engine.state = States::Play;
        }

        if self.setting_btn.is_click(engine){
            self.state = 1;
        }

        if self.exit_btn.is_click(engine){
            engine.exit();
        }

        engine.renderer.draw(&self.magic, 0.0, 0.0);
        engine.renderer.set_ui_layer();
        self.title.draw(engine);
        self.play_btn.draw(engine);
        self.continue_btn.draw(engine);
        self.setting_btn.draw(engine);
        self.exit_btn.draw(engine);
    }

    pub fn update_setting(&mut self, engine: &mut Engine, dt: f32){
        engine.renderer.set_ui_layer();


        self.setting_title.draw(engine);
        self.slider.update(engine);
        self.slider.draw(engine);
        self.vol_text.draw(engine);
        self.menu_btn.draw(engine);
        Music::set_volume(self.slider.get_value());

        if self.menu_btn.is_click(engine){
            self.state = 0;
            let mut p = Pref::open();
            p.set_int("volume", self.slider.get_value());
            p.save();
        }
    }
}

impl Screen for MenuScreen{
    fn init(&mut self, engine: &mut Engine){
        engine.renderer.set_color(Color::RGB(19, 33, 42));
        {
            let mut cam = engine.renderer.get_camera();
            cam.set_zoom(0.37);
            let x = cam.get_width().clone();
            let y = cam.get_height().clone();
            cam.set_central_position(x as f32 / 2.0, y as f32 / 2.0);
        }

        self.set_ui_position(engine);
        let mut p = Pref::open();
        let volume = p.get_int("volume");

        self.slider.set_value(volume);
        Music::set_volume(volume);
        self.sound.play(-1);
    }

    fn update(&mut self, engine: &mut Engine, dt: f32){
        if self.state == 0 {
            self.update_menu(engine, dt);
        }else{
            self.update_setting(engine, dt);
        }
    }
}
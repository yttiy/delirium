extern crate sdl2;
use engine::Engine;
use self::sdl2::mouse::MouseButton;
use self::sdl2::render::Texture;
use self::sdl2::pixels::Color;
use self::sdl2::ttf::Font;
use std::path::Path;

pub trait UI{
    fn set_position(&mut self, x: f32, y: f32);
    fn draw(&self, engine: &mut Engine);
    fn get_x(&self) -> f32;
    fn get_y(&self) -> f32;
    fn get_width(&self) -> i32;
    fn get_height(&self) -> i32;
}

pub struct Image{
    texture: self::sdl2::render::Texture,
    x: f32,
    y: f32,
    width: i32,
    height: i32
}

impl Image {
    pub fn new(texture: Texture) -> Image {
        let q = texture.query();
        Image {
            texture: texture,
            x: 0.0, y: 0.0,
            width: q.width as i32,
            height: q.height as i32
        }
    }

    pub fn is_click(&self, engine: &mut Engine) -> bool{
        engine.event_manager.is_rect_click(MouseButton::Left, self.x as i32, self.y as i32, self.width, self.height)
    }
}

impl UI for Image{
    fn draw(&self, engine: &mut Engine){
        engine.renderer.draw(&self.texture, self.x, self.y);
    }

    fn set_position(&mut self, x: f32, y: f32){
        self.x = x;
        self.y = y;
    }

    fn get_x(&self) -> f32{
        self.x
    }

    fn get_y(&self) -> f32{
        self.y
    }

    fn get_width(&self) -> i32{
        self.width
    }

    fn get_height(&self) -> i32{
        self.height
    }
}

pub struct Bar{
    background: self::sdl2::render::Texture,
    bar: self::sdl2::render::Texture,
    x: f32,
    y: f32,
    width: i32,
    height: i32,
    bar_width: i32,
}

impl Bar{
    pub fn new(w: i32, h: i32, color: Color, color1: Color, engine: &mut Engine) -> Bar{
        Bar{
            background: engine.renderer.create_rgb_texture(w as u32, h as u32, color).unwrap(),
            bar: engine.renderer.create_rgb_texture(w as u32, h as u32, color1).unwrap(),
            x: 0.0,
            y: 0.0,
            width: w, height: h, bar_width: w,
        }
    }

    pub fn set_value(&mut self, max: i32, v: i32){
        /*
        if v > 100 || v < 0 {
            return;
        }
        self.bar_width = ((self.width as f32 / 100.0) * v as f32) as i32;*/
        if v > max || v < 0 {
            return;
        }
        self.bar_width = ((self.width as f32 / max as f32) * v as f32) as i32;
    }
}

impl UI for Bar{
    fn draw(&self, engine: &mut Engine){
        engine.renderer.draw(&self.background, self.x, self.y);
        if self.bar_width == 0 {
            return;
        }
        engine.renderer.draw_region(&self.bar, self.x, self.y, 0, 0, self.bar_width as u32, self.height as u32);
    }

    fn set_position(&mut self, x: f32, y: f32){
        self.x = x;
        self.y = y;
    }

    fn get_x(&self) -> f32{
        self.x
    }

    fn get_y(&self) -> f32{
        self.y
    }

    fn get_width(&self) -> i32{
        self.width
    }

    fn get_height(&self) -> i32{
        self.height
    }
}

pub struct TextInfo{
    textures: Vec<Texture>,
    current_text: i32,
    x: f32,
    y: f32,
    width: i32,
    height: i32,
    current_time: f32,
    interval: f32
}

impl TextInfo{
    pub fn new(path: &Path, font_size: u16, color: Color, interval: f32, mut engine: &mut Engine, n: Vec<&str>) -> TextInfo{
        let font = engine.ttf.load_font(path, font_size).unwrap();

        let mut textures: Vec<Texture> = Vec::new();
        for s in n.iter(){
            textures.push(engine.renderer.create_texture_from_font(&font, s, color).unwrap());
        }

        let q = textures.get(0).unwrap().query();

        TextInfo{
            textures: textures,
            current_text: -1,
            x: 0.0,
            y: 0.0,
            width: q.width as i32,
            height: q.height as i32,
            current_time: interval,
            interval: interval
        }
    }

    pub fn set_text(&mut self, n: i32){
        if n < self.textures.len() as i32{
            self.current_text = n;
            let q = self.textures.get(n as usize).unwrap().query();
            self.width = q.width as i32;
            self.height = q.height as i32;
            self.current_time = 0.0;
        }
    }

    pub fn update(&mut self, dt: f32){
        if self.current_time < self.interval{
            self.current_time += dt;
        }
    }

    pub fn is_active(&self) -> bool{
        if self.current_time < self.interval{
            return true;
        }
        false
    }
}

impl UI for TextInfo{
    fn draw(&self, engine: &mut Engine){
        if self.current_text > -1  && self.current_time < self.interval{
            engine.renderer.draw(&self.textures[self.current_text as usize], self.x, self.y);
        }
    }

    fn set_position(&mut self, x: f32, y: f32){
        self.x = x;
        self.y = y;
    }

    fn get_x(&self) -> f32{
        self.x
    }

    fn get_y(&self) -> f32{
        self.y
    }

    fn get_width(&self) -> i32{
        self.width
    }

    fn get_height(&self) -> i32{
        self.height
    }
}

pub struct MultilineText{
    textures: Vec<Texture>,
    x: f32,
    y: f32,
}

impl MultilineText{
    pub fn new(path: &Path, font_size: u16, color: Color, mut engine: &mut Engine, n: Vec<&str>) -> MultilineText{
        let font = engine.ttf.load_font(path, font_size).unwrap();
        let mut textures: Vec<Texture> = Vec::new();
        for s in n.iter(){
            textures.push(engine.renderer.create_texture_from_font(&font, s, color).unwrap());
        }
        MultilineText{
            textures: textures,
            x: 0.0,
            y: 0.0,
        }
    }
}

impl UI for MultilineText{
    fn draw(&self, engine: &mut Engine){
        let h = self.textures[0].query().height as f32;
        let mut n = 0.0;
        for t in self.textures.iter(){
            let mut y = self.y + n * h;
            if n != 0.0 {
                y += 10.0;
            }
            engine.renderer.draw(&t, self.x, y);
            n += 1.0;
        }
    }

    fn set_position(&mut self, x: f32, y: f32){
        self.x = x;
        self.y = y;
    }

    fn get_x(&self) -> f32{
        self.x
    }

    fn get_y(&self) -> f32{
        self.y
    }

    fn get_width(&self) -> i32{
        0
    }

    fn get_height(&self) -> i32{
        0
    }
}

/////

pub struct Slider{
    x: f32,
    slide_x: f32,
    y: f32,
    width: f32,
    height: f32,
    value: i32,
    line: Texture,
    slide: Texture
}

impl Slider{
    pub fn new(x: f32, y: f32, value: i32, color: Color, engine: &mut Engine) -> Slider{
        Slider{
            x: x,
            y: y,
            slide_x: x,
            width: 150.0,
            height: 20.0,
            value: value,
            line: engine.renderer.create_rgb_texture(150, 5, color).unwrap(),
            slide: engine.renderer.create_rgb_texture(5, 20, color).unwrap(),
        }
    }

    pub fn update(&mut self, engine: &mut Engine){
        if engine.event_manager.is_mouse_left_pressed(){
            if engine.event_manager.is_mouse_to_rect(self.x as i32 - 5, self.y as i32 - 3, self.width as i32 + 5, self.height as i32 + 3){
                self.slide_x = engine.event_manager.get_mouse_x() as f32;
                if self.slide_x < self.x {
                    self.slide_x = self.x;
                }
                if self.slide_x > self.x + self.width - 5.0 {
                    self.slide_x = self.x + self.width - 5.0;
                }

                self.value = ((self.slide_x - self.x) / ((self.width - 5.0) / 100.0)) as i32;
                if self.value > 100 {
                    self.value = 100;
                }
                if self.value < 0 {
                    self.value = 0;
                }
            }
        }
    }

    pub fn get_value(&self) -> i32{
        self.value
    }

    pub fn set_value(&mut self, mut v: i32){
        if v > 100 {
            v = 100;
        }
        if v < 0 {
            v = 0;
        }
        self.value = v;
        self.slide_x = self.x + (self.width - 5.0) / 100.0 * v as f32;
    }
}


impl UI for Slider{
    fn draw(&self, engine: &mut Engine){
        engine.renderer.draw(&self.line, self.x, self.y + 8.0);
        engine.renderer.draw(&self.slide, self.slide_x, self.y);
    }

    fn set_position(&mut self, x: f32, y: f32){
        self.x = x;
        self.y = y;
        let n = self.value.clone();
        self.set_value(n);
    }

    fn get_x(&self) -> f32{
        self.x
    }

    fn get_y(&self) -> f32{
        self.y
    }

    fn get_width(&self) -> i32{
        self.width as i32
    }

    fn get_height(&self) -> i32{
        self.height as i32
    }
}
extern crate sdl2;
use std::path::Path;
use self::sdl2::image::{LoadTexture, INIT_PNG, INIT_JPG};
//use self::sdl2::image::{LoadTexture, INIT_PNG };
use self::sdl2::keyboard::Keycode;
use self::sdl2::render::Texture;
use self::sdl2::rect::Rect;
use self::sdl2::pixels::{ Color, PixelMasks };
use engine::{ Camera, Config };

use std::collections::HashMap;
use self::sdl2::render::TextureValueError;

pub struct Renderer<'a>{
    context: self::sdl2::Sdl,
    renderer: self::sdl2::render::Renderer<'a>,
    view: Rect,
    gui: Rect,
    camera: Camera,
    window_width: u32,
    window_height: u32,
    texture_rect: Rect,
    texture_position: Rect,
    r_state: i32
}

impl<'a> Renderer<'a>{
    pub fn new(config: &Config) -> Renderer<'a>{
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let _image_context = sdl2::image::init(INIT_PNG | INIT_JPG).unwrap();
        let _image_context = sdl2::image::init(INIT_PNG).unwrap();
        let window = match config.window_resizable {
            true => video_subsystem.window(config.title, config.width, config.height)
                .position_centered()
                .resizable()
                .build()
                .unwrap(),
            false => video_subsystem.window(config.title, config.width, config.height)
                .position_centered()
                .build()
                .unwrap()
        };

        let mut renderer = match config.vsync {
            true => window.renderer().accelerated().present_vsync().build().unwrap(),
            false => window.renderer().accelerated().build().unwrap(),
        };
        let mut view = Rect::new(0, 0, config.width, config.height);
        let mut gui = Rect::new(0, 0, config.width, config.height);

        Renderer{
            context: sdl_context,
            renderer: renderer,
            view: view,
            gui: gui,
            camera: Camera::new((config.width as f32) / 2.0,
                (config.height as f32) / 2.0,
                config.width,
                config.height),
            window_width: config.width,
            window_height: config.height,
            texture_rect: Rect::new(0, 0, 0, 0),
            texture_position: Rect::new(0, 0, 0, 0),
            r_state: 0
        }
    }
    pub fn create_texture_from_font(&mut self, font: &self::sdl2::ttf::Font, text: &str, color: Color)-> Result<Texture, TextureValueError>{
        let surface = font.render(text).blended(color).unwrap();
        self.renderer.create_texture_from_surface(&surface)
    }

    pub fn create_rgb_texture(&mut self, w: u32, h: u32, color: Color) -> Result<Texture, TextureValueError>{
        let mut surface = self::sdl2::surface::Surface::from_pixelmasks(w, h, PixelMasks{bpp: 32, rmask: 0, gmask: 0, bmask: 0, amask: 0}).unwrap();
        surface.fill_rect(None, color);
        self.renderer.create_texture_from_surface(&surface)
    }

    pub fn create_texture(&mut self, filename: &Path) -> Result<Texture, String>{
        self.renderer.load_texture(filename)
    }

    pub fn get_context(&mut self) -> &self::sdl2::Sdl{
        &self.context
    }


    pub fn set_color(&mut self, color: Color){
        self.renderer.set_draw_color(color);
    }
	
    /////////////////////
	
    pub fn render_begin(&mut self){
        self.update_view();

        self.renderer.clear();
        self.r_state = 0;
    }

    fn update_view(&mut self){
        let sw = (self.window_width as f32) / (self.camera.get_width() as f32);
        let sh = (self.window_height as f32) / (self.camera.get_height() as f32);
        if sw < sh {
            self.view.set_width(self.window_width);
            self.view.set_height((sw * (self.camera.get_height() as f32)) as u32);
        }else{
            self.view.set_width((sh * (self.camera.get_width() as f32)) as u32);
            self.view.set_height(self.window_height);
        }

        let x = ((self.window_width as f32) / 2.0 - (self.view.width()  as f32) / 2.0) as i32;
        let y = ((self.window_height as f32) / 2.0 - (self.view.height()  as f32) / 2.0) as i32;
        self.view.set_x(x);
        self.view.set_y(y);

        self.renderer.set_viewport(Some(self.view));
    }

    pub fn draw(&mut self, texture: &Texture, x: f32, y: f32){
        let q = texture.query();
        self.draw_region(texture, x, y, 0, 0, q.width, q.height);
    }

    pub fn draw_region(&mut self, texture: &Texture, x: f32, y: f32, tex_x: i32, tex_y: i32, tex_w: u32, tex_h: u32){
        self.texture_rect.set_x(tex_x);
        self.texture_rect.set_y(tex_y);
        self.texture_rect.set_width(tex_w);
        self.texture_rect.set_height(tex_h);

        if self.r_state == 0 {
			//используются виртуальные координаты, т.к sdl с float не работает, вроде
			
            let sw = (self.view.width() as f32) / (self.camera.get_width() as f32);
            let sh = (self.view.height() as f32) / (self.camera.get_height() as f32);

            let plane_x = (sw * (x - self.camera.get_x())) as i32;//.round()
            let plane_y = (sh * (y - self.camera.get_y())) as i32;
            let plane_width = (sw * (tex_w as f32)) as u32 + 1;
            let plane_height = (sh * (tex_h as f32)) as u32 + 1;

            self.texture_position.set_x(plane_x);
            self.texture_position.set_y(plane_y);
            self.texture_position.set_width(plane_width);
            self.texture_position.set_height(plane_height);
        }else{
            self.texture_position.set_x(x as i32);
            self.texture_position.set_y(y as i32);
            self.texture_position.set_width(tex_w);
            self.texture_position.set_height(tex_h);
        }

        self.renderer.copy(&texture, Some(self.texture_rect), Some(self.texture_position));
    }

    pub fn set_ui_layer(&mut self){
        if self.r_state != 1{
            self.r_state = 1;
            self.renderer.set_viewport(Some(self.gui));
        }
    }

    pub fn render_end(&mut self){
        self.renderer.present();
    }

    /////////////////////

    pub fn resize(&mut self){
        let size = self.renderer.output_size().unwrap();
        self.window_width = size.0;
        self.window_height = size.1;

        self.gui.set_width(self.window_width);
        self.gui.set_height(self.window_height);
    }

    pub fn get_width_height(&self) -> (u32, u32){
        self.renderer.output_size().unwrap()
    }

    pub fn set_camera(&mut self, c: Camera){
        self.camera = c;
    }

    pub fn get_camera(&mut self) -> &mut Camera{
        &mut self.camera
    }
}

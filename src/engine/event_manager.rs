extern crate sdl2;
use self::sdl2::EventPump;
use self::sdl2::event::{Event, WindowEvent };
use self::sdl2::keyboard::Keycode;
use std::collections::HashSet;
use self::sdl2::render::Texture;
use self::sdl2::mouse::MouseButton;
use self::sdl2::mouse::MouseState;

pub struct EventManager{
    events: EventPump,
    run: bool,
    resize: bool,
    prev_keys: HashSet<Keycode>,
    new_keys: HashSet<Keycode>,
    key_m: MouseButton,
}

impl EventManager{
    pub fn new(context: &self::sdl2::Sdl) -> EventManager{
        EventManager{
            events: context.event_pump().unwrap(),
            run: true,
            resize: false,
            prev_keys: HashSet::new(),
            new_keys: HashSet::new(),
            key_m: MouseButton::Unknown,
        }
    }

    pub fn update(&mut self, w: u32, h: u32){
        self.resize = false;
        self.key_m = MouseButton::Unknown;

        for event in self.events.poll_iter() {
            match event {
                Event::Quit {..} => self.run = false,
                Event::Window {win_event: WindowEvent::Resized(w, h), ..} => self.resize = true,
                Event::MouseButtonDown { mouse_btn: mouse_btn, ..} => {
                    self.key_m = mouse_btn;
                },
                _ => ()
            }
        }

        self.keys_update();

    }

    pub fn is_mouse_left_pressed(&self) -> bool{
        if self.events.mouse_state().is_mouse_button_pressed(MouseButton::Left){
            return true
        }
        false
    }

    pub fn get_mouse_x(&self) -> i32{
        self.events.mouse_state().x()
    }

    pub fn get_mouse_y(&self) -> i32{
        self.events.mouse_state().y()
    }

    pub fn stop_run(&mut self){
        self.run = false;
    }

    fn keys_update(&mut self){
        let keys: HashSet<Keycode>  = self.events.keyboard_state().pressed_scancodes().filter_map(Keycode::from_scancode).collect();
        self.new_keys = &keys - &self.prev_keys;
        self.prev_keys = keys;
    }

    pub fn is_window_resize(&self) -> bool{
        self.resize
    }

    pub fn is_key_down(&self, key: Keycode) -> bool{
        for i in &self.new_keys {
            if *i == key {
                return true;
                break;
            }
        }
        false
    }

    pub fn is_key_pressed(&self, key: Keycode) -> bool{
        for i in &self.prev_keys {
            if *i == key {
                return true;
                break;
            }
        }
        false
    }

    pub fn is_run(&self) -> bool{
        self.run
    }

    pub fn is_rect_click(&self, mb: MouseButton, x: i32, y: i32, w: i32, h: i32) -> bool{
        if mb == self.key_m {
            let state = self.events.mouse_state();
            let m_x = state.x();
            let m_y = state.y();
            if m_x >= x && m_x <= x + w && m_y >= y && m_y <= y + h {
                return true;
            }
        }
        false
    }

    pub fn is_mouse_to_rect(&self, x: i32, y: i32, w: i32, h: i32) -> bool{
        let state = self.events.mouse_state();
        let m_x = state.x();
        let m_y = state.y();
        if m_x >= x && m_x <= x + w && m_y >= y && m_y <= y + h {
            return true;
        }
        false
    }
}
extern crate delirium as org;
extern crate winapi;
extern crate user32;
extern crate kernel32;
use org::engine::{ App, Config };
use std::ptr;

fn hide_console_window() {
    let window = unsafe {
        kernel32::GetConsoleWindow()
    };
    if window != ptr::null_mut() {
        unsafe {
            user32::ShowWindow (window, winapi::SW_HIDE)
        };
    }
}

fn main() {
    hide_console_window();
    let mut config = Config::new();
    config.fps_limit = 150;
    config.vsync = true;
    config.window_resizable = false;
    config.width = 960;
    config.height = 540;
    config.output_fps = true;
    let mut app = App::new(config);
    app.start();
}
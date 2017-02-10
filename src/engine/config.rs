pub struct Config{
    pub width: u32,
    pub height: u32,
    pub title: &'static str,
    pub vsync: bool,
    pub fps_limit: i32,
    pub output_fps: bool,
    pub window_resizable: bool
}


impl Config{
    pub fn new() -> Config{
        Config{
            width: 800,
            height: 600,
            title: "App",
            vsync: true,
            output_fps: false,
            fps_limit: 1000,
            window_resizable: false
        }
    }
}
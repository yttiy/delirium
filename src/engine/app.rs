extern crate sdl2;
use self::sdl2::keyboard::Keycode;
use self::sdl2::mixer::{ INIT_MP3, INIT_FLAC, INIT_MOD, INIT_FLUIDSYNTH, INIT_MODPLUG, INIT_OGG, AUDIO_S16LSB };
//use self::sdl2::mixer::{ INIT_OGG, AUDIO_S16LSB };
use engine::{ Engine, Screen, Config };
use fsm::fsm;

struct DefaultScreen{}

impl DefaultScreen{
    fn new() -> DefaultScreen{
        DefaultScreen{}
    }
}

impl Screen for DefaultScreen{}

/////////

pub struct App{
    screen: Box<Screen>,
    config: Config
}

impl App{

    pub fn new(config: Config) -> App{
        App{
            screen: Box::new(DefaultScreen::new()),
            config: config
        }
    }

    pub fn start(&mut self){
        let mut engine = Engine::new(&self.config);

        //init audio
        let frequency = 44100;
        let format = AUDIO_S16LSB; // signed 16 bit samples, in little-endian byte order
        let channels = 2; // Stereo
        let chunk_size = 1024;
        let _ = sdl2::mixer::open_audio(frequency, format, channels, chunk_size).unwrap();
        sdl2::mixer::allocate_channels(8);
        ///

        let mut timer = engine.renderer.get_context().timer().unwrap();
        let mut old_time = 0u32;
        let mut current_time = 0u32;
        let mut delta_time = 0.0;
        let mut second = 0.0;
        let mut fps = 0;
        let interval = (1000 / self.config.fps_limit) as f32;

        while engine.event_manager.is_run() {
            old_time = current_time;
            current_time = timer.ticks();
            delta_time = (current_time - old_time) as f32 / 1000.0;

            if delta_time > 0.5{
                delta_time = 0.0;
            }
            if self.config.output_fps{
                if second < 1.0 {
                    second += delta_time;
                    fps += 1;
                }else{
                    second = 0.0;
                    println!("Ootput FPS: {}", fps);
                    fps = 0;
                }
            }
            if delta_time < interval {
                timer.delay((interval - delta_time) as u32);
            }
            //                timer.delay(interval - dt);
            let win_size = engine.renderer.get_width_height();
            engine.event_manager.update(win_size.0, win_size.1);
            if engine.event_manager.is_window_resize() {
                engine.renderer.resize();
            }
            engine.renderer.render_begin();

            fsm(&mut self.screen, &mut engine, delta_time);
            engine.renderer.render_end();
        }
    }
}



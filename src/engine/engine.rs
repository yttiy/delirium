extern crate sdl2;
use self::sdl2::ttf::Sdl2TtfContext;
use engine::{ Renderer, EventManager, Config };
use fsm::{ States, start_state };

pub struct Engine<'a>{
    pub renderer: Renderer<'a>,
    pub state: States,
    pub event_manager: EventManager,
    pub ttf: Sdl2TtfContext
}

impl<'a> Engine<'a>{
    pub fn new(config: &Config) -> Engine<'a>{
        let mut renderer = Renderer::new(config);
        let mut e = EventManager::new(&renderer.get_context());

        Engine{
            renderer: renderer,
            state: start_state,
            event_manager: e,
            ttf: sdl2::ttf::init().unwrap()
        }
    }

    pub fn exit(&mut self){
        self.event_manager.stop_run();
    }
}
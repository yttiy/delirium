use engine::{ Engine, Screen };
use screens::{ MenuScreen, PlayScreen, GameOverScreen, GameEndScreen, Intro, PrologScreen };

pub enum States{
    None,
    Intro,
    Menu,
    Prolog,
    Play,
    GameOver,
    GameEnd
}

pub const start_state: States = States::Intro;

pub fn fsm(screen: &mut Box<Screen>, mut engine: &mut Engine, dt: f32){
    match engine.state {
        States::Intro => {
            *screen = Box::new(Intro::new(&mut engine));
            {
                screen.init(&mut engine);
            }

            engine.state = States::None;
        },
        States::Menu => {
            *screen = Box::new(MenuScreen::new(&mut engine));
            {
                screen.init(&mut engine);
            }

            engine.state = States::None;
        },
        States::Prolog => {
            *screen = Box::new(PrologScreen::new(&mut engine));
            {
                screen.init(&mut engine);
            }

            engine.state = States::None;
        },
        States::Play => {
            *screen = Box::new(PlayScreen::new(&mut engine));
            engine.state = States::None;
        },
        States::GameOver => {
            *screen = Box::new(GameOverScreen::new(&mut engine));
            {
                screen.init(&mut engine);
            }

            engine.state = States::None;
        },
        States::GameEnd => {
            *screen = Box::new(GameEndScreen::new(&mut engine));
            {
                screen.init(&mut engine);
            }

            engine.state = States::None;
        }
        States::None => screen.update(engine, dt),
    }
}
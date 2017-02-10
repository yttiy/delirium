use engine::Component;

pub struct Body {
    pub dx: f32,
    pub dy: f32,
    pub on_ground: bool,
    pub is_static: bool,
}

impl Body{
    pub fn new(dx: f32, dy: f32, is_static: bool) -> Body{
        Body{
            dx: dx,
            dy: dy,
            on_ground: false,
            is_static: is_static,
            //vector: 0,
        }
    }

    /*pub fn new_static() -> Body{
        Body{
            dx: 0.0,
            dy: 0.0,
            on_ground: false,
            is_static: true,
            vector: 0
        }
    }

    pub fn new_static1(v: i32) -> Body{
        Body{
            dx: 0.0,
            dy: 0.0,
            on_ground: false,
            is_static: true,
            vector: v
        }
    }*/
}

impl Component for Body{}
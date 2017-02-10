extern crate sdl2;
use self::sdl2::render::Texture;
use engine::Component;
use std::rc::Rc;

pub struct Drawable{
    pub tex: Rc<Texture>,
    pub tex_x: i32,
    pub tex_y: i32,
    pub tex_w: u32,
    pub tex_h: u32
}

impl Drawable{
    pub fn new(t: Rc<Texture>) -> Drawable{
        let q = &t.query();
        Drawable{
            tex: t,
            tex_x: 0,
            tex_y: 0,
            tex_w: q.width,
            tex_h: q.height,
        }
    }

}

impl Component for Drawable{}
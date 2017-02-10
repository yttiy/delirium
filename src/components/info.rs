use engine::Component;

pub struct Info{
    pub id: i32
}

impl Info{
    pub fn new(id: i32) -> Info{
        Info{
            id: id
        }
    }
}

impl Component for Info{}
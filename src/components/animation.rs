use engine::Component;

pub struct Animation{
    pub speed: f32,
    current_time: f32,
    current_anim: usize,
    column: i32,
    row: i32,
    pub animations: Vec<Vec<i32>>,
}

impl Animation{
    pub fn new(column: i32, row: i32, speed: f32) -> Animation{
        Animation{
            column: column,
            row: row,
            speed: speed,
            current_time: 0.0,
            current_anim: 0,
            animations: Vec::new(),
        }
    }

    pub fn column(&self) -> i32{
        self.column
    }

    pub fn row(&self) -> i32{
        self.row
    }

    pub fn get_current_time(&self) -> f32{
        self.current_time
    }

    pub fn update_time(&mut self, dt: f32){
        self.current_time += self.speed * dt;
        if self.current_time > self.animations.get(self.current_anim).unwrap().len() as f32 {
            self.current_time = 0.0;
        }
    }

    pub fn get_frame(&mut self, dt: f32) -> i32{
        self.current_time += self.speed * dt;
        if self.current_time >= self.animations.get(self.current_anim).unwrap().len() as f32 {
            self.current_time = 0.0;
        }
        self.animations.get(self.current_anim).unwrap().get(self.current_time as usize).unwrap().clone()
    }

    /*
    pub fn get_frame(&self) -> i32{
        self.animations.get(self.current_anim).unwrap().get(self.current_time as usize).unwrap().clone()
    }
    */

    pub fn add_animation(&mut self, anim: Vec<i32>){
        self.animations.push(anim);
    }

    pub fn set_animation(&mut self, a: usize){
        if self.current_anim != a {
            self.current_anim = a;
            self.current_time = 0.0;
        }
    }
}

impl Component for Animation{}
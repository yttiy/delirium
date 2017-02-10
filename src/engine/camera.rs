pub struct Camera{
    x: f32,
    y: f32,
    width: u32,
    height: u32,
    base_width: u32,
    base_height: u32
}

impl Camera{
    pub fn new(x: f32, y: f32, width: u32, height: u32) -> Camera{
        Camera{
            x: x - (width as f32) / 2.0,
            y: y - (height as f32) / 2.0,
            width: width,
            height: height,
            base_width: width,
            base_height: height,
        }
    }

    pub fn set_central_position(&mut self, x: f32, y: f32){
        self.x = x - (self.width as f32) / 2.0;
        self.y = y - (self.height as f32) / 2.0;
    }

    pub fn get_x(&self) -> f32{
        self.x
    }

    pub fn get_y(&self) -> f32{
        self.y
    }

    pub fn get_width(&self) -> u32{
        self.width
    }

    pub fn get_height(&self) -> u32{
        self.height
    }

    pub fn set_zoom(&mut self, zoom: f32){
        self.width = ((self.base_width as f32) * zoom) as u32;
        self.height = ((self.base_height as f32) * zoom) as u32;
    }
}
use components::{ Map, Drawable };
use engine::{ Engine, System, Entity, Aspect, EntityManager };

pub struct MapRenderSystem{
    pub layer: i32
}

impl System for MapRenderSystem{
    fn aspect(&self) -> Aspect{
        Aspect::new()
            .add::<Map>()
    }

    fn process(&mut self, id: &i32, em: &mut EntityManager, engine: &mut Engine, dt: f32){

        let mut entity = em.get_by_id(id);
        let m = entity.get_component::<Map>();

        //ограничиваем отрисовку карты для увеличения производительности
        //по X
        let mut x_start = 0;
        let mut x_end = 0;
        {
            let cam = engine.renderer.get_camera();
            x_start = cam.get_x() as i32 / m.tile_width - 3;
            x_end = (cam.get_x() + cam.get_width() as f32) as i32 / m.tile_width + 2;
        }
        if x_start < 0 {
            x_start = 0;
        }
        if x_end > m.column {
            x_end = m.column;
        }
        //по Y
        let mut y_start = 0;
        let mut y_end = 0;
        {
            let cam = engine.renderer.get_camera();
            y_start = cam.get_y() as i32 / m.tile_height - 3;
            y_end = (cam.get_y() + cam.get_height() as f32) as i32 / m.tile_height + 2;
        }
        if y_start < 0 {
            y_start = 0;
        }
        if y_end > m.row {
            y_end = m.row;
        }


        for i in y_start..y_end {
            for j in x_start..x_end {
                let mut tile = m.map[self.layer as usize][(m.column * i + j) as usize];
                if tile != 0{
                    tile -= 1;
                    let pos_y = (tile / m.texture_column) * m.tile_height;
                    let pos_x = (tile - m.texture_column * (tile / m.texture_column)) * m.tile_width;
                    engine.renderer.draw_region(&m.texture,
                        (j * m.tile_width) as f32,
                        (i * m.tile_height) as f32,
                        pos_x,
                        pos_y,
                        (m.tile_width) as u32,
                        (m.tile_height) as u32);
                }
            }
        }

    }
}

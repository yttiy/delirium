extern crate sdl2;
use self::sdl2::render::Texture;
use std::path::Path;
use engine::{ Engine, Component };
use util::MapLoader;
use std::rc::Rc;

pub struct Map{
    pub texture: Texture,
    pub map: Vec<Vec<i32>>,
    pub tile_width: i32,
    pub tile_height: i32,
    pub column: i32,
    pub row: i32,
    pub texture_column: i32,
    pub texture_row: i32
}

impl Map{
    pub fn new(map: Rc<MapLoader>, engine: &mut Engine) -> Map{
        let texture = engine.renderer.create_texture(Path::new(&map.texture_path)).unwrap();
        Map{
            map: map.map.clone(),
            texture: texture,
            tile_width:    map.tile_width,
            tile_height: map.tile_height,
            column: map.column,
            row: map.row,
            texture_column: map.texture_column,
            texture_row: map.texture_row
        }
    }
}

impl Component for Map{}
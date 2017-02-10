extern crate json;
extern crate sdl2;
use self::sdl2::rect::Rect;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use engine::{ Engine };
use std::collections::HashMap;

pub struct MapObject{
    pub m_type: String,
    pub x: f32,
    pub y: f32,
    pub width: i32,
    pub height: i32,
}

pub struct MapLoader{
    pub texture_path: String,
    pub map: Vec<Vec<i32>>,
    pub objects: Vec<Vec<MapObject>>,
    pub tile_width: i32,
    pub tile_height: i32,
    pub column: i32,
    pub row: i32,
    pub texture_column: i32,
    pub texture_row: i32,
}

impl MapLoader{
    pub fn new(s: &str) -> MapLoader{
        let mut map: Vec<Vec<i32>> = Vec::new();
        let mut objects: Vec<Vec<MapObject>> = Vec::new();

        let mut file = File::open(s).unwrap();
        let mut file_data = String::new();
        file.read_to_string(&mut file_data).unwrap();
        let parsed = json::parse(&file_data).unwrap();
        let layers = &parsed["layers"];

        let mut map_n = 0;
        let mut object_n = 0;

        for i in 0..layers.len() {
            if layers[i]["type"] == "tilelayer" {
                map.push(Vec::new());
                let data = &layers[i]["data"];
                for j in 0..data.len() {
                    map[map_n].push(data[j].as_i32().unwrap());
                }
                map_n += 1;
            }else if layers[i]["type"] == "objectgroup"{
                objects.push(Vec::new());
                let data = &layers[i]["objects"];
                for j in 0..data.len() {
                    objects[object_n].push(
                        MapObject{
                            m_type: data[j]["type"].as_str().unwrap().to_string(),
                            x:  data[j]["x"].as_f32().unwrap(),
                            y: data[j]["y"].as_f32().unwrap(),
                            width: data[j]["width"].as_i32().unwrap(),
                            height: data[j]["height"].as_i32().unwrap()
                        });
                }
                object_n += 1;
            }
        }

        let tileset = &parsed["tilesets"][0]["image"].as_str().unwrap();
        let texture_path = s.to_string() + &"/../".to_string() + &tileset.to_string();

        let tile_width = parsed["tilewidth"].as_i32().unwrap();
        let tile_height = parsed["tileheight"].as_i32().unwrap();

        let column = parsed["width"].as_i32().unwrap();
        let row = parsed["height"].as_i32().unwrap();

        let texture_column = &parsed["tilesets"][0]["imagewidth"].as_i32().unwrap() / tile_width;
        let texture_row = &parsed["tilesets"][0]["imageheight"].as_i32().unwrap() / tile_height;

        MapLoader{
            map: map,
            objects: objects,
            texture_path: texture_path.to_string(),
            tile_width:    tile_width,
            tile_height: tile_height,
            column: column,
            row: row,
            texture_column: texture_column,
            texture_row: texture_row
        }
    }
}
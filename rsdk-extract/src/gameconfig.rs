use byteorder::{LittleEndian, ReadBytesExt};
use serde::Serialize;
use std::collections::HashMap;
use std::io::Read;

#[derive(Serialize)]
pub(crate) struct GameConfig {
    title: String,
    description: String,
    palettes: Vec<(u8, u8, u8)>,
    objects: HashMap<String, String>, // todo: rename to scripts
    globals: HashMap<String, u32>,
}

pub(crate) fn extract(mut buffer: &[u8]) -> GameConfig {
    let mut game_config = GameConfig {
        title: read_rsdk_string(&mut buffer),
        description: read_rsdk_string(&mut buffer),
        palettes: read_palettes(&mut buffer, 0x60),
        objects: HashMap::new(),
        globals: HashMap::new(),
    };

    // read_pallettes(&mut buffer, 8);
    let object_count = buffer.read_u8().unwrap();
    info!("{} objects detected.", object_count);
    let str_objects: Vec<String> = read_rsdk_strings(&mut buffer, object_count);
    let str_object_paths: Vec<String> = read_rsdk_strings(&mut buffer, object_count);

    for index in 0..(object_count as usize) {
        game_config
            .objects
            .insert(str_objects[index].clone(), str_object_paths[index].clone());
    }

    let object_count = buffer.read_u8().unwrap();
    info!("{} globals detected.", object_count);

    for _index in 0..(object_count as usize) {
        let name = read_rsdk_string(&mut buffer);
        let value = buffer.read_u32::<LittleEndian>().unwrap();
        info!("{}: {}", name, value);
        game_config.globals.insert(name, value);
    }

    game_config
}

fn read_rsdk_string(buffer: &mut &[u8]) -> String {
    let size = buffer.read_u8().unwrap();
    let string: Vec<u8> = read_bytes(buffer, size);
    std::str::from_utf8(&string).unwrap().into()
}

fn read_bytes(buffer: &mut &[u8], size: u8) -> Vec<u8> {
    let buf: &mut Vec<u8> = &mut vec![0u8; size as usize];
    buffer.read_exact(buf).expect("read_bytes");
    buf.clone()
}

fn read_palettes(buffer: &mut &[u8], count: u8) -> Vec<(u8, u8, u8)> {
    let mut palettes = vec![];
    for _ in 0..count {
        palettes.push((
            buffer.read_u8().unwrap(),
            buffer.read_u8().unwrap(),
            buffer.read_u8().unwrap(),
        ));
    }
    palettes
}

fn read_rsdk_strings(buffer: &mut &[u8], count: u8) -> Vec<String> {
    let mut strings = vec![];
    for _i in 0..count {
        strings.push(read_rsdk_string(buffer));
    }
    strings
}

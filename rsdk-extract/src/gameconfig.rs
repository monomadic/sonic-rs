use byteorder::ReadBytesExt;
// use serde::Deserialize;
use std::io::Read;

// #[derive(Deserialize)]
pub(crate) struct GameConfig {}

pub(crate) fn extract(mut buffer: &[u8]) {
    let title = read_rsdk_string(&mut buffer);
    let description = read_rsdk_string(&mut buffer);

    println!("{:?}", [&title, &description]);

    // master palette
    read_palettes(&mut buffer, 0x60);
    // read_pallettes(&mut buffer, 8);
    let object_count = buffer.read_u8().unwrap();
    println!("object_count: {}", object_count);
    let objects = read_rsdk_strings(&mut buffer, object_count);
    let object_paths = read_rsdk_strings(&mut buffer, object_count);

    for (index, object) in objects.iter().enumerate() {
        println!("{}: {}", object, object_paths[index]);
    }
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

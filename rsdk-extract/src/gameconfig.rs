use byteorder::{LittleEndian, ReadBytesExt};
use std::fs::File;
use std::io::Read;
use std::path::Path;

type ExtractResult<T> = Result<T, Box<dyn std::error::Error>>;

pub fn read<P: AsRef<Path>>(path: P) -> ExtractResult<()> {
    // let mut offset = 0usize;
    let mut buffer = Vec::new();
    let mut file = File::open(path)?;

    file.read_to_end(&mut buffer)?;
    extract(&mut buffer);
    Ok(())
}

pub(crate) fn extract(mut buffer: &[u8]) {
    let size = buffer.read_u8().unwrap();
    println!("string: {}", size);

    let title = read_bytes(&mut buffer, size);

    let size = buffer.read_u8().unwrap();
    let description = read_bytes(&mut buffer, size);

    println!(
        "string: {} {}",
        std::str::from_utf8(&title).unwrap(),
        std::str::from_utf8(&description).unwrap(),
    );
}

fn read_bytes(mut buffer: &[u8], size: u8) -> Vec<u8> {
    let buf: &mut Vec<u8> = &mut vec![0u8; size as usize];
    buffer.read_exact(buf).expect("read_bytes");
    buf.clone()
}

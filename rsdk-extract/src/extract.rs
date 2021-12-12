use byteorder::{LittleEndian, ReadBytesExt};
use std::fs::File;
use std::io::Read;
use std::path::Path;

type ExtractResult<T> = Result<T, Box<dyn std::error::Error>>;

pub fn load<P: AsRef<Path>>(path: P) -> ExtractResult<()> {
    // let mut offset = 0usize;
    let mut buffer = Vec::new();
    let mut file = File::open(path)?;

    file.read_to_end(&mut buffer)?;
    header(&mut buffer);
    Ok(())
}

/** Read header data (8 bytes) */
fn header(mut buffer: &[u8]) {
    let dictionary = crate::dictionary::generate();

    let whole_file = &buffer.clone();
    let mut magic_number = [0; 4];
    buffer.read_exact(&mut magic_number).unwrap();
    println!(
        "Magic Number: {}",
        std::str::from_utf8(&magic_number).unwrap()
    );

    let mut version = [0; 2];
    buffer.read_exact(&mut version).unwrap();
    println!("Version: {}", std::str::from_utf8(&version).unwrap());

    match version[1] {
        66 => println!("RSDKv4 Detected (Sonic the Hedgehog 1 and Sonic the Hedgehog 2 remasters)"),
        _ => println!("Unknown or unsupported RSDK version."),
    }

    let file_count = buffer.read_u16::<LittleEndian>().unwrap();

    println!("File count: {}", file_count);

    for _index in 0..file_count {
        let md5 = [
            buffer.read_u32::<LittleEndian>().unwrap(),
            buffer.read_u32::<LittleEndian>().unwrap(),
            buffer.read_u32::<LittleEndian>().unwrap(),
            buffer.read_u32::<LittleEndian>().unwrap(),
        ];

        let md5sum = md5.iter().map(|h| format!("{:08x}", h)).collect::<String>();

        // let md5hash = ::md5::Digest::from(md5);
        // println!("hashhh {:x}", md5);

        let offset = buffer.read_u32::<LittleEndian>().unwrap();
        let filesize_encoded = buffer.read_u32::<LittleEndian>().unwrap();
        let encrypted: bool = (filesize_encoded & 0x80000000) == 0x80000000;
        let filesize = filesize_encoded & 0x7FFFFFFF;
        // println!(
        //     "Checksum: {} Offset: {} Size: {} Encrypted: {}",
        //     md5sum, offset, filesize, encrypted
        // );

        // write stupid file
        let file = &whole_file[(offset as usize)..((filesize + offset) as usize)];

        // println!("Searching for {:?} {:?}", &md5sum, dictionary.get(&*md5sum),);

        let filename: &str = dictionary.get(&*md5sum).unwrap_or(&md5sum);
        let suffix = if encrypted { ".encrypted" } else { "" };

        let output_path = format!("output/{}{}", &filename, suffix);
        println!("Writing: {}", output_path);

        // swallow error
        let mut path = std::path::PathBuf::from(&output_path);
        path.pop();
        let _ = std::fs::create_dir_all(&path);

        use std::io::Write;
        let mut file_buffer = std::fs::File::create(output_path).unwrap();
        file_buffer.write_all(&file).unwrap();
    }

    // LittleEndian::read_u16(load_part(2)) as usize;
}

// /** Read a file index record (20 bytes) () */
// fn read_index(mut buffer: &[u8]) {
//     let mut checksumbuffer = [0; 16];
//     let checksum = buffer.read_exact(&mut checksumbuffer).unwrap();
//     let offset = buffer.read_u32::<LittleEndian>().unwrap();
//     let filesize_encoded = buffer.read_u32::<LittleEndian>().unwrap();
//     let encrypted: bool = (filesize_encoded & 0x80000000) == 0x80000000;
//     let filesize = filesize_encoded & 0x7FFFFFFF;

//     println!(
//         "Checksum: {:?}\nOffset: {}\nSize: {}\nEncrypted: {}",
//         checksum, offset, filesize, encrypted
//     );
// }

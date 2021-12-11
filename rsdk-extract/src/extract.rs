use byteorder::{LittleEndian, ReadBytesExt};
use md5;
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
    let hashed_filenames = crate::hashlist::md5hashes();

    let whole_file = &buffer.clone();
    let mut magic_number = [0; 4];
    buffer.read_exact(&mut magic_number).unwrap();
    println!(
        "Magic Number: {}",
        std::str::from_utf8(&magic_number).unwrap()
    );

    let version = buffer.read_u16::<LittleEndian>().unwrap();
    println!("Version: {}", version);

    let file_count = buffer.read_u16::<LittleEndian>().unwrap();

    println!("File count: {}", file_count);

    for index in 0..file_count {
        // read_index(&mut buffer);
        // let mut md5 = [0; 16];
        // buffer.read_exact(&mut md5).unwrap();
        // this should read as [u32;4] instead
        let md5 = [
            buffer.read_u32::<LittleEndian>().unwrap(),
            buffer.read_u32::<LittleEndian>().unwrap(),
            buffer.read_u32::<LittleEndian>().unwrap(),
            buffer.read_u32::<LittleEndian>().unwrap(),
        ];

        let md5sum = md5.iter().map(|h| format!("{:x}", h)).collect::<String>();

        let offset = buffer.read_u32::<LittleEndian>().unwrap();
        let filesize_encoded = buffer.read_u32::<LittleEndian>().unwrap();
        let encrypted: bool = (filesize_encoded & 0x80000000) == 0x80000000;
        let filesize = filesize_encoded & 0x7FFFFFFF;
        println!(
            "Checksum: {} Offset: {} Size: {} Encrypted: {}",
            md5sum, offset, filesize, encrypted
        );

        if let Some(realname) = hashed_filenames.get(&*md5sum) {
            println!("FOUND: {}", realname);
        }

        if !encrypted || true {
            // write stupid file
            let file = &whole_file[(offset as usize)..((filesize + offset) as usize)];

            // println!(
            //     "file: {}",
            //     std::str::from_utf8(&file[0..4]).unwrap_or("err")
            // );

            use std::io::Write;
            let mut file_buffer = std::fs::File::create(format!("output/{}", md5sum)).unwrap();
            file_buffer.write_all(&file).unwrap();
        }
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

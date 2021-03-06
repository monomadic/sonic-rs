use anyhow::Result;
use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Read;

/** Read rsdkv4 data */
pub(crate) fn rsdkv4(
    mut buffer: &[u8],
    output_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let dictionary = crate::dictionary::generate();

    // todo: convert to binread with Cursor, this is bad
    let whole_file = <&[u8]>::clone(&buffer);
    let mut magic_number = [0; 6];
    buffer.read_exact(&mut magic_number).unwrap();
    let magic_number = std::str::from_utf8(&magic_number).unwrap();
    println!("Magic Number: {}", magic_number);

    match magic_number {
        "RSDKvB" => {
            println!("RSDKvB Detected (Sonic 1 and Sonic 2 Mobile Remakes)")
        }
        _ => panic!("Unknown or unsupported RSDK version."),
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

        let offset = buffer.read_u32::<LittleEndian>().unwrap();
        let filesize_encoded = buffer.read_u32::<LittleEndian>().unwrap();
        let encrypted: bool = (filesize_encoded & 0x80000000) == 0x80000000;
        let filesize = filesize_encoded & 0x7FFFFFFF;

        let filename: &str = dictionary.get(&*md5sum).unwrap_or(&md5sum);
        let suffix = "";

        let output_path = format!("{}{}{}", &output_path, &filename, suffix);

        info!(
            "Writing: {} Offset: {} Size: {} Encrypted: {}",
            output_path, offset, filesize, encrypted
        );

        let filebuffer = &whole_file[(offset as usize)..((filesize + offset) as usize)];
        let mut file: Vec<u8> = Vec::from(filebuffer);

        if encrypted {
            file = decrypt(&file, file.len() as u32);

            println!(
                "Decrypted: {} Offset: {} Size: {} Encrypted: {}",
                output_path, offset, filesize, encrypted
            );
        }

        // swallow error
        let mut path = std::path::PathBuf::from(&output_path);
        path.pop();

        log::info!("creating dir {:?}", &path);
        std::fs::create_dir_all(&path)?;

        use std::io::Write;
        log::info!("writing {:?}", &output_path);
        let mut file_buffer = std::fs::File::create(output_path)?;
        file_buffer.write_all(&file)?;
    }

    generate_eload_keys(4388);

    Ok(())
}

static ENC_KEY_2: u32 = 0x24924925;
static ENC_KEY_1: u32 = 0xAAAAAAAB;

/** XOR-based crypt */
fn decrypt(bytes: &[u8], filesize: u32) -> Vec<u8> {
    let mut tmp_byte: u32;
    let (key1, key2) = generate_eload_keys(filesize);

    let mut e_string_no: u32 = (filesize / 4) & 0x7F; // encrypted string number?
    let mut e_string_pos_a = 0_usize;
    let mut e_string_pos_b = 8_usize;

    let mut e_nibbleswap = 0_u32;

    let mut return_data: Vec<u8> = Vec::with_capacity(filesize as usize);

    let mut temp1;
    let mut temp2;

    print!("filesize {} e_string_no {} ", filesize, e_string_no);

    for (_byte_position, byte) in bytes.iter().enumerate() {
        if e_string_pos_b >= key2.len() {
            panic!(
                "index ({}) out of bounds: key2 is {:?}",
                e_string_pos_b, key2
            );
        };

        tmp_byte = e_string_no ^ (key2[e_string_pos_b] as u32);
        // print!("tmp_byte {:X} ", tmp_byte);
        tmp_byte ^= *byte as u32;
        // print!("tmp_byte {:X} {:X} ", tmp_byte, bytes[0]);
        if e_nibbleswap == 1 {
            // swap nibbles: 0xAB <-> 0xBA
            tmp_byte = ((tmp_byte << 4) + (tmp_byte >> 4)) & 0xFF;
        }
        tmp_byte ^= key1[e_string_pos_a] as u32;
        // println!("tmp_byte {:x}", tmp_byte as u8);
        return_data.push(tmp_byte as u8);

        e_string_pos_a += 1;
        e_string_pos_b += 1;

        if e_string_pos_a <= 0x0F {
            if e_string_pos_b > 0x0C {
                e_string_pos_b = 0;
                e_nibbleswap ^= 0x01;
            }
        } else if e_string_pos_b <= 0x08 {
            e_string_pos_a = 0;
            e_nibbleswap ^= 0x01;
        } else {
            e_string_no += 2;
            e_string_no &= 0x7F;
            if e_nibbleswap != 0 {
                let key1 = mul_unsigned_high(ENC_KEY_1, e_string_no as i32);
                let key2 = mul_unsigned_high(ENC_KEY_2, e_string_no as i32);
                e_nibbleswap = 0;
                let tmpkey1: u32 = u32::from_be_bytes([key1[0], key1[1], key1[2], key1[3]]);
                let tmpkey2: u32 = u32::from_be_bytes([key2[0], key2[1], key2[2], key2[3]]);
                temp1 = tmpkey2 + (e_string_no - tmpkey2) / 2; // convert vec<u8> to u32??
                temp2 = tmpkey1 / 8 * 3;
                e_string_pos_a = (e_string_no - temp1 / 4 * 7) as usize;
                e_string_pos_b = (e_string_no - temp2 * 4 + 2) as usize;
            } else {
                let key1 = mul_unsigned_high(ENC_KEY_1, e_string_no as i32);
                let key1: u32 = u32::from_be_bytes([key1[0], key1[1], key1[2], key1[3]]);
                let key2 = mul_unsigned_high(ENC_KEY_2, e_string_no as i32);
                let key2: u32 = u32::from_be_bytes([key2[0], key2[1], key2[2], key2[3]]);
                e_nibbleswap = 1;
                temp1 = key2 + (e_string_no - key2) / 2;
                temp2 = key1 / 8 * 3;
                e_string_pos_b = (e_string_no - temp1 / 4 * 7) as usize;
                e_string_pos_a = (e_string_no - temp2 * 4 + 3) as usize;
            }
        }
    }

    return_data
}

// fn as_u32_le(array: &[u8; 4]) -> u32 {
//     ((array[0] as u32) << 0)
//         + ((array[1] as u32) << 8)
//         + ((array[2] as u32) << 16)
//         + ((array[3] as u32) << 24)
// }

fn mul_unsigned_high(a: u32, b: i32) -> Vec<u8> {
    ((((a as i64) * (b as i64)) >> 32_i64) as i32)
        .to_be_bytes()
        .to_vec()
}

fn generate_eload_keys(filesize: u32) -> ([u8; 16], [u8; 16]) {
    let arg1 = filesize;
    let arg2 = (filesize >> 1) + 1;

    (generate_key(arg1), generate_key(arg2))
}

fn generate_key(i: u32) -> [u8; 16] {
    let checksum = md5::compute(i.to_string());

    use std::io::Cursor;
    let mut cursor = Cursor::new(checksum.to_vec());

    let boxed_slice = [
        cursor.read_u32::<LittleEndian>().unwrap().to_be_bytes(),
        cursor.read_u32::<LittleEndian>().unwrap().to_be_bytes(),
        cursor.read_u32::<LittleEndian>().unwrap().to_be_bytes(),
        cursor.read_u32::<LittleEndian>().unwrap().to_be_bytes(),
    ]
    .concat()
    .into_boxed_slice();

    let boxed_array: Box<[u8; 16]> = boxed_slice.try_into().unwrap();
    *boxed_array
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_generate_eload_keys() {
        assert_eq!(
            generate_eload_keys(4388),
            (
                [
                    0xF0, 0x03, 0x38, 0x47, 0x7D, 0xD7, 0xEB, 0xF2, 0xDA, 0x60, 0xEE, 0x83, 0x81,
                    0xF3, 0x61, 0xAA
                ],
                [
                    0xCD, 0x6E, 0x5F, 0x8C, 0x23, 0xEB, 0xA0, 0x29, 0x0C, 0x19, 0x59, 0x44, 0xDD,
                    0x16, 0x1C, 0xA5
                ]
            )
        );
    }

    #[test]
    fn test_decrypt() {
        assert_eq!(
            decrypt(&[0xEE, 0x63, 0x75, 0x05, 0xBB], 4388),
            vec![0x5B, 0x30, 0x5D, 0x4F, 0x52]
        );

        // PNG for Data/Game/Menu/Amazon.png
        assert_eq!(
            decrypt(&[0x86, 0x79, 0x4D, 0xF0], 130991),
            vec![0x89, 0x50, 0x4E, 0x47]
        );

        // PNG for Data/Game/Menu/ArrowButtons.png
        assert_eq!(
            decrypt(
                &[
                    0xC0, 0x44, 0x5E, 0x3F, 0x31, 0xFF, 0x30, 0x7C, 0x5F, 0x4A, 0x7E, 0x09, 0x76,
                    0x75, 0xC8, 0x3D, 0xE4, 0x07, 0x0E, 0x59, 0xB0, 0xE4, 0xA7, 0xB2, 0x2A, 0xEF,
                    0x8E, 0x15, 0x4B, 0x4A, 0x60, 0xD2
                ],
                11014
            ),
            vec![
                0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00, 0x00, 0x00, 0x0D, 0x49, 0x48,
                0x44, 0x52, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x08, 0x06, 0x00, 0x00,
                0x00, 0x5C, 0x72, 0xA8,
            ]
        )
    }
}

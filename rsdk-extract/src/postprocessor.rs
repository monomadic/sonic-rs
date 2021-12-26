use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Write;

const TILELAYER_CHUNK_W: u32 = 0x100;
const TILELAYER_CHUNK_H: u32 = 0x100;
const TILELAYER_CHUNK_MAX: u32 = 0x10000;

pub(crate) fn run(resource_dir: &str) -> std::io::Result<()> {
    // let path = format!("{}/Data/Game/GameConfig.bin", resource_dir);
    // info!("reading {}", path);
    // let file = std::fs::read(path)?;
    // let config = crate::gameconfig::extract(&file);
    // let json = serde_json::to_string(&config).unwrap();
    // let output_path = format!("{}/Data/Game/GameConfig.json", resource_dir);
    // info!("writing {}", output_path);
    // std::fs::write(output_path, json)?;

    process_act_file(&format!("{}Data/Stages/Zone01/Act1", resource_dir))?;
    process_act_file(&format!("{}Data/Stages/Zone01/Act2", resource_dir))?;
    process_act_file(&format!("{}Data/Stages/Zone01/Act3", resource_dir))?;

    process_tiles(&format!("{}Data/Stages/Zone01/128x128Tiles", resource_dir))?;

    Ok(())
}

// fn process_stage_config(path: &str) -> std::io::Result<()> {
//     let file = std::fs::read(path)?;
// }

// #[allow(arithmetic_overflow)]
fn process_tiles(input: &str) -> std::io::Result<()> {
    use std::ops::Shl;
    let output = format!("{}.txt", input);
    let input = format!("{}.bin", input);
    let tilefile = format!("{}.tiles.txt", input);
    let mut buffer: &[u8] = &*std::fs::read(input)?;
    let mut file = std::fs::File::create(output).unwrap();
    let mut tilefile = std::fs::File::create(tilefile).unwrap();
    info!("Read {} bytes.", buffer.len());

    let chunks = [0; 511]; // 512 total chunks

    for _ in 0..511 {
        for _x in 0..7 {
            // 16 x 16 grid within a chunk
            for _y in 0..7 {
                let mut entry = [buffer.read_u8()?, buffer.read_u8()?, buffer.read_u8()?];
                entry[0] -= (entry[0] >> 6) << 6;

                write!(&mut file, "visualPlane:{},", entry[0] >> 4);
                entry[0] -= 16 * (entry[0] >> 4);

                write!(&mut file, "direction:{},", entry[0] >> 4);
                entry[0] -= 4 * (entry[0] >> 2);

                let entry_0_shifted = entry[0].checked_shl(8).unwrap_or(0) as u16;
                let tile_16x16: u16 = entry_0_shifted + entry[1] as u16;
                write!(&mut file, "tile_16x16:{},", tile_16x16);
                write!(&mut tilefile, "{:3} ", tile_16x16);

                let collision_flags: [u8; 2] = [entry[2] >> 4, entry[2] - ((entry[2] >> 4) << 4)];
                write!(&mut file, "collisionFlags:{:?} ", collision_flags);
            }
            write!(&mut file, "\n");
            // write!(&mut tilefile, " | ");
        }
        write!(&mut file, "\n");
    }

    Ok(())
}

fn process_act_file(input: &str) -> std::io::Result<()> {
    let output = format!("{}.txt", input);
    let input = format!("{}.bin", input);

    info!("reading {}", input);
    let mut buffer: &[u8] = &*std::fs::read(input)?;
    info!("read {} bytes", buffer.len());

    info!("writing {}", output);
    let mut file = std::fs::File::create(output).unwrap();

    writeln!(&mut file, "title={}", read_rsdk_string(&mut buffer))?;

    for index in 0..4 {
        let line = format!("ActiveLayer{}={}", index, buffer.read_u8()?);
        writeln!(&mut file, "{}", line)?;
    }

    // for key in ["tile_layer_midpoint", "stage_width", "stage_height"] {
    //     writeln!(&mut file, "{}", format!("{}={}", key, buffer.read_u8()?))?;
    // }

    writeln!(
        &mut file,
        "{}",
        format!("Midpoint={} # tile layer midpoint", buffer.read_u8()?)
    )?;
    let stage_width: u16 = buffer.read_u16::<LittleEndian>()?;
    let stage_height: u16 = buffer.read_u16::<LittleEndian>()?;

    writeln!(
        &mut file,
        "stage_width={} # stage width in chunks",
        stage_width
    )?;
    writeln!(
        &mut file,
        "stage_height={} # stage width in chunks",
        stage_height
    )?;

    writeln!(
        &mut file,
        "xBoundary1={} # the starting X Boundary (always 0)",
        0
    )?;
    writeln!(
        &mut file,
        "yBoundary1={} # the starting Y Boundary (always 0)",
        0
    )?;
    writeln!(
        &mut file,
        "xBoundary2={} # the ending X Boundary, it's the value (in pixels) for the stage width",
        stage_width << 7
    )?;
    writeln!(
        &mut file,
        "yBoundary2={} # the ending Y Boundary, it's the value (in pixels) for the stage height",
        stage_height << 7
    )?;
    writeln!(&mut file, "waterLevel={}", (stage_height << 7) + 128)?;

    // let mut map = vec![vec![0; stage_height as usize]; stage_width as usize];

    info!("Level map with {} blocks", (stage_height * stage_width));
    write!(&mut file, "\n\n")?;
    for _y in 0..stage_height {
        for _x in 0..stage_width {
            // 128x128 Block number is 16-bit Little-Endian in RSDKv4
            let block_number: u16 = buffer.read_u16::<LittleEndian>()?;

            // map[x as usize][y as usize] = block_number;
            write!(&mut file, "{:3} ", block_number)?;
        }
        write!(&mut file, "\n\n")?;
    }

    let object_count = buffer.read_u16::<LittleEndian>()?;
    info!("Found {} objects", object_count);

    for _ in 0..object_count {
        let object_attribs: u16 = buffer.read_u16::<LittleEndian>()?;
        let object_type: u8 = buffer.read_u8()?;
        let object_subtype: u8 = buffer.read_u8()?;
        let object_xpos: u32 = buffer.read_u32::<LittleEndian>()?;
        let object_ypos: u32 = buffer.read_u32::<LittleEndian>()?;

        writeln!(&mut file, "type={}", object_type);
        writeln!(&mut file, "subtype={}", object_subtype);
        writeln!(&mut file, "xpos={}", object_xpos);
        writeln!(&mut file, "ypos={}", object_ypos);

        if object_attribs & 0x1 != 0x0 {
            writeln!(&mut file, "state={}", buffer.read_u32::<LittleEndian>()?);
        }
        if object_attribs & 0x2 != 0x0 {
            writeln!(&mut file, "direction={}", buffer.read_u8()?);
        }
        if object_attribs & 0x4 != 0x0 {
            writeln!(&mut file, "scale={}", buffer.read_u32::<LittleEndian>()?);
        }
        if object_attribs & 0x8 != 0x0 {
            writeln!(&mut file, "rotation={}", buffer.read_u32::<LittleEndian>()?);
        }
        if object_attribs & 0x10 != 0x0 {
            writeln!(&mut file, "drawOrder={}", buffer.read_u8()?);
        }
        if object_attribs & 0x20 != 0x0 {
            writeln!(&mut file, "priority={}", buffer.read_u8()?);
        }
        if object_attribs & 0x40 != 0x0 {
            writeln!(&mut file, "alpha={}", buffer.read_u8()?);
        }
        if object_attribs & 0x80 != 0x0 {
            writeln!(&mut file, "animation={}", buffer.read_u8()?);
        }
        if object_attribs & 0x100 != 0x0 {
            writeln!(
                &mut file,
                "animationSpeed={}",
                buffer.read_u32::<LittleEndian>()?
            );
        }
        if object_attribs & 0x200 != 0x0 {
            writeln!(&mut file, "frame={}", buffer.read_u8()?);
        }
        if object_attribs & 0x400 != 0x0 {
            writeln!(&mut file, "inkEffect={}", buffer.read_u8()?);
        }
        if object_attribs & 0x800 != 0x0 {
            writeln!(&mut file, "values_1={}", buffer.read_u32::<LittleEndian>()?);
        }
        if object_attribs & 0x1000 != 0x0 {
            writeln!(&mut file, "values_2={}", buffer.read_u32::<LittleEndian>()?);
        }
        if object_attribs & 0x2000 != 0x0 {
            writeln!(&mut file, "values_3={}", buffer.read_u32::<LittleEndian>()?);
        }
        if object_attribs & 0x4000 != 0x0 {
            writeln!(&mut file, "values_4={}", buffer.read_u32::<LittleEndian>()?);
        }
        write!(&mut file, "\n")?;
    }

    if buffer.len() > 0 {
        error!(
            "{} bytes leftover in file! {:?}",
            buffer.len(),
            &buffer[0..4]
        );
    }

    Ok(())
}

fn read_rsdk_string(buffer: &mut &[u8]) -> String {
    let size = buffer.read_u8().unwrap();
    let string: Vec<u8> = read_bytes(buffer, size);
    std::str::from_utf8(&string).unwrap().into()
}

fn read_bytes(buffer: &mut &[u8], size: u8) -> Vec<u8> {
    use std::io::Read;
    let buf: &mut Vec<u8> = &mut vec![0u8; size as usize];
    buffer.read_exact(buf).expect("read_bytes");
    buf.clone()
}

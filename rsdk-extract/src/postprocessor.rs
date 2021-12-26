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

    Ok(())
}

// fn process_stage_config(path: &str) -> std::io::Result<()> {
//     let file = std::fs::read(path)?;
// }

fn process_act_file(input: &str) -> std::io::Result<()> {
    let output = format!("{}.txt", input);
    let input = format!("{}.bin", input);

    info!("reading {}", input);
    let mut buffer: &[u8] = &*std::fs::read(input)?;

    info!("writing {}", output);
    let mut file = std::fs::File::create(output).unwrap();
    let title = read_rsdk_string(&mut buffer);
    writeln!(&mut file, "title={}", title)?;

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

    write!(&mut file, "\n\n")?;
    for _y in 0..(stage_height - 1) {
        for _x in 0..(stage_width - 1) {
            // 128x128 Block number is 16-bit Little-Endian in RSDKv4
            let block_number: u16 = buffer.read_u16::<LittleEndian>()?;

            // map[x as usize][y as usize] = block_number;
            write!(&mut file, "{:3} ", block_number)?;
        }
        write!(&mut file, "\n\n")?;
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

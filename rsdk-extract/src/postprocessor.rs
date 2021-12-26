use byteorder::{LittleEndian, ReadBytesExt};

pub(crate) fn run(resource_dir: &str) -> std::io::Result<()> {
    let path = format!("{}/Data/Game/GameConfig.bin", resource_dir);
    info!("reading {}", path);
    let file = std::fs::read(path)?;
    let config = crate::gameconfig::extract(&file);
    let json = serde_json::to_string(&config).unwrap();
    let output_path = format!("{}/Data/Game/GameConfig.json", resource_dir);
    info!("writing {}", output_path);
    std::fs::write(output_path, json)?;

    process_act_file(&format!("{}/Data/Stages/Zone01/Act1.bin", resource_dir))?;

    Ok(())
}

// fn process_stage_config(path: &str) -> std::io::Result<()> {
//     let file = std::fs::read(path)?;
// }

fn process_act_file(path: &str) -> std::io::Result<()> {
    info!("reading {}", path);
    let file = std::fs::read(path)?;
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

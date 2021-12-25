pub(crate) fn run(resource_dir: &str) -> std::io::Result<()> {
    let path = format!("{}/Data/Game/GameConfig.bin", resource_dir);
    info!("reading {}", path);
    let file = std::fs::read(path)?;
    let config = crate::gameconfig::extract(&file);
    let json = serde_json::to_string(&config).unwrap();
    let output_path = format!("{}/Data/Game/GameConfig.json", resource_dir);
    info!("writing {}", output_path);
    std::fs::write(output_path, json)?;

    Ok(())
}

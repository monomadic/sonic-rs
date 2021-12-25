pub(crate) fn run() -> std::io::Result<()> {
    info!("reading resources/Data/Game/GameConfig.bin");
    let file = std::fs::read("resources/Data/Game/GameConfig.bin")?;
    let config = crate::gameconfig::extract(&file);
    let json = serde_json::to_string(&config).unwrap();
    info!("writing resources/Data/Game/GameConfig.json");
    std::fs::write("resources/Data/Game/GameConfig.json", json)?;

    Ok(())
}

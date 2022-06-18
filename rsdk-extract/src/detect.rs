pub(crate) fn game_version(md5: &str) -> Option<&'static str> {
    match md5 {
        "2881d2492be3ba5d3b6106cdbf82c3e5" => Some("sonic"),
        _ => None,
    }
}

pub(crate) fn output_path(buffer: &[u8]) -> String {
    // detect version with md5 sum
    let md5 = format!("{:x}", ::md5::compute(&buffer));
    let version = crate::detect::game_version(&md5);
    if let Some(version) = version {
        info!("Md5 checksum match: {}", version);
    }
    format!(
        "resources/{}",
        &version
            .map(|version| format!("{}/", version))
            .unwrap_or_else(|| String::from("")),
    )
}

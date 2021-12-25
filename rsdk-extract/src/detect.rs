pub(crate) detect_version(md5: &str) -> Option<String>) {
    match md5 {
        "2881d2492be3ba5d3b6106cdbf82c3e5" => Some("sonic"),
        _ => None,
    }
}
#[derive(Debug, Default)]
pub struct RSDKContainer {
    files: Vec<RSDKFile>,
}

#[derive(Debug, Default)]
pub struct RSDKFile {
    hash: String,
}

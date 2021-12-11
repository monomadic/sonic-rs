#[derive(Debug, Default)]
pub struct RSDKContainer {
    pub files: Vec<FileDescriptionBlock>,
}

#[derive(Debug, Default)]
pub struct FileDescriptionBlock {
    // pub md5: u32[], // 4 x u32
    pub offset: u32, // absolute offset
    pub size: u32,
    // pub encrypted: bool,
}

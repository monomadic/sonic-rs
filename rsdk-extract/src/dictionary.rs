use md5;
use std::collections::HashMap;

static dictionary: &str = include_str!("dictionary.txt");

pub(crate) fn generate() -> HashMap<String, String> {
    dictionary
        .lines()
        .map(|filename| {
            (
                format!("{:x}", md5::compute(filename.to_lowercase())),
                String::from(filename),
            )
        })
        .collect::<HashMap<String, String>>()
}

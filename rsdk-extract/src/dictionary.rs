use md5;
use std::collections::HashMap;

static DICTIONARY: &str = include_str!("dictionary.txt");

pub(crate) fn generate() -> HashMap<String, String> {
    println!("Generating {} checksums...", DICTIONARY.lines().count());
    DICTIONARY
        .lines()
        .map(|filename| {
            // let md5sum = format!("{:?}", md5::compute(filename.to_lowercase()));

            // println!(
            //     "Hashed {} as {:?} {:?}",
            //     filename.to_lowercase(),
            //     md5sum,
            //     md5::compute(filename.to_lowercase())
            // );
            (
                format!("{:x}", md5::compute(filename.to_lowercase())),
                String::from(filename),
            )
        })
        .collect::<HashMap<String, String>>()
}

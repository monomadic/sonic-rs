pub(crate) use anyhow::Result;
use images::SpriteMap;

mod images;

fn main() -> Result<()> {
    std::fs::create_dir_all("assets")?;

    let sonic = images::read_indexed_gif("resources/sonic/Data/Sprites/Players/Sonic1.gif")?;
    sonic.save("assets/sonic.png")?;

    let mut sonic = image::DynamicImage::from(sonic);

    let mut spritemap = SpriteMap::new(60, 60);
    spritemap.push(sonic.crop(1, 1, 29, 39).to_rgba8());
    spritemap.save("sonic.png")?;

    green_hill_zone()?;

    Ok(())
}

fn green_hill_zone() -> Result<()> {
    let tiles = images::read_indexed_gif("resources/sonic/Data/Stages/Zone01/16x16Tiles.gif")?;
    tiles.save("assets/ghz_16x16_tiles.png")?;
    Ok(())
}

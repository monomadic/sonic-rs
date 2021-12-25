#[derive(Default)]
pub(crate) struct Game {
    pub(crate) config: GameConfig,
    mode: GameMode,
    paused: bool,
    debug: bool,
    scroll_offset: Coordinate,
    camera_position: Coordinate,
    camera_offset: Coordinate,
    water_level: f64,
    boundary: Boundary, // not sure what this is for
}

pub(crate) struct GameConfig {
    pub(crate) window_title: String,
    game_description: String,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            window_title: String::from("Sonic"),
            game_description: String::from("Description"),
        }
    }
}

pub(crate) enum GameMode {
    Loading,
}

impl Default for GameMode {
    fn default() -> Self {
        GameMode::Loading
    }
}

#[derive(Default)]
pub(crate) struct Coordinate {
    x: u32,
    y: u32,
}

#[derive(Default)]
pub(crate) struct Boundary {
    top_left: Coordinate,
    bottom_right: Coordinate,
}

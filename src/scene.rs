pub(crate) struct Scene {
    x_scroll_offset: u32,
    y_scroll_offset: u32,
    stage_mode: StageMode,
    game_mode: GameMode,
    active_palette: Palette,
}

pub(crate) enum StageMode {
    Load,
    Normal,
    Paused,
    Frozen,
    TwoPlayer,
    NormalStep,
    PausedStep,
    FrozenStep,
    TwoPlayerStep,
}

pub(crate) enum GameMode {
    DevMenu,
    MainGame,
    InitDevMenu,
    InitModeMenu,
    Wait,
    ScriptError,
    InitPause,
    ExitPause,
    EndGame,
    ResetGame,
    ConnectTwoPlayerVS,
    WaitTwoPlayerVS,
    InitModMenu,
}

pub(crate) struct Palette {
    r: u8,
    g: u8,
    b: u8,
}

impl Default for Scene {
    fn default() -> Self {
        Self {
            x_scroll_offset: 0,
            y_scroll_offset: 0,
            stage_mode: StageMode::Load,
            game_mode: GameMode::MainGame,
            active_palette: Palette { r: 0, g: 0, b: 0 },
        }
    }
}

impl Scene {
    pub(crate) fn init_first_stage() -> Self {
        Self { ..Self::default() }
    }

    pub(crate) fn process(&mut self) {
        match self.stage_mode {
            StageMode::Load => {
                // SetActivePalette(0, 0, 256);
                // ResetBackgroundSettings();
                // LoadStageFiles();
            }
            _ => unimplemented!(),
        }
    }
}

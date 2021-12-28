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
            StageMode::Normal => {}
            _ => unimplemented!(),
        }
    }

    fn set_active_palette(&mut self, new_active_palette: u8, start_line: u32, end_line: u32) {
        // for line in start_line..end_line {
        //     // if line < SCREEN_YSIZE
        //     gfxLineBuffer[line] = new_active_palette;
        //     self.active_palette = fullPalette[gfxLineBuffer[0]];
        // }
    }
}

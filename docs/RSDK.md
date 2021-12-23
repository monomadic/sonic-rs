# RSDK Info

## Engine

- `startStage` = 0 // the initial stage (but where does it load the script?)
- `stageList` holds all stages
- `activeStageList` holds current valid stages ???
- `stageListPosition` = current stage

## Key Concepts

- `SceneInfo` level data
  - Stored as SceneInfo stageList[STAGELIST_MAX][0x100]; in Scene.cpp:10

## Program Flow

- RetroEngine::Init()
  - CalculateTrigAngles()
  - GenerateBlendLookupTable()
  - Load `data.rsdk`
  - InitUserdata()
  - LoadGameConfig `Data/Game/GameConfig.bin`
    - Read game window text
    - Read Palettes
    - Read Objects
    - Read Script Paths
    - Read Global Variables
    - Read SFX
    - Read Player Names
    - Read StageData
  - InitRenderDevice()
  - InitAudioPlayback()
  - InitFirstStage()
    - Init a bunch of scene variables
    - Stop music
    - Set `stageListPosition` to `self.startStage` (default 0)
  - ClearScriptData()
- RetroEngine::Run()

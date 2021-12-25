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

- RetroEngine::Init() // `RetroEngine.cpp`
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
  - Loop:
    - ProcessInput()
    - switch `gameMode`
      - ENGINE_MAINGAME: ProcessStage() // `Scene.cpp`
        - switch `stageMode` // default `STAGEMODE_LOAD`
          - STAGEMODE_LOAD:
            - set defaults
            - ResetBackgroundSettings()
            - LoadStageFiles()
              - StopAllSfx()
              - CheckCurrentStageFolder(stageListPosition) // load scene
              - ReleaseStageSfx()
              - ClearScriptData()
              - <clear sprites>
              - LoadStageFile(`StageConfig.bin`, stageListPosition, &info)
                - loads `Data/Stages/{stageList[activeStageList][stageID].folder}/StageConfig.bin`
                  - load globals
                  - load sfx
                  - load objects
              - LoadStageGIFFile(stageListPosition)
              - LoadStageCollisions()
              - LoadStageBackground()
                - `Data/Stages/<folder>/Backgrounds.bin`
              - LoadStageChunks()
              - LoadActLayout()
                - LoadActFile()
                  - `Data/Stages/<folder>/Act<id>.bin`
                  - ...
              - Init3DFloorBuffer(0)
              - ProcessStartupObjects()
            - stageMode = STAGEMODE_NORMAL
          - STAGEMODE_NORMAL:
            - ProcessObjects();
            - ProcessParallaxAutoScroll();
            - DrawStageGFX();
              - validate water level
              - DrawObjectList(<layer 0-6>)
                - for each drawListEntries[layer]
                  - ProcessScript(int subDraw.scriptCodePtr, int subDraw.jumpTablePtr, SUB_DRAW)
              - if `fadeMode` > 0 (DrawRectangle() draw black box with alpha)

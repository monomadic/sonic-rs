# Sonic

Reverse engineered Sonic, but targetting rust instead of C. You'll need a retail sonic to extract the assets.

Starting for now with Sonic 1, but the extractor will work with any RDSK archives.

## RSDK Extractor

Tested with following `Data.rsdk` files (md5 sum):

MD5                               Size      Origin
B7339E7A4CF4AE6264449EEC06465C7C  ????????  Sonic 2 Android
78a03b962acf3e2622addab9cdb8e2ce  78711043  Sonic CD 

```bash
cargo run --package rsdk-extract -- <file>
```

## Installing / Playing

```bash
cargo run
```

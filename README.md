# Sonic

Reverse engineered Sonic, but targetting rust instead of C. You'll need a retail sonic to extract the assets.

Starting for now with Sonic 1, but the extractor will work with any RDSK archives.

## RSDK Extractor

Tested with following `Data.rsdk` files (md5 sum):

```
MD5                               Size      Origin
2881d2492be3ba5d3b6106cdbf82c3e5  38198396  Sonic the Hedgehog Classic apk v3.7.0
```

```bash
cargo run --package rsdk-extract -- <file>
```

### Why?

Because for some reason Sonic reversers publish weird windows code and I don't really want to touch it with a 10 foot clownpole.

## Installing / Playing

```bash
cargo run
```

## Resources

- https://www.lucianociccariello.com/research/sonicmania
- http://unhaut.epizy.com/retrun/sonic2013.html

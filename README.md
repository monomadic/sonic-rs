# Sonic

Reverse engineered Sonic, but targetting rust instead of C. You'll need a retail sonic to extract the assets.

Unlike other decompilation efforts, the goals of this project are different:
- Far more cross-platform (simple rust that can target any platform including wasm)
  - Initially, basic framebuffer engine (no SDL dependencies etc)
- Easier to install (cargo install, wasmer, etc)
  - Will always be as simple as a `cargo install sonic` when released
- Emphasis on open formats, extensibility, modding
  - Unpacks RSDK archives into basic file structure which is included in the build
  - Simply editing .gif or .png or the rust code will mod the project (no complex unpackers or encryption)

## Progress
- [x] Support RSDKv4 archives
  - [x] Unpacking
  - [x] Decryption
- [ ] Basic game engine (framebuffer)
- [ ] Sonic 1
- [ ] Sonic 2
- [ ] Sonic CD

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

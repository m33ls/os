
A small multitasking operating system in rust.

## Build

Set current directory to use the nightly branch of rust
```
rustup override set nightly
rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu
rustup target add x86_64_unknown-none
```
Build
```
cargo build
```

## Run
with cargo:
```
cargo run
```
or manually in qemu with:
```
qemu-system-x86_64 -drive format=raw,file=target/x86_64-os/debug/bootimage-os.bin
```

## TODO
- [ ] UEFI
- [ ] VESA VBE (BIOS) / GOP (UEFI)
- [ ] Filesystem
- [ ] Heap Allocation
- [ ] Async / Await
- [x] Paging
- [x] PS/2 Keyboard
- [x] VGA Text Mode
- [x] Serial
- [x] IDT, GDT
- [x] Handle breakpoint exceptions, double faults, page faults

## Useful Resources
* [Creating an Operating System](https://wiki.osdev.org/Creating_an_Operating_System)
* [Writing an OS in Rust](https://os.phil-opp.com/)
* [osdever.net Tutorials](https://web.archive.org/web/20250123233604/http://www.osdever.net/tutorials/)
* [osdever.net Papers](https://web.archive.org/web/20250124112310/http://www.osdever.net/papers/)

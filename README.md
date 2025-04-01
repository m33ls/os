
A small multitasking operating system in rust.

## Build

Set current directory to use the nightly branch of rust
```
rustup override set nightly
rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu
```
Install bootimage and dependencies to link operating system
```
cargo install bootimage
rustup component add llvm-tools-preview
```
Build
```
cargo bootimage
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
- [ ] Paging
- [ ] Heap Allocation
- [ ] Async / Await
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

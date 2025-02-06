
Build
```
rustup override set nightly
rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu
cargo install bootimage
rustup component add llvm-tools-preview
cargo bootimage
```

Run
```
cargo run
```
or
```
qemu-system-x86_64 -drive format=raw,file=target/x86_64-os/debug/bootimage-os.bin
```

TODO
[ ] Rewrite / add UEFI support to bootloader crate
[ ] Replace VGA with VESA VBE (BIOS) / GOP (UEFI)
[ ] Input
[ ] Filesystem

Useful Resources
* [Creating an Operating System](https://wiki.osdev.org/Creating_an_Operating_System)
* [Writing an OS in Rust](https://os.phil-opp.com/)

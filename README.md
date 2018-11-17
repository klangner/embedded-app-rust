# Demo Embedded App written in Rust

Run on QEMU
```bash
qemu-system-arm -cpu cortex-m3 -machine lm3s6965evb -gdb tcp::3333 -S -nographic -kernel target/thumbv7m-none-eabi/debug/app
```
Press Ctrl-A h to get help

Debug application
```bash
arm-none-eabi-gdb -q target/thumbv7m-none-eabi/debug/app
```
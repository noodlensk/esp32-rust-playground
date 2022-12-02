# esp32-rust-playground

A playground where I'm trying to build [Tamahotchi](https://en.wikipedia.org/wiki/Tamagotchi) on top
of [Watchy](https://watchy.sqfmi.com/)(ESP-32) being inspired by [pwnagotchi](https://pwnagotchi.ai/)
and [ESP32-WiFi-Hash-Monster](https://github.com/G4lile0/ESP32-WiFi-Hash-Monster)
using [Rust](https://www.rust-lang.org/) and [esp-rs](https://github.com/esp-rs).

## Current progress

Since the goal is to replicate [pwnagotchi](https://pwnagotchi.ai/) behavior, adding support of all Watchy hardware(like
BMA423) is not a prio for now.

- [x] Screen support
- [x] vibromotor support
- [x] pcf8563 rtc support
- [x] WiFi packet sniffing support
- [ ] Buttons support
- [ ] BMA423 support

## Installation

Follow the [instruction](https://github.com/esp-rs/rust-build) for installing Rust esp32 toolchain.

Available commands

```shell
# download deps
make dep
# run linter
make lint
# run linter and fix errors
make lint-fix
# format code
make fmt
# compile
make build
# compile and upload
make release
```

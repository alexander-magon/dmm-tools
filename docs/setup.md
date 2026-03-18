# Setup

## Prerequisites

- Rust toolchain (stable, 2024 edition)
- `libudev-dev` (Linux, for hidapi)
- UNI-T UT61E+ multimeter connected via USB

## Build

```sh
cargo build --workspace
```

## udev Rule (Linux)

To allow non-root access to the HID device:

```sh
sudo cp udev/99-cp2110-unit.rules /etc/udev/rules.d/
sudo udevadm control --reload-rules
sudo udevadm trigger
```

Then re-plug the meter or log out/in.

## Troubleshooting

*To be filled in as issues are discovered.*

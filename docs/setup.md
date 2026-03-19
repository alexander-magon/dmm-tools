# Setup

## Prerequisites

- Rust toolchain (stable, 2024 edition)
- A supported UNI-T multimeter connected via USB (see [supported devices](supported-devices.md) for the full list: UT61E+, UT61B+, UT61D+, UT161 series, UT8803, UT171, UT181A)

**Linux:** `libudev-dev` (Debian/Ubuntu) or `systemd-devel` (Fedora) for hidapi.

**Windows:** Install the [CP2110 HID USB-to-UART bridge driver](https://www.silabs.com/developers/usb-to-uart-bridge-vcp-drivers) from Silicon Labs.

## Build

```sh
cargo build --workspace
```

## Platform setup

### Linux — udev rule

To allow non-root access to the HID device:

```sh
sudo cp udev/99-cp2110-unit.rules /etc/udev/rules.d/
sudo udevadm control --reload-rules
sudo udevadm trigger
```

Then re-plug the meter or log out/in.

Your user must be in the `plugdev` group:

```sh
sudo usermod -aG plugdev $USER
```

Log out and back in for the group change to take effect.

### Windows — driver

Install the CP2110 driver from [Silicon Labs](https://www.silabs.com/developers/usb-to-uart-bridge-vcp-drivers). After installation, verify the device appears in Device Manager under "Human Interface Devices" or "USB Devices".

## Troubleshooting

### "USB adapter not found"

- Verify the CP2110 USB adapter is plugged in
- **Linux:** `lsusb | grep 10C4:EA80` — if missing, check the udev rule (see above)
- **Windows:** check Device Manager for the CP2110 device — if missing or showing an error, reinstall the driver

### "No response from meter"

The USB adapter is detected but the meter isn't transmitting data:

1. Insert the USB module into the meter's IR port
2. Turn the meter on
3. Long press the **USB/Hz** button until the **S** icon appears on the LCD
4. The S icon confirms USB data transmission is active

### GUI won't start (Linux, Wayland/X11)

The GUI uses eframe/egui which supports both Wayland and X11. If you encounter display issues, try forcing X11:

```sh
WINIT_UNIX_BACKEND=x11 ut61eplus-gui
```

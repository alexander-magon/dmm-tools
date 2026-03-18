# UX Design

## Design Principles

1. **Clean and modern** — minimal chrome, subtle separators, generous but efficient use of space
2. **High information density** — no wasted space; every pixel earns its place
3. **Readable** — large primary reading, clear hierarchy, good contrast in both themes
4. **Configurable** — users choose which panels are visible via a settings gear menu
5. **Responsive** — adapts between wide (side-by-side) and narrow (stacked) layouts

## CLI Interface

### Subcommands

- `ut61eplus list` — enumerate connected devices
- `ut61eplus info` — connect to device and print device info
- `ut61eplus read` — continuous measurement reading with `--format` (text/csv/json), `--output`, `--interval-ms`
- `ut61eplus command` — send button press to meter (hold/minmax/rel/range/select/light)
- `ut61eplus debug` — raw hex dump mode for protocol development

## GUI Layout

### Theme

- Supports light and dark mode, toggled via settings
- Default: follows system preference (egui `Visuals`)
- Connected status: green indicator dot
- Disconnected/error: red indicator dot
- Warnings (low battery): orange text

### Top Bar

Compact toolbar row: app title, connect/disconnect button, connection status indicator, settings gear icon (right-aligned).

### Settings Panel

Toggled by the gear icon. Dropdown/popup with:

- **Theme:** Dark / Light / System
- **Show graph:** toggle
- **Show statistics:** toggle
- **Show recording:** toggle
- **Graph time window:** 30s / 1m / 5m / 10m / 1h

Settings persist across sessions via a config file (`~/.config/ut61eplus/settings.json`).

### Responsive Layout

Threshold at ~900px available width:

**Wide (≥ 900px):** Two-column layout.
- Left column (fixed ~220px): reading display, mode/range/flags, statistics panel
- Right column (remaining width): graph (top, fills space), recording bar (bottom)

**Narrow (< 900px):** Single-column stack.
- Reading (compact single line for mode/flags)
- Graph
- Statistics (compact 2-line grid)
- Recording (single-line toolbar)

### Reading Display

- Primary value in large monospace font (e.g., "5.678")
- Unit adjacent, slightly smaller ("V")
- Mode, range label, and active flags below
- Flags shown as subtle badges: AUTO, HOLD, REL, MIN, MAX
- Low battery warning shown as orange "LOW BAT" badge

### Graph Panel

- `egui_plot` scrolling time series
- X-axis: elapsed time in seconds, scrolling window (configurable)
- Y-axis: auto-scaling with some padding
- Configurable time window (30s to 1h)
- History buffer: ~10,000 points (VecDeque, oldest dropped)
- Handles mode changes by clearing the plot (unit change = incompatible data)
- Overload values shown as gaps in the line

### Statistics Panel

- Min, Max, Avg values with units
- Sample count
- Reset button clears all stats
- Resets automatically on mode change

### Recording Panel

- Record toggle button (changes to "Stop" when active)
- Export CSV button (opens file dialog via `rfd`)
- Shows sample count and duration while recording
- Records to in-memory buffer, exported on demand

//! Golden file tests for UT61E+ measurement parsing.
//!
//! Each `.json` file in `tests/golden/ut61eplus/` contains:
//! - `payload_hex`: hex-encoded 14-byte raw measurement payload (spaces allowed for readability)
//! - Expected parsed fields: `mode`, `value`, `unit`, `range_label`, `flags`
//!
//! The test harness decodes the hex payload, parses it with `Ut61ePlusTable`,
//! and compares every field against the expected values.

use serde::Deserialize;
use std::path::Path;
use ut61eplus_lib::measurement::MeasuredValue;
use ut61eplus_lib::protocol::ut61eplus::parse_measurement;
use ut61eplus_lib::protocol::ut61eplus::tables::ut61e_plus::Ut61ePlusTable;

/// Expected value from a golden file. Supports numeric, "OL", and NCV level.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum ExpectedValue {
    Number(f64),
    Overload(String),
    NcvLevel { ncv_level: u8 },
}

/// Expected flag state from a golden file.
#[derive(Debug, Deserialize)]
struct ExpectedFlags {
    hold: bool,
    rel: bool,
    min: bool,
    max: bool,
    auto_range: bool,
    low_battery: bool,
    hv_warning: bool,
    dc: bool,
    peak_max: bool,
    peak_min: bool,
}

/// A single golden test case loaded from JSON.
#[derive(Debug, Deserialize)]
struct GoldenTestCase {
    /// Human-readable description (not used in assertions).
    #[serde(default)]
    #[allow(dead_code)]
    description: String,
    /// Hex-encoded 14-byte payload (spaces stripped before decoding).
    payload_hex: String,
    mode: String,
    value: ExpectedValue,
    unit: String,
    range_label: String,
    flags: ExpectedFlags,
}

/// Decode a hex string (with optional spaces) into bytes.
fn decode_hex(hex: &str) -> Vec<u8> {
    let clean: String = hex.chars().filter(|c| !c.is_whitespace()).collect();
    assert!(
        clean.len().is_multiple_of(2),
        "hex string has odd length: {}\n  cleaned hex: {clean}",
        clean.len()
    );
    (0..clean.len())
        .step_by(2)
        .map(|i| {
            u8::from_str_radix(&clean[i..i + 2], 16)
                .unwrap_or_else(|e| panic!("invalid hex at offset {i}: {e}\n  hex: {clean}"))
        })
        .collect()
}

/// Discover all `.json` golden files in the given directory.
fn discover_golden_files(dir: &Path) -> Vec<std::path::PathBuf> {
    let mut files: Vec<_> = std::fs::read_dir(dir)
        .unwrap_or_else(|e| panic!("cannot read golden dir {}: {e}", dir.display()))
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                Some(path)
            } else {
                None
            }
        })
        .collect();
    files.sort();
    files
}

#[test]
fn golden_ut61eplus() {
    let golden_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/golden/ut61eplus");
    let files = discover_golden_files(&golden_dir);
    assert!(
        !files.is_empty(),
        "no golden files found in {}",
        golden_dir.display()
    );

    let table = Ut61ePlusTable::new();
    let mut passed = 0;

    for path in &files {
        let stem = path.file_stem().unwrap().to_string_lossy();
        let json_str = std::fs::read_to_string(path)
            .unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()));
        let case: GoldenTestCase = serde_json::from_str(&json_str)
            .unwrap_or_else(|e| panic!("cannot parse {}: {e}", path.display()));

        // Decode payload from hex
        let payload = decode_hex(&case.payload_hex);
        assert_eq!(
            payload.len(),
            14,
            "golden file {stem}: payload must be 14 bytes, got {}",
            payload.len()
        );

        // Parse measurement
        let measurement = parse_measurement(&payload, &table)
            .unwrap_or_else(|e| panic!("golden file {stem}: parse failed: {e}"));

        // Check mode
        assert_eq!(
            measurement.mode, case.mode,
            "golden file {stem}: mode mismatch"
        );

        // Check value
        match (&measurement.value, &case.value) {
            (MeasuredValue::Normal(actual), ExpectedValue::Number(expected)) => {
                assert!(
                    (actual - expected).abs() < 1e-6,
                    "golden file {stem}: value mismatch: got {actual}, expected {expected}"
                );
            }
            (MeasuredValue::Overload, ExpectedValue::Overload(s)) => {
                assert_eq!(
                    s, "OL",
                    "golden file {stem}: overload value should be \"OL\", got \"{s}\""
                );
            }
            (MeasuredValue::NcvLevel(actual), ExpectedValue::NcvLevel { ncv_level }) => {
                assert_eq!(
                    actual, ncv_level,
                    "golden file {stem}: NCV level mismatch: got {actual}, expected {ncv_level}"
                );
            }
            (actual, expected) => {
                panic!(
                    "golden file {stem}: value type mismatch: got {actual:?}, expected {expected:?}"
                );
            }
        }

        // Check unit
        assert_eq!(
            measurement.unit, case.unit,
            "golden file {stem}: unit mismatch"
        );

        // Check range label
        assert_eq!(
            measurement.range_label, case.range_label,
            "golden file {stem}: range_label mismatch"
        );

        // Check flags
        let f = &measurement.flags;
        let ef = &case.flags;
        assert_eq!(f.hold, ef.hold, "golden file {stem}: flags.hold mismatch");
        assert_eq!(f.rel, ef.rel, "golden file {stem}: flags.rel mismatch");
        assert_eq!(f.min, ef.min, "golden file {stem}: flags.min mismatch");
        assert_eq!(f.max, ef.max, "golden file {stem}: flags.max mismatch");
        assert_eq!(
            f.auto_range, ef.auto_range,
            "golden file {stem}: flags.auto_range mismatch"
        );
        assert_eq!(
            f.low_battery, ef.low_battery,
            "golden file {stem}: flags.low_battery mismatch"
        );
        assert_eq!(
            f.hv_warning, ef.hv_warning,
            "golden file {stem}: flags.hv_warning mismatch"
        );
        assert_eq!(f.dc, ef.dc, "golden file {stem}: flags.dc mismatch");
        assert_eq!(
            f.peak_max, ef.peak_max,
            "golden file {stem}: flags.peak_max mismatch"
        );
        assert_eq!(
            f.peak_min, ef.peak_min,
            "golden file {stem}: flags.peak_min mismatch"
        );

        passed += 1;
    }

    eprintln!("golden_ut61eplus: {passed}/{} tests passed", files.len());
}

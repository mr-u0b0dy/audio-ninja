// SPDX-License-Identifier: Apache-2.0
//! Fuzzing target for clock synchronization deserialization
//! Tests timestamp parsing and clock source handling

#![no_main]
use audio_ninja::transport::ClockTimestamp;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // Ensure we have enough bytes to interpret as a timestamp
    if data.len() < 16 {
        return;
    }

    // Parse as seconds (8 bytes big-endian) + nanos (4 bytes big-endian)
    let seconds = u64::from_be_bytes([
        data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7],
    ]);

    let nanos = u32::from_be_bytes([data[8], data[9], data[10], data[11]]);

    // Nanoseconds must be < 1_000_000_000
    if nanos >= 1_000_000_000 {
        return;
    }

    let ts = ClockTimestamp {
        seconds,
        nanos,
        source: audio_ninja::transport::ClockSource::System,
    };

    // Test conversions
    let _duration = ts.to_duration();

    // Test skew calculation
    let ts2 = ClockTimestamp {
        seconds: seconds.wrapping_add(1),
        nanos,
        source: audio_ninja::transport::ClockSource::System,
    };

    let _skew = ts.skew_from(&ts2);
});

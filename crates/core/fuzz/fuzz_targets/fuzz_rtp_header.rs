// SPDX-License-Identifier: Apache-2.0
//! Fuzzing target for RTP header deserialization
//! Tests the robustness of RtpHeader::deserialize against malformed input

#![no_main]
use audio_ninja::transport::RtpHeader;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // Attempt to deserialize arbitrary bytes as RTP header
    if let Some(header) = RtpHeader::deserialize(data) {
        // Verify round-trip: serialize and deserialize should be idempotent
        let serialized = header.serialize();
        assert_eq!(serialized.len(), 12);

        // Deserialize the re-serialized header
        if let Some(re_header) = RtpHeader::deserialize(&serialized) {
            // Verify all fields match
            assert_eq!(re_header.version, header.version);
            assert_eq!(re_header.padding, header.padding);
            assert_eq!(re_header.extension, header.extension);
            assert_eq!(re_header.csrc_count, header.csrc_count);
            assert_eq!(re_header.marker, header.marker);
            assert_eq!(re_header.payload_type, header.payload_type);
            assert_eq!(re_header.sequence, header.sequence);
            assert_eq!(re_header.timestamp, header.timestamp);
            assert_eq!(re_header.ssrc, header.ssrc);
        }
    }
});

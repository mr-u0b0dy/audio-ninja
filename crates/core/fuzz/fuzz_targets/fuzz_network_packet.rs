// SPDX-License-Identifier: Apache-2.0
//! Fuzzing target for network packet handling
//! Tests FEC decode and packet assembly logic

#![no_main]
use audio_ninja::fec::XorFec;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // Ensure minimum size for FEC parameters
    if data.len() < 4 {
        return;
    }

    // Extract FEC parameters
    let k = (data[0] as usize) % 32 + 1; // 1-32 data packets
    let m = (data[1] as usize) % 8 + 1;  // 1-8 FEC packets

    // Create FEC decoder
    let fec = XorFec::new(k, m).ok()?;

    // Split data into packets
    let packet_size = if data.len() > 4 { data.len() / (k + m) } else { 1 };

    let mut packets: Vec<Vec<u8>> = Vec::new();
    for i in 0..k {
        let start = 4 + (i * packet_size);
        let end = std::cmp::min(start + packet_size, data.len());
        if start < data.len() {
            packets.push(data[start..end].to_vec());
        } else {
            packets.push(vec![0; packet_size]);
        }
    }

    let fec_packets = if data.len() > 4 + k * packet_size {
        &data[4 + k * packet_size..]
    } else {
        &[]
    };

    // Attempt FEC decode
    let _ = fec.decode(&packets, fec_packets);
});

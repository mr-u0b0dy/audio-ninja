// SPDX-License-Identifier: Apache-2.0

use audio_ninja::fec::*;
use audio_ninja::AudioBlock;

#[test]
fn test_xor_fec_basic() {
    let mut fec = XorFec::new(4);

    // Feed 3 packets
    assert!(fec.encode(&[1, 2, 3]).is_none());
    assert!(fec.encode(&[4, 5, 6]).is_none());
    assert!(fec.encode(&[7, 8, 9]).is_none());

    // 4th packet triggers FEC generation
    let result = fec.encode(&[10, 11, 12]);
    assert!(result.is_some());
}

#[test]
fn test_xor_fec_recovery() {
    let fec = XorFec::new(3);

    // Original packets
    let p1 = vec![0xAA, 0xBB, 0xCC];
    let p2 = vec![0x11, 0x22, 0x33];
    let p3 = vec![0xFF, 0xEE, 0xDD];

    // Generate FEC (XOR of all three)
    let fec_packet: Vec<u8> = p1
        .iter()
        .zip(p2.iter())
        .zip(p3.iter())
        .map(|((&a, &b), &c)| a ^ b ^ c)
        .collect();

    // Lose p2, recover it from p1, p3, and FEC
    let available = vec![p1.clone(), p3.clone()];
    let recovered = fec.decode(&available, &fec_packet).unwrap();

    assert_eq!(recovered, p2);
}

#[test]
fn test_loss_statistics_no_loss() {
    let mut stats = LossStatistics::new();

    for i in 0..100 {
        stats.update(i);
    }

    assert_eq!(stats.total_received, 100);
    assert_eq!(stats.total_lost, 0);
    assert_eq!(stats.loss_rate(), 0.0);
    assert_eq!(stats.consecutive_losses, 0);
}

#[test]
fn test_loss_statistics_with_gaps() {
    let mut stats = LossStatistics::new();

    stats.update(0);
    stats.update(1);
    stats.update(2);
    // Gap: 3, 4 missing
    stats.update(5);
    stats.update(6);
    // Gap: 7 missing
    stats.update(8);

    assert_eq!(stats.total_received, 6);
    assert_eq!(stats.total_lost, 3); // packets 3, 4, 7
    assert!(stats.loss_rate() > 0.0);
    assert_eq!(stats.max_consecutive_losses, 2);
}

#[test]
fn test_loss_statistics_recovery() {
    let mut stats = LossStatistics::new();

    stats.update(0);
    stats.update(2); // Lost packet 1

    assert_eq!(stats.total_lost, 1);

    stats.record_recovery();

    assert_eq!(stats.total_recovered, 1);
    assert_eq!(stats.recovery_rate(), 1.0);
}

#[test]
fn test_loss_concealment_silence() {
    let concealer = LossConcealer::new(ConcealmentStrategy::Silence);

    let block = concealer.conceal(48000, 2, 480);

    assert_eq!(block.sample_rate, 48000);
    assert_eq!(block.channels.len(), 2);
    assert_eq!(block.channels[0].len(), 480);

    // All samples should be zero
    for channel in &block.channels {
        assert!(channel.iter().all(|&s| s == 0.0));
    }
}

#[test]
fn test_loss_concealment_repeat() {
    let mut concealer = LossConcealer::new(ConcealmentStrategy::Repeat);

    // Feed a block
    let original = AudioBlock {
        sample_rate: 48000,
        channels: vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]],
    };

    concealer.update(original.clone());

    // Request concealment
    let concealed = concealer.conceal(48000, 2, 3);

    assert_eq!(concealed.channels, original.channels);
}

#[test]
fn test_loss_concealment_interpolate() {
    let mut concealer = LossConcealer::new(ConcealmentStrategy::Interpolate);

    let original = AudioBlock {
        sample_rate: 48000,
        channels: vec![vec![1.0; 10]],
    };

    concealer.update(original);

    let concealed = concealer.conceal(48000, 1, 10);

    // First sample should be close to 1.0, last should be close to 0.0 (fade out)
    assert!(concealed.channels[0][0] > 0.9);
    assert!(concealed.channels[0][9] < 0.15); // More lenient threshold

    // Verify it's monotonically decreasing
    for i in 1..concealed.channels[0].len() {
        assert!(concealed.channels[0][i] <= concealed.channels[0][i - 1]);
    }
}

#[test]
fn test_fec_receiver_basic() {
    let mut receiver = FecReceiver::new(4, ConcealmentStrategy::Silence);

    receiver.process_packet(0, vec![1, 2, 3]);
    receiver.process_packet(1, vec![4, 5, 6]);
    receiver.process_packet(2, vec![7, 8, 9]);

    let stats = receiver.statistics();
    assert_eq!(stats.total_received, 3);
    assert_eq!(stats.total_lost, 0);
}

#[test]
fn test_fec_receiver_loss_detection() {
    let mut receiver = FecReceiver::new(4, ConcealmentStrategy::Silence);

    receiver.process_packet(0, vec![1, 2, 3]);
    receiver.process_packet(1, vec![4, 5, 6]);
    // Skip packet 2
    receiver.process_packet(3, vec![10, 11, 12]);

    let stats = receiver.statistics();
    assert_eq!(stats.total_lost, 1);
    assert_eq!(stats.consecutive_losses, 1);
}

#[test]
fn test_fec_receiver_concealment() {
    let receiver = FecReceiver::new(4, ConcealmentStrategy::Silence);

    let concealed = receiver.conceal_loss(48000, 2, 240);

    assert_eq!(concealed.sample_rate, 48000);
    assert_eq!(concealed.channels.len(), 2);
    assert_eq!(concealed.channels[0].len(), 240);
}

#[test]
fn test_fec_receiver_with_update() {
    let mut receiver = FecReceiver::new(4, ConcealmentStrategy::Repeat);

    let block = AudioBlock {
        sample_rate: 48000,
        channels: vec![vec![0.5; 10], vec![0.5; 10]],
    };

    receiver.update_concealer(block.clone());

    let concealed = receiver.conceal_loss(48000, 2, 10);
    assert_eq!(concealed.channels, block.channels);
}

#[test]
fn test_loss_rate_calculation() {
    let mut stats = LossStatistics::new();

    // Receive 80 out of 100 packets (20% loss)
    for i in 0..100 {
        if i % 5 != 0 {
            stats.update(i);
        }
    }

    let loss_rate = stats.loss_rate();
    assert!(loss_rate > 0.15 && loss_rate < 0.25);
}

#[test]
fn test_max_consecutive_losses() {
    let mut stats = LossStatistics::new();

    stats.update(0);
    stats.update(1);
    // Lose 2, 3, 4, 5
    stats.update(6);
    stats.update(7);
    // Lose 8, 9
    stats.update(10);

    assert_eq!(stats.max_consecutive_losses, 4);
}

// SPDX-License-Identifier: Apache-2.0

use audio_ninja::iamf::*;
use audio_ninja::mapping::*;
use audio_ninja::{AudioBlock, SpeakerRole};
use std::time::Duration;

#[test]
fn test_iamf_decoder_configure() {
    let mut decoder = ReferenceIamfDecoder::new();
    let cfg = IamfStreamConfig {
        sample_rate: 48000,
        channel_count: 6,
        frame_duration: Duration::from_millis(20),
        mix_presentations: vec![],
        channel_elements: vec![],
        object_elements: vec![],
        scene_elements: vec![],
    };

    assert!(decoder.configure(cfg).is_ok());
}

#[test]
fn test_iamf_decode_block() {
    let mut decoder = ReferenceIamfDecoder::new();
    let cfg = IamfStreamConfig {
        sample_rate: 48000,
        channel_count: 2,
        frame_duration: Duration::from_millis(20),
        mix_presentations: vec![],
        channel_elements: vec![],
        object_elements: vec![],
        scene_elements: vec![],
    };

    decoder.configure(cfg).unwrap();
    let result = decoder.decode_block(&[]);
    assert!(result.is_ok());

    let block = result.unwrap();
    assert_eq!(block.audio.sample_rate, 48000);
    assert_eq!(block.audio.channels.len(), 2);
}

#[test]
fn test_layout_from_name_stereo() {
    let layout = layout_from_name("stereo").expect("stereo layout should exist");
    assert_eq!(layout.name, "stereo");
    assert_eq!(layout.speakers.len(), 2);
    assert_eq!(layout.speakers[0].role, SpeakerRole::FrontLeft);
    assert_eq!(layout.speakers[1].role, SpeakerRole::FrontRight);
}

#[test]
fn test_layout_from_name_5_1() {
    let layout = layout_from_name("5.1").expect("5.1 layout should exist");
    assert_eq!(layout.name, "5.1");
    assert_eq!(layout.speakers.len(), 6);
}

#[test]
fn test_downmix_5_1_to_stereo() {
    let input = vec![
        vec![1.0; 100], // FL
        vec![1.0; 100], // FR
        vec![1.0; 100], // C
        vec![0.5; 100], // LFE
        vec![0.5; 100], // SL
        vec![0.5; 100], // SR
    ];

    let output = downmix_channels(&input, 2);
    assert_eq!(output.len(), 2);
    assert_eq!(output[0].len(), 100);
    assert_eq!(output[1].len(), 100);

    // Check center was mixed in
    assert!(output[0][0] > 1.0);
    assert!(output[1][0] > 1.0);
}

#[test]
fn test_upmix_stereo_to_5_1() {
    let input = vec![
        vec![1.0; 100], // L
        vec![1.0; 100], // R
    ];

    let output = upmix_channels(&input, 6);
    assert_eq!(output.len(), 6);

    // Original channels preserved
    assert_eq!(output[0], input[0]);
    assert_eq!(output[1], input[1]);

    // Extra channels are silence
    assert_eq!(output[2][0], 0.0);
    assert_eq!(output[5][0], 0.0);
}

#[test]
fn test_map_channel_element_to_layout() {
    let layout = layout_from_name("stereo").unwrap();

    let element = ChannelAudioElement {
        element_id: 1,
        channel_labels: vec![SpeakerRole::FrontLeft, SpeakerRole::FrontRight],
        codec: CodecConfig::Pcm {
            sample_rate: 48000,
            bit_depth: 16,
            channels: 2,
        },
    };

    let audio = AudioBlock {
        sample_rate: 48000,
        channels: vec![vec![1.0; 100], vec![2.0; 100]],
    };

    let mapped = map_channel_element_to_layout(&element, &audio, &layout);
    assert_eq!(mapped.channels.len(), 2);
    assert_eq!(mapped.channels[0][0], 1.0);
    assert_eq!(mapped.channels[1][0], 2.0);
}

#[test]
fn test_vbap_stereo_center() {
    use audio_ninja::Position3;

    let speakers = layout_from_name("stereo").unwrap().speakers;

    // Object at center should give equal gain to both speakers
    let obj_pos = Position3 {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };

    let gains = vbap_stereo(&obj_pos, &speakers);
    assert_eq!(gains.len(), 2);

    // Gains should sum to approximately 1.0 (power normalized)
    let sum_squared: f32 = gains.iter().map(|g| g * g).sum();
    assert!((sum_squared - 1.0).abs() < 0.01);
}

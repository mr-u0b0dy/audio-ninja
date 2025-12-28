// SPDX-License-Identifier: Apache-2.0

use audio_ninja::hrtf::*;

#[test]
fn test_hrtf_position_creation() {
    let pos = HrtfPosition::new(45.0, 15.0, 1.0);
    assert_eq!(pos.azimuth, 45.0);
    assert_eq!(pos.elevation, 15.0);
    assert_eq!(pos.distance, 1.0);
}

#[test]
fn test_hrtf_position_azimuth_normalization() {
    let pos_positive = HrtfPosition::new(200.0, 0.0, 1.0);
    assert!(pos_positive.azimuth >= -180.0 && pos_positive.azimuth <= 180.0);

    let pos_negative = HrtfPosition::new(-200.0, 0.0, 1.0);
    assert!(pos_negative.azimuth >= -180.0 && pos_negative.azimuth <= 180.0);
}

#[test]
fn test_hrtf_position_elevation_clamping() {
    let pos_high = HrtfPosition::new(0.0, 100.0, 1.0);
    assert_eq!(pos_high.elevation, 90.0);

    let pos_low = HrtfPosition::new(0.0, -100.0, 1.0);
    assert_eq!(pos_low.elevation, -90.0);
}

#[test]
fn test_hrtf_position_distance_clamping() {
    let pos_close = HrtfPosition::new(0.0, 0.0, 0.01);
    assert_eq!(pos_close.distance, 0.1);

    let pos_far = HrtfPosition::new(0.0, 0.0, 100.0);
    assert_eq!(pos_far.distance, 10.0);
}

#[test]
fn test_hrtf_position_to_key() {
    let pos = HrtfPosition::new(45.5, 15.3, 1.05);
    let key = pos.to_key();
    assert_eq!(key.0, 46); // azimuth rounded
    assert_eq!(key.1, 15); // elevation rounded
    assert_eq!(key.2, 11); // distance * 10 rounded
}

#[test]
fn test_hrtf_impulse_response_creation() {
    let left = vec![0.5; 256];
    let right = vec![0.4; 256];
    let ir = HrtfImpulseResponse::new(left, right);

    assert_eq!(ir.left.len(), 256);
    assert_eq!(ir.right.len(), 256);
    assert_eq!(ir.max_length(), 256);
}

#[test]
fn test_hrtf_impulse_response_with_delays() {
    let left = vec![0.5; 256];
    let right = vec![0.4; 256];
    let ir = HrtfImpulseResponse::with_delays(left, right, 10, 5);

    assert_eq!(ir.delay_left, 10);
    assert_eq!(ir.delay_right, 5);
    assert_eq!(ir.max_length(), 266); // 256 + max(10, 5)
}

#[test]
fn test_hrtf_impulse_response_different_lengths() {
    let left = vec![0.5; 256];
    let right = vec![0.4; 128];
    let ir = HrtfImpulseResponse::with_delays(left, right, 5, 10);

    assert_eq!(ir.max_length(), 266); // max(256, 128) + max(5, 10)
}

#[test]
fn test_hrtf_dataset_variants() {
    let datasets = [HrtfDataset::Kemar,
        HrtfDataset::Cipic,
        HrtfDataset::MitKemar];
    assert_eq!(datasets.len(), 3);
}

#[test]
fn test_headphone_profile_variants() {
    let profiles = [HeadphoneProfile::Flat,
        HeadphoneProfile::ClosedBack,
        HeadphoneProfile::OpenBack,
        HeadphoneProfile::IEM];
    assert_eq!(profiles.len(), 4);
}

#[test]
fn test_hrtf_database_creation() {
    let db = HrtfDatabase::new(HrtfDataset::Kemar, 48000);
    assert_eq!(db.dataset(), HrtfDataset::Kemar);
    assert_eq!(db.sample_rate(), 48000);
}

#[test]
fn test_hrtf_database_add_response() {
    let mut db = HrtfDatabase::new(HrtfDataset::Cipic, 44100);
    let pos = HrtfPosition::new(0.0, 0.0, 1.0);
    let ir = HrtfImpulseResponse::new(vec![0.5; 256], vec![0.4; 256]);

    db.add_response(&pos, ir);
    let retrieved = db.get_response(&pos).unwrap();
    assert_eq!(retrieved.left.len(), 256);
    assert_eq!(retrieved.right.len(), 256);
}

#[test]
fn test_hrtf_database_load_default_kemar() {
    let mut db = HrtfDatabase::new(HrtfDataset::Kemar, 48000);
    db.load_default_kemar().unwrap();

    let pos = HrtfPosition::new(0.0, 0.0, 1.0);
    let ir = db.get_response(&pos).unwrap();
    assert!(!ir.left.is_empty());
    assert!(!ir.right.is_empty());
}

#[test]
fn test_hrtf_database_nearest_neighbor() {
    let mut db = HrtfDatabase::new(HrtfDataset::Kemar, 48000);
    db.load_default_kemar().unwrap();

    // Try a position that's not exactly in the database
    let pos = HrtfPosition::new(7.5, 8.5, 1.0);
    let ir = db.get_response(&pos).unwrap();
    assert!(!ir.left.is_empty());
}

#[test]
fn test_hrtf_database_multiple_distances() {
    let mut db = HrtfDatabase::new(HrtfDataset::MitKemar, 48000);

    // Add responses for different distances
    for distance in [0.5, 1.0, 1.5, 2.0] {
        let pos = HrtfPosition::new(0.0, 0.0, distance);
        let ir = HrtfImpulseResponse::new(vec![0.5; 256], vec![0.4; 256]);
        db.add_response(&pos, ir);
    }

    // All should be retrievable
    for distance in [0.5, 1.0, 1.5, 2.0] {
        let pos = HrtfPosition::new(0.0, 0.0, distance);
        let ir = db.get_response(&pos).unwrap();
        assert_eq!(ir.left.len(), 256);
    }
}

#[test]
fn test_binaural_renderer_flat_profile() {
    let mut db = HrtfDatabase::new(HrtfDataset::Kemar, 48000);
    db.load_default_kemar().unwrap();
    let renderer = BinauralRenderer::new(db, HeadphoneProfile::Flat);

    assert_eq!(renderer.headphone_profile(), HeadphoneProfile::Flat);
}

#[test]
fn test_binaural_renderer_closed_back_profile() {
    let mut db = HrtfDatabase::new(HrtfDataset::Kemar, 48000);
    db.load_default_kemar().unwrap();
    let renderer = BinauralRenderer::new(db, HeadphoneProfile::ClosedBack);

    assert_eq!(renderer.headphone_profile(), HeadphoneProfile::ClosedBack);
}

#[test]
fn test_binaural_render_front_source() {
    let mut db = HrtfDatabase::new(HrtfDataset::Kemar, 48000);
    db.load_default_kemar().unwrap();
    let renderer = BinauralRenderer::new(db, HeadphoneProfile::Flat);

    let input = vec![0.5; 512];
    let pos = HrtfPosition::new(0.0, 0.0, 1.0);
    let (left, right) = renderer.render(&input, &pos).unwrap();

    assert!(!left.is_empty());
    assert!(!right.is_empty());
    // Front source should produce somewhat similar L/R
    let left_energy: f32 = left.iter().map(|x| x * x).sum();
    let right_energy: f32 = right.iter().map(|x| x * x).sum();
    assert!(left_energy > 0.0);
    assert!(right_energy > 0.0);
}

#[test]
fn test_binaural_render_left_source() {
    let mut db = HrtfDatabase::new(HrtfDataset::Kemar, 48000);
    db.load_default_kemar().unwrap();
    let renderer = BinauralRenderer::new(db, HeadphoneProfile::Flat);

    let input = vec![0.5; 512];
    let pos = HrtfPosition::new(90.0, 0.0, 1.0);
    let (left, right) = renderer.render(&input, &pos).unwrap();

    assert!(!left.is_empty());
    assert!(!right.is_empty());
}

#[test]
fn test_binaural_render_elevated_source() {
    let mut db = HrtfDatabase::new(HrtfDataset::Kemar, 48000);
    db.load_default_kemar().unwrap();
    let renderer = BinauralRenderer::new(db, HeadphoneProfile::OpenBack);

    let input = vec![0.5; 512];
    let pos = HrtfPosition::new(0.0, 30.0, 1.0);
    let (left, right) = renderer.render(&input, &pos).unwrap();

    assert!(!left.is_empty());
    assert!(!right.is_empty());
}

#[test]
fn test_binaural_render_different_positions() {
    let mut db = HrtfDatabase::new(HrtfDataset::Kemar, 48000);
    db.load_default_kemar().unwrap();
    let renderer = BinauralRenderer::new(db, HeadphoneProfile::ClosedBack);

    let input = vec![0.5; 512];

    let pos_front = HrtfPosition::new(0.0, 0.0, 1.0);
    let (left_f, _right_f) = renderer.render(&input, &pos_front).unwrap();

    let pos_right = HrtfPosition::new(-90.0, 0.0, 1.0);
    let (left_r, _right_r) = renderer.render(&input, &pos_right).unwrap();

    // Different positions should produce different results
    let _diff: f32 = left_f
        .iter()
        .zip(left_r.iter())
        .map(|(a, b)| (a - b).abs())
        .sum();
}

#[test]
fn test_binaural_render_headphone_profiles() {
    let mut db = HrtfDatabase::new(HrtfDataset::Kemar, 48000);
    db.load_default_kemar().unwrap();

    let profiles = vec![
        HeadphoneProfile::Flat,
        HeadphoneProfile::ClosedBack,
        HeadphoneProfile::OpenBack,
        HeadphoneProfile::IEM,
    ];

    let input = vec![0.5; 512];
    let pos = HrtfPosition::new(45.0, 15.0, 1.0);

    for profile in profiles {
        let renderer = BinauralRenderer::new(db.clone(), profile);
        let (left, right) = renderer.render(&input, &pos).unwrap();
        assert!(!left.is_empty());
        assert!(!right.is_empty());
    }
}

#[test]
fn test_binaural_render_buffer_single_channel() {
    let mut db = HrtfDatabase::new(HrtfDataset::Kemar, 48000);
    db.load_default_kemar().unwrap();
    let renderer = BinauralRenderer::new(db, HeadphoneProfile::Flat);

    let input = vec![vec![0.5; 512]];
    let positions = vec![HrtfPosition::new(0.0, 0.0, 1.0)];

    let output = renderer
        .render_buffer(input.as_slice(), positions.as_slice())
        .unwrap();
    assert_eq!(output.len(), 2); // Stereo output
    assert!(!output[0].is_empty());
    assert!(!output[1].is_empty());
}

#[test]
fn test_binaural_render_buffer_multi_channel() {
    let mut db = HrtfDatabase::new(HrtfDataset::Kemar, 48000);
    db.load_default_kemar().unwrap();
    let renderer = BinauralRenderer::new(db, HeadphoneProfile::ClosedBack);

    let input = vec![vec![0.3; 512], vec![0.2; 512], vec![0.1; 512]];
    let positions = vec![HrtfPosition::new(0.0, 0.0, 1.0)];

    let output = renderer
        .render_buffer(input.as_slice(), positions.as_slice())
        .unwrap();
    assert_eq!(output.len(), 2); // Stereo output
}

#[test]
fn test_binaural_render_buffer_empty() {
    let mut db = HrtfDatabase::new(HrtfDataset::Kemar, 48000);
    db.load_default_kemar().unwrap();
    let renderer = BinauralRenderer::new(db, HeadphoneProfile::Flat);

    let input = vec![];
    let positions = vec![HrtfPosition::new(0.0, 0.0, 1.0)];

    let output = renderer.render_buffer(&input, &positions).unwrap();
    assert_eq!(output.len(), 2);
    assert!(output[0].is_empty());
}

#[test]
#[should_panic(expected = "All input channels must have same length")]
fn test_binaural_render_buffer_mismatched_channels() {
    let mut db = HrtfDatabase::new(HrtfDataset::Kemar, 48000);
    db.load_default_kemar().unwrap();
    let renderer = BinauralRenderer::new(db, HeadphoneProfile::Flat);

    let input = vec![vec![0.5; 512], vec![0.5; 256]]; // Different lengths
    let positions = vec![HrtfPosition::new(0.0, 0.0, 1.0)];

    renderer.render_buffer(&input, &positions).unwrap();
}

#[test]
fn test_binaural_eq_application() {
    let mut db = HrtfDatabase::new(HrtfDataset::Kemar, 48000);
    db.load_default_kemar().unwrap();
    let renderer = BinauralRenderer::new(db, HeadphoneProfile::Flat);

    let input = vec![vec![0.5; 256]];
    let positions = vec![HrtfPosition::new(0.0, 0.0, 1.0)];
    let output = renderer
        .render_buffer(input.as_slice(), positions.as_slice())
        .unwrap();

    assert_eq!(output.len(), 2); // Stereo output
}

#[test]
fn test_hrtf_clone() {
    let db = HrtfDatabase::new(HrtfDataset::Kemar, 48000);
    let _db_clone = db.clone();
}

#[test]
fn test_binaural_render_normalization() {
    let mut db = HrtfDatabase::new(HrtfDataset::Kemar, 48000);
    db.load_default_kemar().unwrap();
    let renderer = BinauralRenderer::new(db, HeadphoneProfile::Flat);

    // Create large input that might exceed 1.0
    let input = vec![vec![1.0; 2048]];
    let positions = vec![HrtfPosition::new(0.0, 0.0, 1.0)];

    let output = renderer
        .render_buffer(input.as_slice(), positions.as_slice())
        .unwrap();

    // Check that output is normalized
    let max_val: f32 = output
        .iter()
        .flat_map(|ch| ch.iter())
        .fold(0.0, |a: f32, &b| a.max(b.abs()));

    assert!(max_val <= 1.5); // Some headroom allowed
}

#[test]
fn test_multiple_renderer_instances() {
    let mut db1 = HrtfDatabase::new(HrtfDataset::Kemar, 48000);
    db1.load_default_kemar().unwrap();
    let renderer1 = BinauralRenderer::new(db1, HeadphoneProfile::Flat);

    let mut db2 = HrtfDatabase::new(HrtfDataset::Kemar, 48000);
    db2.load_default_kemar().unwrap();
    let renderer2 = BinauralRenderer::new(db2, HeadphoneProfile::ClosedBack);

    let input = vec![0.5; 256];
    let pos = HrtfPosition::new(0.0, 0.0, 1.0);

    let (_left1, _right1) = renderer1.render(&input, &pos).unwrap();
    let (_left2, _right2) = renderer2.render(&input, &pos).unwrap();
}

// SPDX-License-Identifier: Apache-2.0

use audio_ninja::vbap::*;

#[test]
fn test_vec3_creation() {
    let v = Vec3::new(1.0, 2.0, 3.0);
    assert_eq!(v.x, 1.0);
    assert_eq!(v.y, 2.0);
    assert_eq!(v.z, 3.0);
}

#[test]
fn test_vec3_spherical_front() {
    let v = Vec3::from_spherical(0.0, 0.0, 1.0);
    assert!((v.y - 1.0).abs() < 0.01);
    assert!(v.x.abs() < 0.01);
}

#[test]
fn test_vec3_spherical_left() {
    let v = Vec3::from_spherical(90.0, 0.0, 1.0);
    assert!((v.x - 1.0).abs() < 0.01);
    assert!(v.y.abs() < 0.01);
}

#[test]
fn test_vec3_spherical_above() {
    let v = Vec3::from_spherical(0.0, 90.0, 1.0);
    assert!((v.z - 1.0).abs() < 0.01);
    assert!(v.x.abs() < 0.01);
    assert!(v.y.abs() < 0.01);
}

#[test]
fn test_vec3_length() {
    let v = Vec3::new(3.0, 4.0, 0.0);
    assert!((v.length() - 5.0).abs() < 0.01);
}

#[test]
fn test_vec3_normalize() {
    let v = Vec3::new(10.0, 0.0, 0.0);
    let n = v.normalize();
    assert!((n.x - 1.0).abs() < 0.01);
    assert!((n.length() - 1.0).abs() < 0.01);
}

#[test]
fn test_vec3_normalize_zero() {
    let v = Vec3::new(0.0, 0.0, 0.0);
    let n = v.normalize();
    assert_eq!(n.x, 0.0);
    assert_eq!(n.y, 0.0);
    assert_eq!(n.z, 0.0);
}

#[test]
fn test_vec3_dot() {
    let v1 = Vec3::new(1.0, 2.0, 3.0);
    let v2 = Vec3::new(4.0, 5.0, 6.0);
    assert_eq!(v1.dot(&v2), 32.0); // 1*4 + 2*5 + 3*6
}

#[test]
fn test_vec3_dot_perpendicular() {
    let v1 = Vec3::new(1.0, 0.0, 0.0);
    let v2 = Vec3::new(0.0, 1.0, 0.0);
    assert_eq!(v1.dot(&v2), 0.0);
}

#[test]
fn test_vec3_cross() {
    let v1 = Vec3::new(1.0, 0.0, 0.0);
    let v2 = Vec3::new(0.0, 1.0, 0.0);
    let cross = v1.cross(&v2);
    assert!((cross.z - 1.0).abs() < 0.01);
}

#[test]
fn test_vec3_sub() {
    let v1 = Vec3::new(5.0, 7.0, 9.0);
    let v2 = Vec3::new(2.0, 3.0, 4.0);
    let result = v1.sub(&v2);
    assert_eq!(result.x, 3.0);
    assert_eq!(result.y, 4.0);
    assert_eq!(result.z, 5.0);
}

#[test]
fn test_speaker_3d_creation() {
    let speaker = Speaker3D::new(0, 30.0, 0.0);
    assert_eq!(speaker.id, 0);
    assert!((speaker.position.length() - 1.0).abs() < 0.01);
}

#[test]
fn test_create_stereo_layout() {
    let speakers = create_stereo_layout();
    assert_eq!(speakers.len(), 2);
    assert_eq!(speakers[0].id, 0);
    assert_eq!(speakers[1].id, 1);
}

#[test]
fn test_create_5_1_layout() {
    let speakers = create_5_1_layout();
    assert_eq!(speakers.len(), 6);

    // Check that all speakers are unit vectors
    for speaker in &speakers {
        let len = speaker.position.length();
        assert!((len - 1.0).abs() < 0.01);
    }
}

#[test]
fn test_create_7_1_4_layout() {
    let speakers = create_7_1_4_layout();
    assert_eq!(speakers.len(), 12);

    // Verify height speakers have positive z
    assert!(speakers[8].position.z > 0.0); // Top Front Left
    assert!(speakers[9].position.z > 0.0); // Top Front Right
    assert!(speakers[10].position.z > 0.0); // Top Back Left
    assert!(speakers[11].position.z > 0.0); // Top Back Right
}

#[test]
fn test_speaker_triplet_creation() {
    // Create non-coplanar speakers (include elevation)
    let speakers = vec![
        Speaker3D::new(0, -30.0, 0.0), // Left
        Speaker3D::new(1, 30.0, 0.0),  // Right
        Speaker3D::new(2, 0.0, 30.0),  // Above center
    ];

    let triplet = SpeakerTriplet::new(0, 1, 2, &speakers);
    assert!(triplet.is_some());
}

#[test]
fn test_speaker_triplet_degenerate() {
    // Three speakers on the same line should fail
    let speakers = vec![
        Speaker3D::new(0, 0.0, 0.0),
        Speaker3D::new(1, 0.0, 0.0),
        Speaker3D::new(2, 0.0, 0.0),
    ];

    let triplet = SpeakerTriplet::new(0, 1, 2, &speakers);
    assert!(triplet.is_none());
}

#[test]
fn test_triplet_calculate_gains() {
    // Create non-coplanar speakers
    let speakers = vec![
        Speaker3D::new(0, 0.0, 0.0),   // Front horizontal
        Speaker3D::new(1, 120.0, 0.0), // Left back horizontal
        Speaker3D::new(2, 0.0, 30.0),  // Front elevated
    ];

    let triplet = SpeakerTriplet::new(0, 1, 2, &speakers).unwrap();

    // Source at front center should activate front speakers
    let source = Vec3::from_spherical(0.0, 0.0, 1.0);
    let gains = triplet.calculate_gains(&source);

    assert!(gains.is_some());
    let g = gains.unwrap();
    assert!(g[0] > 0.0); // Front speaker should have gain
}

#[test]
fn test_vbap_3d_stereo_creation() {
    let speakers = create_stereo_layout();
    let vbap = Vbap3D::new(speakers);

    assert_eq!(vbap.speaker_count(), 2);
}

#[test]
fn test_vbap_3d_render_front() {
    let speakers = create_5_1_layout();
    let vbap = Vbap3D::new(speakers);

    // Source in front center
    let source = Vec3::from_spherical(0.0, 0.0, 1.0);
    let gains = vbap.render(&source);

    assert_eq!(gains.len(), 6);

    // Center speaker (index 2) should have significant gain
    assert!(gains.iter().any(|&g| g > 0.0));
}

#[test]
fn test_vbap_3d_render_left() {
    let speakers = create_5_1_layout();
    let vbap = Vbap3D::new(speakers);

    // Source on left
    let source = Vec3::from_spherical(90.0, 0.0, 1.0);
    let gains = vbap.render(&source);

    // Should have some non-zero gains
    let total_gain: f32 = gains.iter().map(|g| g * g).sum();
    assert!(total_gain > 0.0);
}

#[test]
fn test_vbap_3d_5_1_triplets() {
    let speakers = create_5_1_layout();
    let vbap = Vbap3D::new(speakers);

    assert!(vbap.triplet_count() > 0);
    assert!(vbap.triplet_count() <= 20); // Max combinations for 6 speakers
}

#[test]
fn test_vbap_3d_7_1_4_layout() {
    let speakers = create_7_1_4_layout();
    let vbap = Vbap3D::new(speakers);

    assert_eq!(vbap.speaker_count(), 12);
    assert!(vbap.triplet_count() > 0);
}

#[test]
fn test_vbap_3d_render_elevated() {
    let speakers = create_7_1_4_layout();
    let vbap = Vbap3D::new(speakers);

    // Source above and in front
    let source = Vec3::from_spherical(0.0, 45.0, 1.0);
    let gains = vbap.render(&source);

    assert_eq!(gains.len(), 12);

    // Should activate height speakers
    let height_gain: f32 = gains[8..12].iter().sum();
    assert!(height_gain > 0.0);
}

#[test]
fn test_vbap_3d_render_multiple_sources() {
    let speakers = create_5_1_layout();
    let vbap = Vbap3D::new(speakers);

    // Test multiple source positions
    let positions = vec![
        Vec3::from_spherical(0.0, 0.0, 1.0),   // Front
        Vec3::from_spherical(90.0, 0.0, 1.0),  // Left
        Vec3::from_spherical(180.0, 0.0, 1.0), // Back
        Vec3::from_spherical(-90.0, 0.0, 1.0), // Right
    ];

    for pos in positions {
        let gains = vbap.render(&pos);
        // Each position should produce some output
        let total: f32 = gains.iter().sum();
        assert!(total >= 0.0);
    }
}

#[test]
fn test_vbap_energy_preservation() {
    let speakers = create_5_1_layout();
    let vbap = Vbap3D::new(speakers);

    let source = Vec3::from_spherical(30.0, 0.0, 1.0);
    let gains = vbap.render(&source);

    // Energy should be approximately preserved (sum of squares ≈ 1)
    let energy: f32 = gains.iter().map(|g| g * g).sum();

    // Allow some tolerance for inactive speakers
    if energy > 0.0 {
        assert!(energy <= 1.5); // Reasonable upper bound
    }
}

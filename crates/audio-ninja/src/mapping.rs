// SPDX-License-Identifier: Apache-2.0

use crate::{Position3, SpeakerDescriptor, SpeakerLayout};

/// Vector Base Amplitude Panning (VBAP) for object positioning.
/// Maps a 3D audio object position to speaker gains using the nearest speaker triangle/pair.
pub fn vbap_stereo(object_pos: &Position3, speakers: &[SpeakerDescriptor]) -> Vec<f32> {
    if speakers.len() < 2 {
        return vec![1.0; speakers.len()];
    }

    let mut gains = vec![0.0; speakers.len()];

    // Simple 2-speaker panning based on azimuth (x-y plane)
    let obj_azimuth = object_pos.x.atan2(object_pos.y);

    // Find the two nearest speakers in azimuth
    let mut best_pair = (0, 1);
    let mut best_score = f32::MAX;

    for i in 0..speakers.len() {
        for j in (i + 1)..speakers.len() {
            let az_i = speakers[i].position.x.atan2(speakers[i].position.y);
            let az_j = speakers[j].position.x.atan2(speakers[j].position.y);

            // Check if object is between these two speakers
            let min_az = az_i.min(az_j);
            let max_az = az_i.max(az_j);

            if obj_azimuth >= min_az && obj_azimuth <= max_az {
                let score = (az_i - obj_azimuth).abs() + (az_j - obj_azimuth).abs();
                if score < best_score {
                    best_score = score;
                    best_pair = (i, j);
                }
            }
        }
    }

    let (i, j) = best_pair;
    let az_i = speakers[i].position.x.atan2(speakers[i].position.y);
    let az_j = speakers[j].position.x.atan2(speakers[j].position.y);

    // Linear panning between the two speakers
    let span = (az_j - az_i).abs();
    if span > 0.0 {
        let t = ((obj_azimuth - az_i) / span).clamp(0.0, 1.0);
        gains[i] = (1.0 - t).sqrt();
        gains[j] = t.sqrt();
    } else {
        gains[i] = 1.0;
    }

    gains
}

/// Downmix N channels to M channels using basic rules.
pub fn downmix_channels(input: &[Vec<f32>], target_count: usize) -> Vec<Vec<f32>> {
    if input.is_empty() || target_count == 0 {
        return vec![];
    }

    let frame_len = input[0].len();

    if input.len() == target_count {
        return input.to_vec();
    }

    if input.len() > target_count {
        // Simple truncation or mix
        if target_count == 2 && input.len() >= 5 {
            // 5.1 to stereo: FL+C/2, FR+C/2
            let mut left = input[0].clone();
            let mut right = input[1].clone();

            if input.len() > 2 {
                // Add center to both
                for i in 0..frame_len {
                    left[i] += input[2][i] * 0.707;
                    right[i] += input[2][i] * 0.707;
                }
            }

            return vec![left, right];
        }

        return input[..target_count].to_vec();
    }

    // Upmix: pad with silence
    let mut output = input.to_vec();
    while output.len() < target_count {
        output.push(vec![0.0; frame_len]);
    }
    output
}

/// Upmix M channels to N channels with simple duplication or silence.
pub fn upmix_channels(input: &[Vec<f32>], target_count: usize) -> Vec<Vec<f32>> {
    downmix_channels(input, target_count)
}

/// Map ITU layout names to speaker descriptors.
pub fn layout_from_name(name: &str) -> Option<SpeakerLayout> {
    use crate::SpeakerRole::*;
    use std::time::Duration;

    let speakers = match name {
        "2.0" | "stereo" => vec![
            SpeakerDescriptor {
                id: "FL".into(),
                role: FrontLeft,
                position: Position3 {
                    x: -0.5,
                    y: 1.0,
                    z: 0.0,
                },
                max_spl_db: 110.0,
                latency: Duration::ZERO,
            },
            SpeakerDescriptor {
                id: "FR".into(),
                role: FrontRight,
                position: Position3 {
                    x: 0.5,
                    y: 1.0,
                    z: 0.0,
                },
                max_spl_db: 110.0,
                latency: Duration::ZERO,
            },
        ],
        "5.1" => vec![
            SpeakerDescriptor {
                id: "FL".into(),
                role: FrontLeft,
                position: Position3 {
                    x: -0.5,
                    y: 1.0,
                    z: 0.0,
                },
                max_spl_db: 110.0,
                latency: Duration::ZERO,
            },
            SpeakerDescriptor {
                id: "FR".into(),
                role: FrontRight,
                position: Position3 {
                    x: 0.5,
                    y: 1.0,
                    z: 0.0,
                },
                max_spl_db: 110.0,
                latency: Duration::ZERO,
            },
            SpeakerDescriptor {
                id: "C".into(),
                role: Center,
                position: Position3 {
                    x: 0.0,
                    y: 1.0,
                    z: 0.0,
                },
                max_spl_db: 110.0,
                latency: Duration::ZERO,
            },
            SpeakerDescriptor {
                id: "LFE".into(),
                role: Subwoofer,
                position: Position3 {
                    x: 0.0,
                    y: 0.0,
                    z: -0.5,
                },
                max_spl_db: 120.0,
                latency: Duration::ZERO,
            },
            SpeakerDescriptor {
                id: "SL".into(),
                role: SideLeft,
                position: Position3 {
                    x: -1.0,
                    y: -0.5,
                    z: 0.0,
                },
                max_spl_db: 110.0,
                latency: Duration::ZERO,
            },
            SpeakerDescriptor {
                id: "SR".into(),
                role: SideRight,
                position: Position3 {
                    x: 1.0,
                    y: -0.5,
                    z: 0.0,
                },
                max_spl_db: 110.0,
                latency: Duration::ZERO,
            },
        ],
        _ => return None,
    };

    Some(SpeakerLayout {
        name: name.into(),
        speakers,
    })
}

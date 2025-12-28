// SPDX-License-Identifier: Apache-2.0

//! 3D VBAP (Vector Base Amplitude Panning) for spatial audio rendering

/// 3D position in Cartesian coordinates
#[derive(Clone, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// Create from spherical coordinates (azimuth, elevation, radius)
    pub fn from_spherical(azimuth_deg: f32, elevation_deg: f32, radius: f32) -> Self {
        let az_rad = azimuth_deg.to_radians();
        let el_rad = elevation_deg.to_radians();

        let x = radius * el_rad.cos() * az_rad.sin();
        let y = radius * el_rad.cos() * az_rad.cos();
        let z = radius * el_rad.sin();

        Self { x, y, z }
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let len = self.length();
        if len > 0.0 {
            Self {
                x: self.x / len,
                y: self.y / len,
                z: self.z / len,
            }
        } else {
            Self::new(0.0, 0.0, 0.0)
        }
    }

    pub fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn sub(&self, other: &Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

/// Speaker in 3D space
#[derive(Clone, Debug)]
pub struct Speaker3D {
    pub id: usize,
    pub position: Vec3,
}

impl Speaker3D {
    pub fn new(id: usize, azimuth_deg: f32, elevation_deg: f32) -> Self {
        Self {
            id,
            position: Vec3::from_spherical(azimuth_deg, elevation_deg, 1.0),
        }
    }
}

/// Speaker triplet (triangle) for 3D VBAP
#[derive(Clone, Debug)]
pub struct SpeakerTriplet {
    pub speakers: [usize; 3],
    inverse_matrix: [[f32; 3]; 3],
}

impl SpeakerTriplet {
    /// Create a triplet from three speaker indices and compute inverse matrix
    pub fn new(s1: usize, s2: usize, s3: usize, speakers: &[Speaker3D]) -> Option<Self> {
        let p1 = &speakers[s1].position;
        let p2 = &speakers[s2].position;
        let p3 = &speakers[s3].position;

        // Build matrix from speaker vectors
        let matrix = [[p1.x, p2.x, p3.x], [p1.y, p2.y, p3.y], [p1.z, p2.z, p3.z]];

        // Compute determinant
        let det = matrix[0][0] * (matrix[1][1] * matrix[2][2] - matrix[1][2] * matrix[2][1])
            - matrix[0][1] * (matrix[1][0] * matrix[2][2] - matrix[1][2] * matrix[2][0])
            + matrix[0][2] * (matrix[1][0] * matrix[2][1] - matrix[1][1] * matrix[2][0]);

        if det.abs() < 1e-6 {
            return None; // Speakers are coplanar or degenerate
        }

        // Compute inverse matrix
        let inv_det = 1.0 / det;
        let inverse_matrix = [
            [
                (matrix[1][1] * matrix[2][2] - matrix[1][2] * matrix[2][1]) * inv_det,
                (matrix[0][2] * matrix[2][1] - matrix[0][1] * matrix[2][2]) * inv_det,
                (matrix[0][1] * matrix[1][2] - matrix[0][2] * matrix[1][1]) * inv_det,
            ],
            [
                (matrix[1][2] * matrix[2][0] - matrix[1][0] * matrix[2][2]) * inv_det,
                (matrix[0][0] * matrix[2][2] - matrix[0][2] * matrix[2][0]) * inv_det,
                (matrix[0][2] * matrix[1][0] - matrix[0][0] * matrix[1][2]) * inv_det,
            ],
            [
                (matrix[1][0] * matrix[2][1] - matrix[1][1] * matrix[2][0]) * inv_det,
                (matrix[0][1] * matrix[2][0] - matrix[0][0] * matrix[2][1]) * inv_det,
                (matrix[0][0] * matrix[1][1] - matrix[0][1] * matrix[1][0]) * inv_det,
            ],
        ];

        Some(Self {
            speakers: [s1, s2, s3],
            inverse_matrix,
        })
    }

    /// Calculate gains for a sound source position
    pub fn calculate_gains(&self, source: &Vec3) -> Option<[f32; 3]> {
        let src_norm = source.normalize();

        // Multiply inverse matrix with source vector to get gains
        let g1 = self.inverse_matrix[0][0] * src_norm.x
            + self.inverse_matrix[0][1] * src_norm.y
            + self.inverse_matrix[0][2] * src_norm.z;
        let g2 = self.inverse_matrix[1][0] * src_norm.x
            + self.inverse_matrix[1][1] * src_norm.y
            + self.inverse_matrix[1][2] * src_norm.z;
        let g3 = self.inverse_matrix[2][0] * src_norm.x
            + self.inverse_matrix[2][1] * src_norm.y
            + self.inverse_matrix[2][2] * src_norm.z;

        // Check if all gains are non-negative (source is inside the triplet)
        if g1 >= 0.0 && g2 >= 0.0 && g3 >= 0.0 {
            // Normalize gains for energy preservation
            let sum = g1 * g1 + g2 * g2 + g3 * g3;
            if sum > 0.0 {
                let norm = sum.sqrt();
                Some([g1 / norm, g2 / norm, g3 / norm])
            } else {
                None
            }
        } else {
            None
        }
    }
}

/// 3D VBAP renderer
pub struct Vbap3D {
    speakers: Vec<Speaker3D>,
    triplets: Vec<SpeakerTriplet>,
}

impl Vbap3D {
    pub fn new(speakers: Vec<Speaker3D>) -> Self {
        let mut renderer = Self {
            speakers,
            triplets: Vec::new(),
        };
        renderer.find_triplets();
        renderer
    }

    /// Find all valid speaker triplets (Delaunay triangulation approximation)
    fn find_triplets(&mut self) {
        let n = self.speakers.len();

        if n < 3 {
            return;
        }

        // Brute force: try all combinations of 3 speakers
        for i in 0..n {
            for j in (i + 1)..n {
                for k in (j + 1)..n {
                    if let Some(triplet) = SpeakerTriplet::new(i, j, k, &self.speakers) {
                        // Check if triplet forms a valid triangle (not too flat)
                        if self.is_valid_triplet(&triplet) {
                            self.triplets.push(triplet);
                        }
                    }
                }
            }
        }
    }

    /// Check if a triplet is valid (forms a reasonable triangle)
    fn is_valid_triplet(&self, triplet: &SpeakerTriplet) -> bool {
        let [i, j, k] = triplet.speakers;
        let p1 = &self.speakers[i].position;
        let p2 = &self.speakers[j].position;
        let p3 = &self.speakers[k].position;

        // Compute normal vector of the triangle
        let v1 = p2.sub(p1);
        let v2 = p3.sub(p1);
        let normal = v1.cross(&v2);

        // Triangle should have sufficient area
        normal.length() > 0.1
    }

    /// Render a sound source to speaker gains
    pub fn render(&self, source: &Vec3) -> Vec<f32> {
        let mut gains = vec![0.0; self.speakers.len()];

        // Find the triplet that contains the source
        for triplet in &self.triplets {
            if let Some(trip_gains) = triplet.calculate_gains(source) {
                for (i, &speaker_idx) in triplet.speakers.iter().enumerate() {
                    gains[speaker_idx] = trip_gains[i];
                }
                break;
            }
        }

        gains
    }

    pub fn speaker_count(&self) -> usize {
        self.speakers.len()
    }

    pub fn triplet_count(&self) -> usize {
        self.triplets.len()
    }
}

/// Create a standard speaker layout
pub fn create_stereo_layout() -> Vec<Speaker3D> {
    vec![
        Speaker3D::new(0, 30.0, 0.0),  // Front Left
        Speaker3D::new(1, -30.0, 0.0), // Front Right
    ]
}

pub fn create_5_1_layout() -> Vec<Speaker3D> {
    vec![
        Speaker3D::new(0, 30.0, 0.0),   // Front Left
        Speaker3D::new(1, -30.0, 0.0),  // Front Right
        Speaker3D::new(2, 0.0, 0.0),    // Center
        Speaker3D::new(3, 0.0, -45.0),  // LFE (below)
        Speaker3D::new(4, 110.0, 0.0),  // Surround Left
        Speaker3D::new(5, -110.0, 0.0), // Surround Right
    ]
}

pub fn create_7_1_4_layout() -> Vec<Speaker3D> {
    vec![
        // Ground level
        Speaker3D::new(0, 30.0, 0.0),   // Front Left
        Speaker3D::new(1, -30.0, 0.0),  // Front Right
        Speaker3D::new(2, 0.0, 0.0),    // Center
        Speaker3D::new(3, 0.0, -45.0),  // LFE
        Speaker3D::new(4, 90.0, 0.0),   // Side Left
        Speaker3D::new(5, -90.0, 0.0),  // Side Right
        Speaker3D::new(6, 135.0, 0.0),  // Back Left
        Speaker3D::new(7, -135.0, 0.0), // Back Right
        // Height level
        Speaker3D::new(8, 45.0, 45.0),    // Top Front Left
        Speaker3D::new(9, -45.0, 45.0),   // Top Front Right
        Speaker3D::new(10, 135.0, 45.0),  // Top Back Left
        Speaker3D::new(11, -135.0, 45.0), // Top Back Right
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec3_from_spherical() {
        let v = Vec3::from_spherical(0.0, 0.0, 1.0);
        assert!((v.y - 1.0).abs() < 0.01); // Front
        assert!(v.x.abs() < 0.01);
        assert!(v.z.abs() < 0.01);
    }

    #[test]
    fn test_vec3_normalize() {
        let v = Vec3::new(3.0, 4.0, 0.0);
        let n = v.normalize();
        assert!((n.length() - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_vec3_dot_product() {
        let v1 = Vec3::new(1.0, 0.0, 0.0);
        let v2 = Vec3::new(0.0, 1.0, 0.0);
        assert_eq!(v1.dot(&v2), 0.0);
    }

    #[test]
    fn test_vbap_3d_stereo() {
        let speakers = create_stereo_layout();
        let vbap = Vbap3D::new(speakers);

        assert_eq!(vbap.speaker_count(), 2);

        // Source in center
        let gains = vbap.render(&Vec3::from_spherical(0.0, 0.0, 1.0));
        assert_eq!(gains.len(), 2);
    }

    #[test]
    fn test_vbap_3d_5_1() {
        let speakers = create_5_1_layout();
        let vbap = Vbap3D::new(speakers);

        assert_eq!(vbap.speaker_count(), 6);
        assert!(vbap.triplet_count() > 0);
    }
}

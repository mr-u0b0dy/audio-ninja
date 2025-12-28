// SPDX-License-Identifier: Apache-2.0

use crate::calibration::CalibrationSolution;

#[derive(Clone, Debug, PartialEq)]
pub struct CamillaDspConfig {
    pub devices: DeviceConfig,
    pub filters: Vec<FilterConfig>,
    pub mixers: Vec<MixerConfig>,
    pub pipeline: Vec<PipelineStep>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DeviceConfig {
    pub sample_rate: u32,
    pub chunk_size: u32,
    pub channels: u32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FilterConfig {
    pub name: String,
    pub filter_type: FilterType,
}

#[derive(Clone, Debug, PartialEq)]
pub enum FilterType {
    Biquad { coeffs: Vec<f32> },
    Conv { filename: String },
    Delay { delay_samples: u32 },
}

#[derive(Clone, Debug, PartialEq)]
pub struct MixerConfig {
    pub name: String,
    pub channels_in: u32,
    pub channels_out: u32,
    pub mapping: Vec<Vec<f32>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PipelineStep {
    pub filter_name: String,
    pub channel: Option<u32>,
}

impl CamillaDspConfig {
    pub fn to_yaml(&self) -> String {
        let mut yaml = String::new();

        yaml.push_str("devices:\n");
        yaml.push_str(&format!("  samplerate: {}\n", self.devices.sample_rate));
        yaml.push_str(&format!("  chunksize: {}\n", self.devices.chunk_size));
        yaml.push_str(&format!("  channels: {}\n\n", self.devices.channels));

        if !self.filters.is_empty() {
            yaml.push_str("filters:\n");
            for filter in &self.filters {
                yaml.push_str(&format!("  {}:\n", filter.name));
                match &filter.filter_type {
                    FilterType::Biquad { coeffs } => {
                        yaml.push_str("    type: Biquad\n");
                        yaml.push_str("    parameters:\n");
                        if coeffs.len() >= 5 {
                            yaml.push_str(&format!("      b0: {}\n", coeffs[0]));
                            yaml.push_str(&format!("      b1: {}\n", coeffs[1]));
                            yaml.push_str(&format!("      b2: {}\n", coeffs[2]));
                            yaml.push_str(&format!("      a1: {}\n", coeffs[3]));
                            yaml.push_str(&format!("      a2: {}\n", coeffs[4]));
                        }
                    }
                    FilterType::Conv { filename } => {
                        yaml.push_str("    type: Conv\n");
                        yaml.push_str("    parameters:\n");
                        yaml.push_str(&format!("      filename: {}\n", filename));
                    }
                    FilterType::Delay { delay_samples } => {
                        yaml.push_str("    type: Delay\n");
                        yaml.push_str("    parameters:\n");
                        yaml.push_str(&format!("      delay: {}\n", delay_samples));
                    }
                }
            }
            yaml.push('\n');
        }

        if !self.pipeline.is_empty() {
            yaml.push_str("pipeline:\n");
            for step in &self.pipeline {
                yaml.push_str("  - type: Filter\n");
                if let Some(ch) = step.channel {
                    yaml.push_str(&format!("    channel: {}\n", ch));
                }
                yaml.push_str("    names:\n");
                yaml.push_str(&format!("      - {}\n", step.filter_name));
            }
        }

        yaml
    }
}

pub fn solution_to_camilladsp(
    solution: &CalibrationSolution,
    sample_rate: u32,
    channels: u32,
) -> CamillaDspConfig {
    let mut filters = Vec::new();
    let mut pipeline = Vec::new();

    // Add delay filters
    for (idx, delay) in solution.delays.iter().enumerate() {
        let delay_samples = (delay.as_secs_f32() * sample_rate as f32) as u32;
        if delay_samples > 0 {
            let filter_name = format!("delay_ch{}", idx);
            filters.push(FilterConfig {
                name: filter_name.clone(),
                filter_type: FilterType::Delay { delay_samples },
            });
            pipeline.push(PipelineStep {
                filter_name,
                channel: Some(idx as u32),
            });
        }
    }

    // Add PEQ filters
    for (idx, biquad) in solution.peq.iter().enumerate() {
        let filter_name = format!("peq{}", idx);
        filters.push(FilterConfig {
            name: filter_name.clone(),
            filter_type: FilterType::Biquad {
                coeffs: vec![
                    biquad.coeffs.b0,
                    biquad.coeffs.b1,
                    biquad.coeffs.b2,
                    biquad.coeffs.a1,
                    biquad.coeffs.a2,
                ],
            },
        });
        pipeline.push(PipelineStep {
            filter_name,
            channel: None, // Apply to all channels
        });
    }

    // Add FIR convolution if present
    if let Some(_fir) = &solution.fir {
        filters.push(FilterConfig {
            name: "fir_correction".into(),
            filter_type: FilterType::Conv {
                filename: "fir_coeffs.wav".into(),
            },
        });
        pipeline.push(PipelineStep {
            filter_name: "fir_correction".into(),
            channel: None,
        });
    }

    CamillaDspConfig {
        devices: DeviceConfig {
            sample_rate,
            chunk_size: 1024,
            channels,
        },
        filters,
        mixers: vec![],
        pipeline,
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct BruteFirConfig {
    pub sample_rate: u32,
    pub filter_length: u32,
    pub partitions: u32,
    pub coeffs: Vec<Vec<f32>>,
}

impl BruteFirConfig {
    pub fn to_config_file(&self) -> String {
        let mut config = String::new();

        config.push_str(&format!("sampling_rate: {};\n", self.sample_rate));
        config.push_str(&format!("filter_length: {};\n", self.filter_length));
        config.push_str(&format!("partitions: {};\n", self.partitions));
        config.push_str("float_bits: 32;\n\n");

        for (idx, coeffs) in self.coeffs.iter().enumerate() {
            config.push_str(&format!("coeff \"ch{}\" {{\n", idx));
            config.push_str("  format: \"FLOAT_LE\";\n");
            config.push_str(&format!("  filename: \"ch{}_coeffs.raw\";\n", idx));
            config.push_str("};\n\n");
        }

        config
    }
}

pub fn solution_to_brutefir(solution: &CalibrationSolution, sample_rate: u32) -> BruteFirConfig {
    let coeffs = if let Some(fir) = &solution.fir {
        vec![fir.taps.clone()]
    } else {
        vec![vec![1.0]] // Impulse
    };

    let filter_length = coeffs.first().map(|c| c.len()).unwrap_or(1) as u32;
    let partitions = (filter_length / 512).max(1);

    BruteFirConfig {
        sample_rate,
        filter_length,
        partitions,
        coeffs,
    }
}

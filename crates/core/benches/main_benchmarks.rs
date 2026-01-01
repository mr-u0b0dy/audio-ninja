// SPDX-License-Identifier: Apache-2.0

//! Core Performance Benchmarks for audio-ninja library

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use audio_ninja::{
    vbap::{Vec3, Speaker3D, Vbap3D},
    loudness::LoudnessMeter,
    AudioBlock,
};

// VBAP Renderer Benchmarks
fn bench_vbap_stereo_render(c: &mut Criterion) {
    c.bench_function("vbap_stereo_2speakers", |b| {
        let speakers = vec![
            Speaker3D::new(0, -30.0, 0.0),
            Speaker3D::new(1, 30.0, 0.0),
        ];
        let renderer = Vbap3D::new(speakers);
        let source = black_box(Vec3::from_spherical(0.0, 0.0, 1.0));
        
        b.iter(|| {
            renderer.render(&source)
        });
    });
}

fn bench_vbap_5_1_render(c: &mut Criterion) {
    c.bench_function("vbap_5.1_6speakers", |b| {
        let speakers = vec![
            Speaker3D::new(0, -30.0, 0.0),  // L
            Speaker3D::new(1, 30.0, 0.0),   // R
            Speaker3D::new(2, 0.0, 0.0),    // C
            Speaker3D::new(3, 110.0, 0.0),  // LS
            Speaker3D::new(4, -110.0, 0.0), // RS
            Speaker3D::new(5, 0.0, -90.0),  // LFE
        ];
        let renderer = Vbap3D::new(speakers);
        let source = black_box(Vec3::from_spherical(45.0, 15.0, 1.0));
        
        b.iter(|| {
            renderer.render(&source)
        });
    });
}

fn bench_vbap_elevation_sweep(c: &mut Criterion) {
    let mut group = c.benchmark_group("vbap_elevation");
    
    for elevation in [0.0, 15.0, 30.0, 45.0, 60.0].iter() {
        let speakers = vec![
            Speaker3D::new(0, -30.0, 0.0),
            Speaker3D::new(1, 30.0, 0.0),
            Speaker3D::new(2, 0.0, 90.0),
        ];
        let renderer = Vbap3D::new(speakers);
        let source = black_box(Vec3::from_spherical(0.0, *elevation, 1.0));
        
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}deg", elevation)),
            elevation,
            |b, _| {
                b.iter(|| renderer.render(&source));
            },
        );
    }
    
    group.finish();
}

// HRTF Processing Benchmarks - Vec3 operations (HRTF requires HrtfDatabase setup)
fn bench_vec3_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("vec3_operations");
    
    let v1 = black_box(Vec3::from_spherical(30.0, 15.0, 1.0));
    let v2 = black_box(Vec3::from_spherical(-30.0, -15.0, 1.0));
    
    group.bench_function("normalize", |b| {
        b.iter(|| v1.normalize());
    });
    
    group.bench_function("dot_product", |b| {
        b.iter(|| v1.dot(&v2));
    });
    
    group.bench_function("cross_product", |b| {
        b.iter(|| v1.cross(&v2));
    });
    
    group.bench_function("magnitude", |b| {
        b.iter(|| v1.length());
    });
    
    group.finish();
}

// Loudness Measurement Benchmarks
fn bench_loudness_mono(c: &mut Criterion) {
    c.bench_function("loudness_mono_1sec", |b| {
        let audio: Vec<f32> = (0..48000)
            .map(|i| ((i as f32 / 48000.0) * 2.0 * std::f32::consts::PI).sin() * 0.1)
            .collect();
        let block = AudioBlock {
            channels: vec![audio],
            sample_rate: 48000,
        };
        let block = black_box(block);
        
        b.iter(|| {
            let mut meter = LoudnessMeter::new(48000);
            meter.measure_integrated_loudness(&block)
        });
    });
}

fn bench_loudness_stereo(c: &mut Criterion) {
    c.bench_function("loudness_stereo_1sec", |b| {
        let mut left = vec![0.0; 48000];
        let mut right = vec![0.0; 48000];
        for i in 0..48000 {
            left[i] = ((i as f32 / 48000.0) * 2.0 * std::f32::consts::PI).sin() * 0.1;
            right[i] = ((i as f32 / 48000.0) * 2.0 * std::f32::consts::PI).cos() * 0.1;
        }
        let block = AudioBlock {
            channels: vec![left, right],
            sample_rate: 48000,
        };
        let block = black_box(block);
        
        b.iter(|| {
            let mut meter = LoudnessMeter::new(48000);
            meter.measure_integrated_loudness(&block)
        });
    });
}

fn bench_loudness_5_1(c: &mut Criterion) {
    c.bench_function("loudness_5.1_1sec", |b| {
        let channels: Vec<Vec<f32>> = (0..6)
            .map(|ch| {
                (0..48000)
                    .map(|i| {
                        let freq = (i as f32 / 48000.0) * 2.0 * std::f32::consts::PI;
                        (freq + (ch as f32) * 0.2).sin() * 0.1
                    })
                    .collect()
            })
            .collect();
        let block = AudioBlock {
            channels,
            sample_rate: 48000,
        };
        let block = black_box(block);
        
        b.iter(|| {
            let mut meter = LoudnessMeter::new(48000);
            meter.measure_integrated_loudness(&block)
        });
    });
}

fn bench_loudness_types(c: &mut Criterion) {
    let mut group = c.benchmark_group("loudness_types");
    
    let mut left = vec![0.0; 48000];
    let mut right = vec![0.0; 48000];
    for i in 0..48000 {
        left[i] = ((i as f32 / 48000.0) * 2.0 * std::f32::consts::PI).sin() * 0.1;
        right[i] = ((i as f32 / 48000.0) * 2.0 * std::f32::consts::PI).cos() * 0.1;
    }
    let block = AudioBlock {
        channels: vec![left, right],
        sample_rate: 48000,
    };
    let block = black_box(block);
    
    group.bench_function("integrated", |b| {
        b.iter(|| {
            let mut meter = LoudnessMeter::new(48000);
            meter.measure_integrated_loudness(&block)
        });
    });
    
    group.bench_function("short_term", |b| {
        b.iter(|| {
            let mut meter = LoudnessMeter::new(48000);
            meter.measure_short_term_loudness(&block)
        });
    });
    
    group.bench_function("loudness_range", |b| {
        b.iter(|| {
            let mut meter = LoudnessMeter::new(48000);
            meter.measure_loudness_range(&block)
        });
    });
    
    group.finish();
}

criterion_group!(
    benches,
    bench_vbap_stereo_render,
    bench_vbap_5_1_render,
    bench_vbap_elevation_sweep,
    bench_vec3_operations,
    bench_loudness_mono,
    bench_loudness_stereo,
    bench_loudness_5_1,
    bench_loudness_types
);

criterion_main!(benches);
# Audio Ninja Examples

This directory contains example applications demonstrating various features of Audio Ninja.

## Examples

### Basic VBAP Rendering
```rust
// examples/vbap_basic.rs
use audio_ninja::vbap::{Vbap3D, create_5_1_layout, Vec3};

fn main() {
    // Create a 5.1 surround layout
    let speakers = create_5_1_layout();
    let vbap = Vbap3D::new(speakers);
    
    // Render a source at 45° left, 10° elevated
    let source = Vec3::from_spherical(45.0, 10.0, 1.0);
    let gains = vbap.render(&source);
    
    println!("Speaker gains: {:?}", gains);
}
```

### HOA Decoding
```rust
// examples/hoa_basic.rs
use audio_ninja::hoa::{HoaDecoder, AmbisonicOrder, DecodingMode, create_7_1_4_hoa_layout};

fn main() {
    // Create 2nd order decoder for 7.1.4 layout
    let speakers = create_7_1_4_hoa_layout();
    let decoder = HoaDecoder::new(
        AmbisonicOrder::SECOND,
        DecodingMode::MaxRE,
        speakers
    );
    
    // Decode ambisonic signal
    let ambisonic_input = vec![1.0, 0.5, 0.3, 0.2, 0.1, 0.05, 0.08, 0.06, 0.04];
    let speaker_output = decoder.decode(&ambisonic_input);
    
    println!("Decoded to {} speakers", speaker_output.len());
}
```

### Network Streaming
```rust
// examples/network_sender.rs
use audio_ninja::network::UdpRtpSender;
use audio_ninja::AudioBlock;

fn main() -> anyhow::Result<()> {
    let mut sender = UdpRtpSender::new(
        "0.0.0.0:0",
        "192.168.1.100:5004",
        12345
    )?;
    
    // Enable FEC
    sender.enable_fec(8);
    
    // Create audio block
    let audio = AudioBlock {
        channels: vec![vec![0.5; 480], vec![0.5; 480]],
        sample_rate: 48000,
    };
    
    sender.send_block(&audio)?;
    println!("Sent audio block");
    
    Ok(())
}
```

### Room Calibration
```rust
// examples/calibration_sweep.rs
use audio_ninja::calibration::{generate_log_sweep, extract_ir_from_sweep, compute_delay};

fn main() {
    // Generate 2-second sweep from 20Hz to 20kHz
    let sweep = generate_log_sweep(48000, 2.0, 20.0, 20000.0);
    println!("Generated sweep with {} samples", sweep.len());
    
    // In a real application, you would:
    // 1. Play the sweep through a speaker
    // 2. Record the response with a microphone
    // 3. Extract the impulse response
    // 4. Analyze and design correction filters
}
```

## Running Examples

```bash
# Run a specific example
cargo run --example vbap_basic

# With release optimizations
cargo run --release --example hoa_basic
```

## Contributing Examples

To add a new example:

1. Create a new file in `examples/`
2. Add a brief description here
3. Include clear comments in the code
4. Test that it compiles and runs

See [CONTRIBUTING.md](../CONTRIBUTING.md) for more details.

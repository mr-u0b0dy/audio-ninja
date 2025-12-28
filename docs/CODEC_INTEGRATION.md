# Codec Integration Guide

This guide covers integrating real codec libraries into Audio Ninja's demuxer/decoder pipeline, replacing the current stub implementations.

## Current Status

Audio Ninja currently uses stub implementations for media demuxing and decoding:

**File**: `crates/core/src/ffmpeg.rs`

- `StubDemuxer`: Placeholder for FFmpeg demuxing (file format extraction)
- `StubDecoder`: Placeholder for audio codec decoding (Opus, AAC, FLAC, PCM)

## Codec Support Matrix

| Codec | Container | Library | Status | Priority |
|-------|-----------|---------|--------|----------|
| PCM | WAV, RAW | stdlib | ✅ Possible | High |
| Opus | Ogg, Matroska, WebM | opus | ⏳ Planned | High |
| AAC | MP4, M4A, ADTS | faac/fdk-aac | ⏳ Planned | Medium |
| FLAC | FLAC, Matroska | FLAC | ⏳ Planned | Medium |
| AC-3 | MPEG-TS, Matroska | ac3 (libavcodec) | ⏳ Deferred | Low |
| E-AC-3 | MPEG-TS, Matroska | eac3 (libavcodec) | ⏳ Deferred | Low |
| TrueHD | MKV, Blu-ray | truehd (libavcodec) | ⏳ Deferred | Low |

## Codec Library Options

### Option 1: FFmpeg (Recommended for Most Codecs)

**Pros:**
- Supports 100+ codecs and containers
- Widely used and well-maintained
- Strong licensing infrastructure (LGPL)
- Bindings available: `ffmpeg-sys`, `ffmpeg-next`

**Cons:**
- Heavy dependency (~100 MB)
- Compile-time complexity
- Licensing considerations for distribution

**Codecs**: AAC, AC-3, E-AC-3, TrueHD, MP3, Vorbis, and many more

**Bindings**:
```rust
// ffmpeg-next crate
cargo add ffmpeg-next
```

### Option 2: Opus (Dedicated Library)

**Pros:**
- Lightweight, focused library
- Excellent audio quality
- Modern, actively maintained
- IETF standard

**Cons:**
- Single codec only
- Requires libopus system dependency

**Bindings**:
```rust
cargo add opus
```

### Option 3: FLAC (Rust-based)

**Pros:**
- Pure Rust implementation
- No system dependencies
- Fast, zero-copy parsing possible
- Good test coverage

**Cons:**
- Single codec
- Slightly slower than native C

**Bindings**:
```rust
cargo add metaflac  # FLAC metadata
cargo add flac      # Pure Rust FLAC decoder
```

### Option 4: Hybrid Approach (Recommended)

Combine specialized libraries for best performance:

- **FLAC**: Use pure-Rust `flac` or `metaflac`
- **Opus**: Use `opus` bindings
- **AAC/AC-3**: Use FFmpeg bindings for complex codecs
- **PCM**: Implement directly (simple byte copying + format conversion)

## Implementation Path

### Phase 1: PCM Support (Quick Win)

PCM audio needs minimal processing—just byte reordering and format conversion.

**File**: `crates/core/src/ffmpeg.rs`

```rust
use byteorder::{LittleEndian, BigEndian, ReadBytesExt};

impl Decoder for PcmDecoder {
    fn decode(&mut self, packet: &CodecPacket) -> Result<Option<DecodedFrame>, FfmpegError> {
        let mut cursor = std::io::Cursor::new(&packet.data);
        let channels = self.codec_config.channels as usize;
        let sample_rate = self.codec_config.sample_rate;
        
        // Read interleaved samples
        let sample_count = packet.data.len() / (channels * 4); // f32 = 4 bytes
        let mut channels_data = vec![vec![]; channels];
        
        for _ in 0..sample_count {
            for ch in 0..channels {
                let sample = cursor.read_f32::<LittleEndian>()?;
                channels_data[ch].push(sample);
            }
        }
        
        Ok(Some(DecodedFrame {
            pts: packet.pts,
            sample_rate,
            channels: channels_data,
        }))
    }
}
```

**Testing**: Add unit tests with synthetic PCM data.

### Phase 2: FLAC Support

FLAC is well-suited for lossless audio. Use the pure-Rust `flac` crate.

**Step 1**: Add dependency
```toml
[dependencies]
flac = "0.3"
```

**Step 2**: Implement FLACDecoder
```rust
use flac::StreamReader;

pub struct FlacDecoder {
    reader: Option<StreamReader<std::io::Cursor<Vec<u8>>>>,
    codec: Option<CodecConfig>,
}

impl Decoder for FlacDecoder {
    fn decode(&mut self, packet: &CodecPacket) -> Result<Option<DecodedFrame>, FfmpegError> {
        // Initialize reader if needed
        if self.reader.is_none() {
            let cursor = std::io::Cursor::new(packet.data.clone());
            self.reader = Some(StreamReader::new(cursor)
                .map_err(|e| FfmpegError::Decode(e.to_string()))?);
        }
        
        // Read frames and convert to f32
        let mut channels = vec![];
        for frame in self.reader.iter() {
            // Convert frame samples to f32 channels
        }
        
        Ok(Some(DecodedFrame { /* ... */ }))
    }
}
```

### Phase 3: Opus Support

Opus requires `libopus` system library.

**Step 1**: Add dependency
```toml
[dependencies]
opus = "0.3"
```

**Step 2**: Install system library
```bash
# Linux
sudo apt-get install libopus-dev

# macOS
brew install opus

# Windows
# Included in pre-built binaries
```

**Step 3**: Implement OpusDecoder
```rust
use opus::Decoder as OpusDecoder_;

pub struct OpusDecoder {
    decoder: Option<OpusDecoder_>,
    codec: Option<CodecConfig>,
}

impl Decoder for OpusDecoder {
    fn decode(&mut self, packet: &CodecPacket) -> Result<Option<DecodedFrame>, FfmpegError> {
        let decoder = self.decoder.as_mut()
            .ok_or(FfmpegError::Decode("not configured".into()))?;
        
        // Decode Opus packet
        let frame_size = packet.data.len() as i32; // or determine from packet
        let mut out = vec![0.0; frame_size as usize];
        
        decoder.decode_float(&packet.data, &mut out, false)
            .map_err(|e| FfmpegError::Decode(e.to_string()))?;
        
        // Split into stereo channels
        Ok(Some(DecodedFrame { /* ... */ }))
    }
}
```

### Phase 4: FFmpeg Integration (Complex Codecs)

For AAC, AC-3, E-AC-3, use FFmpeg bindings.

**Step 1**: Add dependency
```toml
[dependencies]
ffmpeg-next = "6.0"
```

**Step 2**: Update Cargo.toml features
```toml
[features]
ffmpeg-support = ["ffmpeg-next"]

[dependencies.ffmpeg-next]
version = "6.0"
optional = true
```

**Step 3**: Implement FFmpegDecoder
```rust
#[cfg(feature = "ffmpeg-support")]
use ffmpeg_next as ffmpeg;

#[cfg(feature = "ffmpeg-support")]
pub struct FfmpegDecoder {
    context: Option<ffmpeg::decoder::Audio>,
}

#[cfg(feature = "ffmpeg-support")]
impl Decoder for FfmpegDecoder {
    fn decode(&mut self, packet: &CodecPacket) -> Result<Option<DecodedFrame>, FfmpegError> {
        // Use ffmpeg::decoder to decode packet
    }
}
```

### Phase 5: Codec Factory Pattern

Implement factory to select decoder based on codec type:

```rust
pub fn create_decoder(codec_config: &CodecConfig) -> Result<Box<dyn Decoder>, FfmpegError> {
    match codec_config.codec_type {
        CodecType::PCM => Ok(Box::new(PcmDecoder::new())),
        CodecType::FLAC => Ok(Box::new(FlacDecoder::new())),
        CodecType::Opus => Ok(Box::new(OpusDecoder::new())),
        #[cfg(feature = "ffmpeg-support")]
        CodecType::AAC => Ok(Box::new(FfmpegDecoder::new(ffmpeg::codec::Id::AAC))),
        _ => Err(FfmpegError::Format(format!(
            "Codec {:?} not supported", 
            codec_config.codec_type
        ))),
    }
}
```

## Demuxing Implementation

### Using FFmpeg for Demuxing

```rust
#[cfg(feature = "ffmpeg-support")]
pub struct FfmpegDemuxer {
    context: Option<ffmpeg::format::context::Input>,
    stream_index: usize,
}

#[cfg(feature = "ffmpeg-support")]
impl Demuxer for FfmpegDemuxer {
    fn open(&mut self, cfg: DemuxConfig) -> Result<IamfStreamConfig, FfmpegError> {
        let mut context = ffmpeg::format::input(&cfg.input_path)
            .map_err(|e| FfmpegError::Init(e.to_string()))?;
        
        // Find audio stream
        let stream_index = context
            .streams()
            .position(|s| s.codec().medium() == ffmpeg::media::Type::Audio)
            .ok_or(FfmpegError::Format("no audio stream".into()))?;
        
        let stream = context.stream(stream_index as u32).unwrap();
        let codec_params = stream.codec();
        
        self.stream_index = stream_index;
        self.context = Some(context);
        
        Ok(IamfStreamConfig {
            sample_rate: codec_params.rate(),
            channel_count: codec_params.channels() as u32,
            frame_duration: Duration::from_millis(20),
            // ... extract metadata
        })
    }
    
    fn read_packet(&mut self) -> Result<Option<CodecPacket>, FfmpegError> {
        let context = self.context.as_mut()
            .ok_or(FfmpegError::Format("not opened".into()))?;
        
        for (packet, _) in context.packets() {
            if packet.stream() == self.stream_index {
                return Ok(Some(CodecPacket {
                    pts: packet.pts().unwrap_or(0),
                    dts: packet.dts().unwrap_or(0),
                    data: packet.data().unwrap_or(&[]).to_vec(),
                    is_keyframe: packet.is_key(),
                }));
            }
        }
        
        Ok(None)
    }
}
```

## Testing Codec Implementations

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_pcm_decoder() {
        let mut decoder = PcmDecoder::new();
        let codec = CodecConfig { sample_rate: 48000, channels: 2, ..Default::default() };
        decoder.configure(codec).unwrap();
        
        // Create synthetic PCM data
        let pcm_data = vec![0.0_f32; 2048]; // 2 channels, 1024 samples
        let packet = CodecPacket {
            pts: 0,
            dts: 0,
            data: pcm_data.into_iter()
                .flat_map(|f| f.to_le_bytes())
                .collect(),
            is_keyframe: false,
        };
        
        let frame = decoder.decode(&packet).unwrap();
        assert!(frame.is_some());
    }
    
    #[test]
    fn test_flac_decoder() {
        // Load real FLAC test file
        let flac_data = include_bytes!("../test_data/sample.flac");
        let packet = CodecPacket {
            pts: 0,
            dts: 0,
            data: flac_data.to_vec(),
            is_keyframe: true,
        };
        
        let mut decoder = FlacDecoder::new();
        let frame = decoder.decode(&packet).unwrap();
        assert!(frame.is_some());
    }
}
```

### Integration Tests

```bash
# Test with real audio files
cargo test --features ffmpeg-support --test codec_integration
```

## System Dependencies

Install codec libraries for your platform:

**Linux (Ubuntu/Debian)**:
```bash
sudo apt-get install \
    libflac-dev \
    libopus-dev \
    libavformat-dev \
    libavcodec-dev \
    libavutil-dev
```

**macOS**:
```bash
brew install flac opus ffmpeg
```

**Windows**:
- FFmpeg: Download from https://ffmpeg.org/download.html
- Opus: Included in pre-built dependencies
- FLAC: Included via package managers

## Performance Considerations

### Streaming vs. Full Decode
- Use streaming decoders for real-time playback
- Full decode for quick verification

### Memory Management
- Use circular buffers for demuxing
- Implement frame pooling for decoders
- Clear decoded frames after rendering

### Optimization
- Enable SIMD in codec libraries: `-C target-cpu=native`
- Profile with `perf` or `Instruments`
- Use `cargo bench` for codec performance

## Licensing

**Important**: Verify codec library licenses for distribution:

- **FLAC**: BSD 3-Clause ✅ Compatible with Apache-2.0
- **Opus**: BSD 3-Clause ✅ Compatible with Apache-2.0
- **FFmpeg**: LGPL 2.1+ ⚠️ Requires binary distribution compliance
- **fdk-aac**: Apache-2.0 ✅ Direct compatible

Document all dependencies in `NOTICE` file if using LGPL or proprietary codecs.

## CI/CD Integration

Add optional codec feature to CI builds:

```yaml
# .github/workflows/ci.yml
- name: Test with codec support
  run: cargo test --features ffmpeg-support --workspace
```

## Implementation Roadmap

**Month 1**:
- [ ] PCM decoder (simple, ~50 lines)
- [ ] Add unit tests
- [ ] Update pipeline.rs to use real decoders

**Month 2**:
- [ ] FLAC decoder integration
- [ ] Opus decoder integration
- [ ] Demuxer factory pattern

**Month 3**:
- [ ] FFmpeg integration (AAC, AC-3, E-AC-3)
- [ ] Performance optimization
- [ ] Integration tests with real audio files

**Month 4+**:
- [ ] Advanced codec features (ATMOS metadata, TrueHD)
- [ ] Codec parameter negotiation
- [ ] Network streaming codec selection

## References

- [FFmpeg Documentation](https://ffmpeg.org/documentation.html)
- [Opus Codec Specification](https://datatracker.ietf.org/doc/html/rfc6716)
- [FLAC Format Specification](https://xiph.org/flac/format.html)
- [Rust Audio Processing](https://github.com/RustAudio)

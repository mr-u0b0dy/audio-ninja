# Fuzzing Tests for Audio Ninja

This directory contains fuzz tests for the audio-ninja core library, targeting parsers and decoders that handle untrusted input.

## Setup

Fuzz testing uses `cargo-fuzz` (libFuzzer). To install and run:

```bash
cargo install cargo-fuzz

# Run all fuzz tests
cargo +nightly fuzz run --all

# Run specific fuzz target
cargo +nightly fuzz run fuzz_rtp_header

# Run with custom corpus
cargo +nightly fuzz run fuzz_rtp_header -- corpus/

# Limit execution time (in seconds)
cargo +nightly fuzz run fuzz_rtp_header -- -max_len=512 -max_total_time=10
```

## Fuzz Targets

### `fuzz_rtp_header`
Fuzzes the RTP header deserialization logic (`RtpHeader::deserialize`). Covers:
- Malformed packet lengths
- Invalid version numbers
- Bitfield overflow
- Boundary conditions

### `fuzz_sync_clock`
Fuzzes the clock synchronization deserialization logic. Covers:
- Invalid timestamp formats
- Epoch overflow
- Nanosecond boundary conditions

### `fuzz_network_packet`
Fuzzes network packet deserialization. Covers:
- Malformed network data
- Invalid header combinations
- Payload parsing edge cases

## Coverage

To generate coverage reports:

```bash
cargo +nightly fuzz cov fuzz_rtp_header
```

Coverage data is generated in the `coverage/` directory.

## Corpus

Fuzzing generates interesting test cases in the `corpus/` directory for each target:

- `fuzz-target/corpus/` - Contains seeds discovered during fuzzing
- `fuzz-target/artifacts/` - Contains crashes and panics found

## Continuous Fuzzing

For CI/CD, fuzz for a limited time:

```bash
timeout 300 cargo +nightly fuzz run fuzz_rtp_header -- -max_total_time=300 || true
```

This prevents fuzzing from running indefinitely in CI while still discovering bugs.

## References

- [cargo-fuzz Guide](https://rust-fuzz.github.io/book/cargo-fuzz.html)
- [libFuzzer Documentation](https://llvm.org/docs/LibFuzzer/)
- [Rust Fuzzing Book](https://rust-fuzz.github.io/book/)

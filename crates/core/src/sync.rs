// SPDX-License-Identifier: Apache-2.0

use crate::transport::{ClockSource, ClockTimestamp};
use std::time::Duration;

#[derive(Clone, Debug, PartialEq)]
pub struct ClockConfig {
    pub source: ClockSource,
    pub sync_interval: Duration,
    pub max_skew: Duration,
}

impl Default for ClockConfig {
    fn default() -> Self {
        Self {
            source: ClockSource::System,
            sync_interval: Duration::from_millis(100),
            max_skew: Duration::from_micros(100), // ±100 µs for tight sync
        }
    }
}

pub trait ClockSync {
    fn now(&self) -> ClockTimestamp;
    fn sync(&mut self, reference: &ClockTimestamp) -> anyhow::Result<()>;
    fn skew(&self) -> Duration;
}

pub struct PtpClock {
    config: ClockConfig,
    offset: Duration,
    last_sync: Option<ClockTimestamp>,
}

impl PtpClock {
    pub fn new(config: ClockConfig) -> Self {
        Self {
            config,
            offset: Duration::ZERO,
            last_sync: None,
        }
    }
}

impl Default for PtpClock {
    fn default() -> Self {
        Self::new(ClockConfig {
            source: ClockSource::Ptp,
            ..Default::default()
        })
    }
}

impl ClockSync for PtpClock {
    fn now(&self) -> ClockTimestamp {
        let mut ts = ClockTimestamp::now(ClockSource::Ptp);
        let adjusted = ts.to_duration() + self.offset;
        ts.seconds = adjusted.as_secs();
        ts.nanos = adjusted.subsec_nanos();
        ts
    }

    fn sync(&mut self, reference: &ClockTimestamp) -> anyhow::Result<()> {
        let local = ClockTimestamp::now(ClockSource::System);
        let ref_dur = reference.to_duration();
        let local_dur = local.to_duration();

        self.offset = if ref_dur > local_dur {
            ref_dur - local_dur
        } else {
            Duration::ZERO
        };

        self.last_sync = Some(reference.clone());
        Ok(())
    }

    fn skew(&self) -> Duration {
        if let Some(ref last) = self.last_sync {
            let now = ClockTimestamp::now(ClockSource::System);
            now.skew_from(last)
        } else {
            Duration::ZERO
        }
    }
}

pub struct NtpClock {
    config: ClockConfig,
    offset: Duration,
    last_sync: Option<ClockTimestamp>,
}

impl NtpClock {
    pub fn new(config: ClockConfig) -> Self {
        Self {
            config,
            offset: Duration::ZERO,
            last_sync: None,
        }
    }
}

impl Default for NtpClock {
    fn default() -> Self {
        Self::new(ClockConfig {
            source: ClockSource::Ntp,
            sync_interval: Duration::from_secs(1), // NTP syncs less frequently
            max_skew: Duration::from_millis(10),   // ±10 ms for NTP
            ..Default::default()
        })
    }
}

impl ClockSync for NtpClock {
    fn now(&self) -> ClockTimestamp {
        let mut ts = ClockTimestamp::now(ClockSource::Ntp);
        let adjusted = ts.to_duration() + self.offset;
        ts.seconds = adjusted.as_secs();
        ts.nanos = adjusted.subsec_nanos();
        ts
    }

    fn sync(&mut self, reference: &ClockTimestamp) -> anyhow::Result<()> {
        let local = ClockTimestamp::now(ClockSource::System);
        let ref_dur = reference.to_duration();
        let local_dur = local.to_duration();

        // NTP uses gradual adjustment rather than hard offset
        let diff = if ref_dur > local_dur {
            ref_dur - local_dur
        } else {
            local_dur - ref_dur
        };

        // Apply 10% of the difference each sync
        self.offset += diff / 10;

        self.last_sync = Some(reference.clone());
        Ok(())
    }

    fn skew(&self) -> Duration {
        if let Some(ref last) = self.last_sync {
            let now = ClockTimestamp::now(ClockSource::System);
            now.skew_from(last)
        } else {
            Duration::ZERO
        }
    }
}

pub struct SystemClock {
    config: ClockConfig,
}

impl SystemClock {
    pub fn new() -> Self {
        Self {
            config: ClockConfig::default(),
        }
    }
}

impl Default for SystemClock {
    fn default() -> Self {
        Self::new()
    }
}

impl ClockSync for SystemClock {
    fn now(&self) -> ClockTimestamp {
        ClockTimestamp::now(ClockSource::System)
    }

    fn sync(&mut self, _reference: &ClockTimestamp) -> anyhow::Result<()> {
        // System clock doesn't sync to external references
        Ok(())
    }

    fn skew(&self) -> Duration {
        Duration::ZERO
    }
}

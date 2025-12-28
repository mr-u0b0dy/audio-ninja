// SPDX-License-Identifier: Apache-2.0

use std::time::Duration;

#[derive(Clone, Debug, PartialEq)]
pub struct ControlMessage {
    pub device_id: String,
    pub payload: ControlPayload,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ControlPayload {
    Identify,
    SetLayout(String),
    SetTrimDb(f32),
    SetDelay(Duration),
    FirmwareVersion(String),
    Heartbeat,
}

pub trait ControlEndpoint {
    fn send(&mut self, msg: ControlMessage) -> anyhow::Result<()>;
    fn receive(&mut self) -> anyhow::Result<Option<ControlMessage>>;
}

pub struct LoopbackControl {
    queue: Vec<ControlMessage>,
}

impl LoopbackControl {
    pub fn new() -> Self {
        Self { queue: Vec::new() }
    }
}

impl Default for LoopbackControl {
    fn default() -> Self {
        Self::new()
    }
}

impl ControlEndpoint for LoopbackControl {
    fn send(&mut self, msg: ControlMessage) -> anyhow::Result<()> {
        self.queue.push(msg);
        Ok(())
    }

    fn receive(&mut self) -> anyhow::Result<Option<ControlMessage>> {
        if self.queue.is_empty() {
            return Ok(None);
        }
        Ok(Some(self.queue.remove(0)))
    }
}

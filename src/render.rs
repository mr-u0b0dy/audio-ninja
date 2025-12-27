// SPDX-License-Identifier: Apache-2.0

use crate::{AudioBlock, SpeakerLayout};
use std::time::Duration;

#[derive(Clone, Debug, PartialEq)]
pub struct RenderOptions {
    pub target_layout: SpeakerLayout,
    pub headroom_db: f32,
    pub max_latency: Duration,
}

pub trait Renderer {
    fn render(&mut self, input: AudioBlock, opts: &RenderOptions) -> AudioBlock;
}

pub struct ReferenceRenderer;

impl Renderer for ReferenceRenderer {
    fn render(&mut self, input: AudioBlock, _opts: &RenderOptions) -> AudioBlock {
        // Placeholder: future work will perform object panning, downmix/upmix, and gain staging.
        input
    }
}

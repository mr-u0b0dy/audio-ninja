// SPDX-License-Identifier: Apache-2.0

//! Application state for the TUI

use serde_json::Value;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Screen {
    Dashboard,
    Speakers,
    Layout,
    Transport,
    Input,
    Output,
    Calibration,
}

#[derive(Debug)]
pub struct App {
    pub running: bool,
    pub current_screen: Screen,
    pub base_url: String,
    pub status: Option<Value>,
    pub speakers: Option<Value>,
    pub layout: Option<Value>,
    pub transport_status: Option<Value>,
    pub playback_status: Option<Value>,
    pub input_devices: Option<Value>,
    pub input_status: Option<Value>,
    pub output_devices: Option<Value>,
    pub output_status: Option<Value>,
    pub calibration_status: Option<Value>,
    pub stats: Option<Value>,
    pub error_message: Option<String>,
    pub selected_index: usize,
}

impl App {
    pub fn new(base_url: String) -> Self {
        Self {
            running: true,
            current_screen: Screen::Dashboard,
            base_url,
            status: None,
            speakers: None,
            layout: None,
            transport_status: None,
            playback_status: None,
            input_devices: None,
            input_status: None,
            output_devices: None,
            output_status: None,
            calibration_status: None,
            stats: None,
            error_message: None,
            selected_index: 0,
        }
    }

    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn next_screen(&mut self) {
        self.current_screen = match self.current_screen {
            Screen::Dashboard => Screen::Speakers,
            Screen::Speakers => Screen::Layout,
            Screen::Layout => Screen::Transport,
            Screen::Transport => Screen::Input,
            Screen::Input => Screen::Output,
            Screen::Output => Screen::Calibration,
            Screen::Calibration => Screen::Dashboard,
        };
    }

    pub fn previous_screen(&mut self) {
        self.current_screen = match self.current_screen {
            Screen::Dashboard => Screen::Calibration,
            Screen::Speakers => Screen::Dashboard,
            Screen::Layout => Screen::Speakers,
            Screen::Transport => Screen::Layout,
            Screen::Input => Screen::Transport,
            Screen::Output => Screen::Input,
            Screen::Calibration => Screen::Output,
        };
    }

    pub fn next_item(&mut self) {
        self.selected_index = self.selected_index.saturating_add(1);
    }

    pub fn previous_item(&mut self) {
        self.selected_index = self.selected_index.saturating_sub(1);
    }

    pub fn clear_error(&mut self) {
        self.error_message = None;
    }

    pub fn set_error(&mut self, error: String) {
        self.error_message = Some(error);
    }
}

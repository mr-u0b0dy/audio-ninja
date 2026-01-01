// SPDX-License-Identifier: Apache-2.0

//! Input handling for the TUI

use crossterm::event::{self, Event, KeyCode};
use super::app::App;

pub async fn handle_input(app: &mut App) -> bool {
    if crossterm::event::poll(std::time::Duration::from_millis(250)).unwrap_or(false) {
        if let Ok(Event::Key(key)) = event::read() {
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => {
                    app.quit();
                    return true;
                }
                KeyCode::Right | KeyCode::Char('n') => {
                    app.next_screen();
                }
                KeyCode::Left | KeyCode::Char('p') => {
                    app.previous_screen();
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    app.next_item();
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    app.previous_item();
                }
                KeyCode::Char('r') => {
                    // Trigger refresh
                    return false;
                }
                KeyCode::Char('d') => {
                    // Discover speakers
                    return false;
                }
                KeyCode::Char('c') => {
                    // Start calibration
                    return false;
                }
                KeyCode::Char('a') => {
                    // Apply calibration
                    return false;
                }
                _ => {}
            }
        }
    }
    false
}

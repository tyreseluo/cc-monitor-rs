use std::time::Duration;
use crossterm::event::{self, Event};
use anyhow::Result;

/// Event handler for TUI
pub struct EventHandler;

impl EventHandler {
    pub fn new() -> Self {
        Self
    }

    /// Poll for events with a timeout
    pub fn poll(&self, timeout: Duration) -> Result<Option<Event>> {
        if event::poll(timeout)? {
            Ok(Some(event::read()?))
        } else {
            Ok(None)
        }
    }
}
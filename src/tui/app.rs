use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use crate::monitor::MonitorData;
use crate::ui_updates::{dequeue_monitor_updates, MonitorUpdate};
use crate::i18n::{self, Language};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Panel {
    Network,
    Usage,
    History,
}

pub struct TuiApp {
    pub monitor_data: Arc<Mutex<MonitorData>>,
    pub selected_panel: Panel,
    pub selected_index: usize,
    pub current_language: Language,
    pub status_messages: VecDeque<String>,
}

impl TuiApp {
    pub fn new() -> Self {
        Self {
            monitor_data: Arc::new(Mutex::new(MonitorData::default())),
            selected_panel: Panel::Network,
            selected_index: 0,
            current_language: i18n::get_language(),
            status_messages: VecDeque::with_capacity(5),
        }
    }

    pub fn update_monitor_data(&mut self) {
        // Get all pending updates and process them
        let updates = dequeue_monitor_updates();
        
        for update in updates {
            match update {
                MonitorUpdate::DataUpdate(data) => {
                    *self.monitor_data.lock().unwrap() = data;
                }
                MonitorUpdate::StatusMessage(msg) => {
                    self.add_status_message(msg);
                }
                MonitorUpdate::ErrorMessage(msg) => {
                    self.add_status_message(format!("Error: {}", msg));
                }
                MonitorUpdate::LanguageChanged => {
                    // Language is already updated globally
                }
            }
        }
    }
    
    fn add_status_message(&mut self, msg: String) {
        if self.status_messages.len() >= 5 {
            self.status_messages.pop_front();
        }
        self.status_messages.push_back(msg);
    }

    pub fn next_panel(&mut self) {
        self.selected_panel = match self.selected_panel {
            Panel::Network => Panel::Usage,
            Panel::Usage => Panel::History,
            Panel::History => Panel::Network,
        };
        self.selected_index = 0;
    }

    pub fn on_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    pub fn on_down(&mut self) {
        // The max index depends on the current panel
        let max_index = match self.selected_panel {
            Panel::Network => 3, // Status, Latency, Speed
            Panel::Usage => 6,   // Session, Time, Remaining, Tokens, Cost, Model, Status
            Panel::History => 10, // Up to 10 days of history
        };
        
        if self.selected_index < max_index {
            self.selected_index += 1;
        }
    }

    pub fn cycle_language(&mut self) {
        self.current_language = match self.current_language {
            Language::English => Language::Chinese,
            Language::Chinese => Language::Japanese,
            Language::Japanese => Language::English,
        };
        i18n::set_language(self.current_language);
    }
}
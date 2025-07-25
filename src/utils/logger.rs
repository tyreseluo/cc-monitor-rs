use std::sync::Mutex;
use std::collections::VecDeque;
use once_cell::sync::Lazy;

pub static LOGGER: Lazy<Mutex<Logger>> = Lazy::new(|| {
    Mutex::new(Logger::new())
});

pub struct Logger {
    messages: VecDeque<String>,
    max_messages: usize,
    enabled: bool,
}

impl Logger {
    fn new() -> Self {
        Self {
            messages: VecDeque::new(),
            max_messages: 100,
            enabled: false,
        }
    }
    
    pub fn enable(&mut self) {
        self.enabled = true;
    }
    
    pub fn disable(&mut self) {
        self.enabled = false;
    }
    
    pub fn log(&mut self, message: String) {
        if self.messages.len() >= self.max_messages {
            self.messages.pop_front();
        }
        self.messages.push_back(message);
    }
    
    pub fn get_messages(&self) -> Vec<String> {
        self.messages.iter().cloned().collect()
    }
    
    pub fn clear(&mut self) {
        self.messages.clear();
    }
}

// Macros to replace println! and eprintln! in TUI mode
#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        {
            let message = format!($($arg)*);
            if let Ok(mut logger) = $crate::utils::logger::LOGGER.lock() {
                logger.log(format!("[INFO] {}", message));
            }
        }
    };
}

#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        {
            let message = format!($($arg)*);
            if let Ok(mut logger) = $crate::utils::logger::LOGGER.lock() {
                logger.log(format!("[ERROR] {}", message));
            }
        }
    };
}

#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {
        {
            let message = format!($($arg)*);
            if let Ok(mut logger) = $crate::utils::logger::LOGGER.lock() {
                logger.log(format!("[DEBUG] {}", message));
            }
        }
    };
}
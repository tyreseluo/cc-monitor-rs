use notify_rust::{Notification, Urgency};

pub fn show_notification(title: &str, message: &str) {
    #[cfg(target_os = "macos")]
    {
        // Use macOS-specific notification with sound
        if let Err(e) = Notification::new()
            .summary(title)
            .body(message)
            .sound_name("Glass")
            .show()
        {
            eprintln!("Failed to show notification: {}", e);
        }
    }
    
    #[cfg(not(target_os = "macos"))]
    {
        // Generic notification for other platforms
        if let Err(e) = Notification::new()
            .summary(title)
            .body(message)
            .show()
        {
            eprintln!("Failed to show notification: {}", e);
        }
    }
}

pub fn show_critical_notification(title: &str, message: &str) {
    #[cfg(target_os = "macos")]
    {
        // Use macOS-specific notification with alert sound
        if let Err(e) = Notification::new()
            .summary(title)
            .body(message)
            .sound_name("Basso")  // Alert sound
            .show()
        {
            eprintln!("Failed to show notification: {}", e);
        }
    }
    
    #[cfg(not(target_os = "macos"))]
    {
        // Generic notification for other platforms
        if let Err(e) = Notification::new()
            .summary(title)
            .body(message)
            .urgency(Urgency::Critical)
            .show()
        {
            eprintln!("Failed to show notification: {}", e);
        }
    }
}

pub fn show_usage_notification(tokens: i64, cost: f64, remaining_time: &str) {
    let message = format!(
        "Token 使用量: {}\n花费: ${:.2}\n剩余时间: {}",
        format_number(tokens),
        cost,
        remaining_time
    );
    
    show_notification("Claude Code 使用状态", &message);
}

fn format_number(num: i64) -> String {
    if num >= 1_000_000 {
        format!("{:.1}M", num as f64 / 1_000_000.0)
    } else if num >= 1_000 {
        format!("{:.1}K", num as f64 / 1_000.0)
    } else {
        num.to_string()
    }
}
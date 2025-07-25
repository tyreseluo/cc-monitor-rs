use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Sparkline},
    Frame,
};

use crate::i18n;
use crate::monitor::SpeedLevel;
use crate::utils::model_pricing;
use super::app::{Panel, TuiApp};

pub fn draw(f: &mut Frame, app: &mut TuiApp) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(10),  // Network panel
            Constraint::Length(12),  // Usage panel
            Constraint::Min(10),     // History panel
            Constraint::Length(3),   // Help bar
        ])
        .split(f.size());

    draw_network_panel(f, app, chunks[0]);
    draw_usage_panel(f, app, chunks[1]);
    draw_history_panel(f, app, chunks[2]);
    draw_help_bar(f, chunks[3]);
}

fn draw_network_panel(f: &mut Frame, app: &TuiApp, area: Rect) {
    let monitor_data = app.monitor_data.lock().unwrap();
    
    let title = format!(" {} ", i18n::get(i18n::keys::NETWORK_TITLE));
    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .style(if app.selected_panel == Panel::Network {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        });

    let status_text = if monitor_data.network_connected {
        i18n::get(i18n::keys::NETWORK_CONNECTED)
    } else {
        i18n::get(i18n::keys::NETWORK_DISCONNECTED)
    };
    
    let status_color = if monitor_data.network_connected {
        Color::Green
    } else {
        Color::Red
    };

    let latency_text = monitor_data.latency.as_deref().unwrap_or("--");
    
    let speed_text = match &monitor_data.connection_speed {
        Some(SpeedLevel::Excellent) => i18n::get(i18n::keys::NETWORK_EXCELLENT),
        Some(SpeedLevel::Good) => i18n::get(i18n::keys::NETWORK_GOOD),
        Some(SpeedLevel::Fair) => i18n::get(i18n::keys::NETWORK_FAIR),
        Some(SpeedLevel::Slow) => i18n::get(i18n::keys::NETWORK_SLOW),
        None => i18n::get(i18n::keys::NETWORK_DETECTING),
    };
    
    let speed_color = match &monitor_data.connection_speed {
        Some(SpeedLevel::Excellent) => Color::Green,
        Some(SpeedLevel::Good) => Color::LightGreen,
        Some(SpeedLevel::Fair) => Color::Yellow,
        Some(SpeedLevel::Slow) => Color::Red,
        None => Color::Gray,
    };

    let items = vec![
        ListItem::new(vec![
            Line::from(vec![
                Span::raw(format!("{}: ", i18n::get(i18n::keys::NETWORK_STATUS))),
                Span::styled(status_text, Style::default().fg(status_color)),
            ])
        ]),
        ListItem::new(vec![
            Line::from(vec![
                Span::raw(format!("{}: ", i18n::get(i18n::keys::NETWORK_LATENCY))),
                Span::raw(latency_text),
            ])
        ]),
        ListItem::new(vec![
            Line::from(vec![
                Span::raw(format!("{}: ", i18n::get(i18n::keys::NETWORK_SPEED))),
                Span::styled(speed_text, Style::default().fg(speed_color)),
            ])
        ]),
    ];

    let list = List::new(items)
        .block(block)
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol("> ");

    f.render_widget(list, area);
}

fn draw_usage_panel(f: &mut Frame, app: &TuiApp, area: Rect) {
    let monitor_data = app.monitor_data.lock().unwrap();
    let ccusage_data = &monitor_data.ccusage_data;
    
    let title = format!(" {} ", i18n::get(i18n::keys::USAGE_TITLE));
    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .style(if app.selected_panel == Panel::Usage {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        });

    let status_color = match ccusage_data.status.as_str() {
        "ACTIVE" => Color::Green,
        "COMPLETED" => Color::Blue,
        "RUNNING" => Color::Yellow,
        _ => Color::Gray,
    };

    let status_text = match ccusage_data.status.as_str() {
        "ACTIVE" => i18n::get(i18n::keys::USAGE_ACTIVE),
        "COMPLETED" => i18n::get(i18n::keys::USAGE_COMPLETED),
        "RUNNING" => i18n::get(i18n::keys::USAGE_RUNNING),
        _ => i18n::get(i18n::keys::USAGE_INACTIVE),
    };

    // Calculate model prices
    let model_prices = if ccusage_data.tokens_num > 0 {
        let input_tokens = (ccusage_data.tokens_num as f64 * 0.3) as u64;
        let output_tokens = (ccusage_data.tokens_num as f64 * 0.7) as u64;
        let prices = model_pricing::calculate_all_model_costs(input_tokens, output_tokens);
        
        // Format top 3 prices
        prices.iter()
            .take(3)
            .map(|(_, name, cost)| format!("{}: ${:.2}", name, cost))
            .collect::<Vec<_>>()
            .join(" | ")
    } else {
        String::new()
    };

    let items = vec![
        ListItem::new(format!("{}: {}", i18n::get(i18n::keys::USAGE_SESSION_START), ccusage_data.latest_session)),
        ListItem::new(format!("{}: {} → {} ({})", 
            i18n::get(i18n::keys::USAGE_TIME),
            ccusage_data.session_start,
            ccusage_data.session_end,
            i18n::get(i18n::keys::USAGE_RESET)
        )),
        ListItem::new(format!("{}: {}", i18n::get(i18n::keys::USAGE_REMAINING), ccusage_data.remaining_time)),
        ListItem::new(format!("🎫 Tokens: {}", ccusage_data.tokens)),
        ListItem::new(format!("{}: {}", i18n::get(i18n::keys::USAGE_COST), ccusage_data.cost)),
        ListItem::new(format!("{}: {}", i18n::get(i18n::keys::USAGE_MODEL), ccusage_data.model)),
        ListItem::new(vec![
            Line::from(vec![
                Span::raw(format!("{}: ", i18n::get(i18n::keys::USAGE_STATUS))),
                Span::styled(status_text, Style::default().fg(status_color)),
            ])
        ]),
        ListItem::new(model_prices),
    ];

    let list = List::new(items)
        .block(block)
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol("> ");

    f.render_widget(list, area);
}

fn draw_history_panel(f: &mut Frame, app: &TuiApp, area: Rect) {
    let monitor_data = app.monitor_data.lock().unwrap();
    
    let title = format!(" {} ", i18n::get(i18n::keys::HISTORY_TITLE));
    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .style(if app.selected_panel == Panel::History {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        });

    // Create a sparkline chart from daily costs
    let mut daily_values: Vec<(String, f64)> = monitor_data.daily_costs
        .iter()
        .map(|(date, cost)| (date.clone(), cost.cost))
        .collect();
    daily_values.sort_by(|a, b| a.0.cmp(&b.0));
    
    let values: Vec<u64> = daily_values
        .iter()
        .map(|(_, cost)| (*cost * 100.0) as u64)
        .collect();

    let total_cost: f64 = daily_values.iter().map(|(_, cost)| cost).sum();
    let avg_cost = if !daily_values.is_empty() {
        total_cost / daily_values.len() as f64
    } else {
        0.0
    };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(5),
        ])
        .split(area);

    // Summary
    let summary = Paragraph::new(vec![
        Line::from(format!("{}: ${:.2} | {}: ${:.2}", 
            i18n::get(i18n::keys::HISTORY_TOTAL), total_cost,
            i18n::get(i18n::keys::HISTORY_AVERAGE), avg_cost
        )),
    ])
    .block(block.clone());
    f.render_widget(summary, chunks[0]);

    // Chart
    if !values.is_empty() {
        let sparkline = Sparkline::default()
            .block(Block::default())
            .data(&values)
            .style(Style::default().fg(Color::Green));
        f.render_widget(sparkline, chunks[1]);
    }
}

fn draw_help_bar(f: &mut Frame, area: Rect) {
    let help_text = vec![
        Span::raw(" ["),
        Span::styled("q", Style::default().fg(Color::Yellow)),
        Span::raw("] Quit | ["),
        Span::styled("r", Style::default().fg(Color::Yellow)),
        Span::raw("] Refresh | ["),
        Span::styled("Tab", Style::default().fg(Color::Yellow)),
        Span::raw("] Switch Panel | ["),
        Span::styled("l", Style::default().fg(Color::Yellow)),
        Span::raw("] Language | ["),
        Span::styled("↑↓", Style::default().fg(Color::Yellow)),
        Span::raw("] Navigate"),
    ];

    let help = Paragraph::new(Line::from(help_text))
        .block(Block::default().borders(Borders::ALL))
        .alignment(Alignment::Center);

    f.render_widget(help, area);
}
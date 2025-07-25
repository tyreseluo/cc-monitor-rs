pub mod app;
pub mod ui;
pub mod events;

use std::io;
use std::time::Duration;
use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

use crate::background::{start_monitor_tokio, submit_monitor_request, MonitorRequest};

pub use self::app::TuiApp;

/// Run the TUI application
pub fn run_tui() -> Result<()> {
    // Initialize panic handler to restore terminal on panic
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic| {
        let _ = disable_raw_mode();
        let _ = execute!(
            io::stdout(),
            LeaveAlternateScreen,
            DisableMouseCapture
        );
        original_hook(panic);
    }));
    
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    // Disable logging for TUI mode
    crate::background::disable_logging();
    
    // Start the background monitoring
    start_monitor_tokio()?;

    // Create app and run it
    let app = TuiApp::new();
    let res = run_app(&mut terminal, app);

    // Stop monitoring when exiting
    submit_monitor_request(MonitorRequest::StopMonitoring)?;
    
    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(_err) = res {
        // Log error internally instead of printing
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: TuiApp) -> Result<()> {
    let mut last_update = std::time::Instant::now();
    
    loop {
        // Update monitor data periodically (every second)
        if last_update.elapsed() >= Duration::from_secs(1) {
            app.update_monitor_data();
            last_update = std::time::Instant::now();
        }
        
        // Draw the UI
        terminal.draw(|f| ui::draw(f, &mut app))?;

        // Handle events with shorter timeout to reduce flickering
        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                // Handle Ctrl+C as quit
                if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
                    return Ok(());
                }
                
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                    KeyCode::Char('r') => {
                        submit_monitor_request(MonitorRequest::ForceUpdate)?;
                        app.update_monitor_data();
                    }
                    KeyCode::Char('l') => {
                        app.cycle_language();
                    }
                    KeyCode::Tab => {
                        app.next_panel();
                    }
                    KeyCode::Up => {
                        app.on_up();
                    }
                    KeyCode::Down => {
                        app.on_down();
                    }
                    _ => {}
                }
            }
        }
    }
}
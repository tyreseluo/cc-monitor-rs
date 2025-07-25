use clap::Parser;
use anyhow::Result;

#[derive(Parser, Debug)]
#[command(author, version, about = "Real-time Claude Code usage monitor", long_about = None)]
struct Args {
    /// Run in TUI (Terminal UI) mode
    #[arg(long, conflicts_with = "gui")]
    tui: bool,

    /// Run in GUI mode (default)
    #[arg(long)]
    gui: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    if args.tui {
        // Run TUI mode
        cc_monitor_rs::tui::run_tui()
    } else {
        // Run GUI mode (default)
        cc_monitor_rs::app::app_main();
        Ok(())
    }
}
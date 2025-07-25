# Changelog

All notable changes to Claude Code Monitor will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.0] - 2025-07-25

### 🎉 Major Update: Terminal UI Mode

This release introduces a fully-featured Terminal UI (TUI) mode, making Claude Code Monitor accessible in terminal environments, SSH sessions, and headless servers.

### Added

#### 🖥️ Terminal UI (TUI) Mode
- **Complete Terminal Interface**:
  - Full-featured monitoring in terminal environments
  - Perfect for SSH sessions and headless servers
  - Lightweight alternative to GUI mode
- **Interactive Controls**:
  - `q` or `Esc` - Quit application
  - `r` - Force refresh data
  - `l` - Cycle through languages
  - `Tab` - Switch between panels
  - `↑/↓` - Navigate within panels
  - `Ctrl+C` - Emergency exit
- **Three Main Panels**:
  - Network status with connection indicators
  - Claude Code usage with real-time updates
  - Historical data with ASCII sparkline charts
- **Responsive Layout**:
  - Adapts to terminal size
  - Color-coded status indicators
  - Clean, distraction-free interface


#### 🚀 Command Line Interface
- **New CLI Arguments**:
  - `--tui` - Launch in Terminal UI mode
  - `--gui` - Launch in GUI mode (default)
  - `--help` - Show help information
  - `--version` - Display version
- **Flexible Usage**:
  ```bash
  cc-monitor-rs --tui  # Terminal mode
  cc-monitor-rs        # GUI mode (default)
  ```

### Improved

#### 🔧 Stability Enhancements
- **TUI Stability**:
  - Prevented background output from corrupting interface
  - Improved refresh mechanism to reduce flickering
  - Better error handling and recovery
- **Resource Management**:
  - Proper cleanup on exit
  - Graceful shutdown of background tasks
  - Improved memory usage

#### 📊 Better Data Visualization
- **TUI Charts**:
  - ASCII/Unicode sparkline for historical data
  - Color-coded status indicators
  - Compact model pricing display
- **Enhanced Readability**:
  - Better number formatting
  - Clearer status messages
  - Improved layout spacing

### Fixed
- Background process output interfering with TUI display
- TUI interface flickering during updates
- Various compilation warnings cleaned up
- Logo packaging issues - now embedded in binary (reduced from 980KB to 14KB)

### Technical Details
- **New Dependencies**:
  - `ratatui` - Terminal UI framework
  - `crossterm` - Cross-platform terminal control
  - `clap` - Command-line argument parsing
- **Architecture Improvements**:
  - Modular TUI design for easy maintenance
  - Shared monitoring backend between GUI and TUI
  - Better logging control for TUI mode
  - Icon embedded in binary for easier packaging

### Migration Notes
- Existing users can continue using GUI mode without changes
- TUI mode uses the same configuration and preferences
- Language preference is shared between GUI and TUI modes

## [0.2.0] - 2025-07-23

### 🌍 Internationalization Update

This release introduces comprehensive internationalization (i18n) support to Claude Code Monitor.

### Added

#### Internationalization (i18n)
- **Multi-language Support**:
  - English (en)
  - Chinese Simplified (中文)
  - Japanese (日本語)
- **Automatic Language Detection**:
  - Detects system language from environment variables
  - Falls back to English if unsupported
- **Complete Localization**:
  - All UI elements translated
  - System tray menu fully localized
  - Desktop notifications in selected language
- **Real-time Language Switching**:
  - Change language without restart
  - Settings persist across sessions
  - Language selector in main UI

#### Model Pricing Comparison
- **Dynamic Price Calculation**:
  - Shows costs for different AI models based on current token usage
  - Supports 11+ popular models
- **Configurable Pricing**:
  - Default prices included in `resources/model_pricing.json`
  - User-customizable configuration
  - Prices shown in usage panel

### Improved
- Better code organization with i18n module structure
- Enhanced user preferences system
- More flexible UI updates

### Fixed
- Window title not updating with language change (Makepad limitation)
- Various compilation warnings
- Minor UI layout issues

## [0.1.0] - 2025-07-22

### 🎉 Initial Release

This is the first public release of Claude Code Monitor, a complete rewrite from Python to Rust using the Makepad UI framework.

### Added

#### Core Features
- **Real-time Monitoring**: Updates every second for immediate feedback
- **Network Status Detection**: 
  - Ping latency measurement to Google DNS (8.8.8.8)
  - TCP connection speed testing
  - Automatic proxy detection and support
  - Visual indicators with color-coded status

#### Claude Code Integration
- **Usage Tracking**:
  - Live token consumption monitoring
  - Active/Completed session detection
  - 5-hour session countdown timer
  - Model identification (opus-4, sonnet-4, haiku-3)
  - Real-time cost calculations
- **Smart Update Strategy**:
  - Fast cached mode for regular updates
  - Periodic calculate mode for accuracy
  - Automatic ccusage recovery after failures

#### Historical Analytics
- **7-Day Dashboard**:
  - Visual bar chart with cost-based coloring
  - Daily cost breakdown
  - Cumulative total and averages
  - Session count tracking
- **Data Sources**:
  - Primary: `ccusage daily` command
  - Fallback: Aggregated `ccusage blocks` data

#### System Integration
- **System Tray**:
  - Custom "C" icon design
  - Dropdown menu with full status
  - Model, remaining time, and active status display
  - Dynamic tooltip updates
- **Desktop Notifications**:
  - Network status change alerts
  - Hourly usage reports
  - Native macOS notification support
  - Custom sounds for different alert types

#### User Interface
- **Makepad Native UI**:
  - Dark theme design (#1a1a1a background)
  - Responsive layout with proper spacing
  - Custom widgets for each section
  - Real-time timestamp in footer
- **Localization**:
  - Chinese interface text
  - Formatted numbers and times
  - Emoji indicators for clarity

### Technical Implementation
- **Architecture**:
  - Rust for safety and performance
  - Makepad for native UI rendering
  - Tokio for async background tasks
  - Arc/Mutex for thread-safe state management
- **Background Processing**:
  - Non-blocking UI updates
  - Parallel network and usage checks
  - Queue-based UI update system
  - Efficient resource utilization

### Platform Support
- macOS (primary platform)
- Linux (experimental)
- Windows (planned)

### Known Limitations
- Requires authenticated Claude Code CLI
- Depends on Node.js for ccusage commands
- System tray quit menu simplified for v0.1.0

### Migration from Python Version
- Significantly improved performance
- Native UI instead of terminal display
- Better error handling and recovery
- More accurate real-time updates
- Enhanced visual feedback

---

[0.1.0]: https://github.com/zhanghandong/cc-monitor-rs/releases/tag/v0.1.0
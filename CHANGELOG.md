# Changelog

All notable changes to Claude Code Monitor will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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